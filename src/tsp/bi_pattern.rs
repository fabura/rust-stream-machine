use std::cmp::{max, min};
use std::marker::PhantomData;

use crate::tsp::pattern::{Idx, IdxValue, Pattern, PatternResult, PQueue, WithIndex};

#[derive(Clone)]
pub struct BiPattern<P1, P2, F> {
    left: P1,
    right: P2,
    func: F,
}

impl<P1, T1, P2, T2, F, T3> BiPattern<P1, P2, F>
    where
        P1: Pattern<T=T1>,
        P2: Pattern<T=T2>,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        F: Fn(&T1, &T2) -> T3,
{
    pub fn new(left: P1, right: P2, func: F) -> Self {
        BiPattern { left, right, func }
    }
    fn apply_func(&self, l: &PatternResult<T1>, r: &PatternResult<T2>) -> PatternResult<T3> {
        match (l, r) {
            (PatternResult::Success(lt), PatternResult::Success(rt)) => {
                PatternResult::Success((self.func)(lt, rt))
            }
            _ => PatternResult::Failure,
        }
    }
}

#[derive(Debug)]
pub struct BiPatternState<S1: Default, T1: Clone, S2: Default, T2: Clone> {
    left: S1,
    right: S2,
    left_queue: PQueue<T1>,
    right_queue: PQueue<T2>,
}

impl<S1: Default, T1: Clone, S2: Default, T2: Clone> Default for BiPatternState<S1, T1, S2, T2> {
    fn default() -> Self {
        BiPatternState::new(S1::default(), S2::default())
    }
}

impl<S1: Default, T1: Clone, S2: Default, T2: Clone> BiPatternState<S1, T1, S2, T2> {
    pub fn new(left: S1, right: S2) -> Self {
        BiPatternState {
            left,
            right,
            left_queue: PQueue::default(),
            right_queue: PQueue::default(),
        }
    }
}

impl<E, P1, S1, T1, P2, S2, T2, F, T3> Pattern for BiPattern<P1, P2, F>
    where
        E: WithIndex,
        P1: Pattern<Event=E, State=S1, T=T1, W=Idx>,
        P2: Pattern<Event=E, State=S2, T=T2, W=Idx>,
        T1: Clone,
        T2: Clone,
        T3: Clone,
        S1: Default,
        S2: Default,
        F: Fn(&T1, &T2) -> T3,
        T3: PartialEq,
{
    type State = BiPatternState<S1, T1, S2, T2>;
    type Event = E;
    type T = T3;

    fn apply(&self, event: &Vec<E>, queue: &mut PQueue<T3>, state: &mut Self::State) {
        // todo add async here!
        self.left
            .apply(event, &mut state.left_queue, &mut state.left);
        self.right
            .apply(event, &mut state.right_queue, &mut state.right);

        loop {
            let (l, r) = match (
                state.left_queue.head_option(),
                state.right_queue.head_option(),
            ) {
                (Some(l), Some(r)) => (l, r),
                _ => return,
            };
            use std::cmp::Ordering;
            match l.start.cmp(&r.start) {
                Ordering::Less => {
                    state.left_queue.rewind_to(r.start);
                    continue;
                }
                Ordering::Greater => {
                    state.right_queue.rewind_to(l.start);
                    continue;
                }
                Ordering::Equal => {}
            }

            //at this moment both l and r have same start
            let end = min(l.end, r.end);
            queue.enqueue_joined(IdxValue::new(
                l.start,
                end,
                self.apply_func(&l.result, &r.result),
            ));

            if l.end == r.end {
                state.left_queue.behead();
                state.right_queue.behead();
            } else {
                state.left_queue.rewind_to(end + 1);
                state.right_queue.rewind_to(end + 1);
            }
        }
    }

    type W = Idx;

    fn width(&self) -> Self::W {
        max(self.left.width(), self.right.width())
    }
}
