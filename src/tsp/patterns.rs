use crate::tsp::pattern::{Idx, IdxValue, PQueue, Pattern, PatternResult, WithIndex};
use std::cmp::min;
use std::marker::PhantomData;

#[derive(Debug, Default)]
pub struct NoState;

pub struct ConstantPattern<E, T: Clone> {
    value: PatternResult<T>,
    phantom: PhantomData<E>,
}

impl<E, T: Clone> ConstantPattern<E, T> {
    pub fn new(value: PatternResult<T>) -> Self {
        ConstantPattern {
            value,
            phantom: PhantomData,
        }
    }
}

fn zip_with<T, U, F, R>(this: Option<T>, other: Option<U>, f: F) -> Option<R>
where
    F: FnOnce(T, U) -> R,
{
    Some(f(this?, other?))
}

impl<E: WithIndex, T: Clone> Pattern for ConstantPattern<E, T> {
    type State = NoState;
    type Event = E;
    type T = T;

    fn apply(
        &self,
        event: &Vec<Self::Event>,
        queue: &mut PQueue<Self::T>,
        _state: &mut Self::State,
    ) {
        queue.enqueue(
            zip_with(event.first(), event.last(), |first, last| {
                IdxValue::new(first.index(), last.index(), self.value.clone())
            })
            .into_iter(),
        );
    }

    type W = Idx;

    fn width(&self) -> Self::W {
        1u64
    }
}

pub struct FunctionPattern<E, F, T>
where
    F: Fn(&E) -> T,
{
    func: F,
    phantom: PhantomData<E>,
}

impl<E, F, T> FunctionPattern<E, F, T>
where
    F: Fn(&E) -> T,
{
    pub fn new(func: F) -> Self {
        FunctionPattern {
            func,
            phantom: PhantomData,
        }
    }
}

//todo should we relax requirements and remove PartialEq out of here?
impl<E: WithIndex, F, T: Clone + PartialEq> Pattern for FunctionPattern<E, F, T>
where
    F: Fn(&E) -> T,
{
    type State = NoState;
    type Event = E;
    type T = T;
    fn apply(
        &self,
        event: &Vec<Self::Event>,
        queue: &mut PQueue<Self::T>,
        _state: &mut Self::State,
    ) {
        event
            .iter()
            .map(|e| IdxValue::new(e.index(), e.index(), PatternResult::Success((self.func)(e))))
            .for_each(|x| {
                queue.enqueue_joined(x);
            })
    }

    type W = Idx;

    fn width(&self) -> Self::W {
        1
    }
}

pub struct BiPattern<P1, P2, F> {
    left: P1,
    right: P2,
    func: F,
}

impl<P1, T1, P2, T2, F, T3> BiPattern<P1, P2, F>
where
    P1: Pattern<T = T1>,
    P2: Pattern<T = T2>,
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
    P1: Pattern<Event = E, State = S1, T = T1>,
    P2: Pattern<Event = E, State = S2, T = T2>,
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

            if l.start < r.start {
                state.left_queue.rewind_to(r.start);
                continue;
            } else if l.start > r.start {
                state.right_queue.rewind_to(l.start);
                continue;
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
        1
    }
}
