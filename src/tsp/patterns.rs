use crate::tsp::pattern::{Idx, IdxValue, PQueue, Pattern, PatternResult, WithIndex};
use std::cmp::{max, min};
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
        0u64
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
        0
    }
}

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

pub struct AssertPattern<P> {
    inner: P,
}

impl<P> AssertPattern<P>
    where
        P: Pattern<T=bool>,
{
    pub fn new(inner: P) -> Self {
        AssertPattern { inner }
    }
}

#[derive(Default)]
pub struct AssertPatternState<S: Default> {
    inner_state: S,
    inner_queue: PQueue<bool>,
}

impl<E, P, S> Pattern for AssertPattern<P>
    where
        S: Default,
        E: WithIndex,
        P: Pattern<State=S, T=bool, Event=E>,
{
    type State = AssertPatternState<S>;
    type Event = E;
    type T = ();

    fn apply(
        &self,
        event: &Vec<Self::Event>,
        queue: &mut PQueue<Self::T>,
        state: &mut Self::State,
    ) {
        self.inner
            .apply(event, &mut state.inner_queue, &mut state.inner_state);
        while let Some(IdxValue { start, end, result }) = state.inner_queue.dequeue_option() {
            queue.enqueue_joined(IdxValue::new(
                start,
                end,
                match result {
                    PatternResult::Failure | PatternResult::Success(false) => {
                        PatternResult::Failure
                    }
                    PatternResult::Success(true) => PatternResult::Success(()),
                },
            ));
        }
    }

    type W = P::W;

    fn width(&self) -> Self::W {
        self.inner.width()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Window {
    size: u32,
}

pub struct WindowPattern<P> {
    inner: P,
    window: Window,
}

impl<P> WindowPattern<P>
    where
        P: Pattern<T=()>,
{
    pub fn new(inner: P, size: u32) -> Self {
        assert!(size > 0);
        WindowPattern {
            inner,
            window: Window { size },
        }
    }
}

#[derive(Default)]
pub struct WindowPatternState<S: Default> {
    inner_state: S,
    inner_queue: PQueue<()>,
    last_end: Idx,
}

impl<E, P, InnerState> Pattern for WindowPattern<P>
    where
        E: WithIndex,
        InnerState: Default,
        P: Pattern<Event=E, T=(), State=InnerState, W=Idx>,
{
    type State = WindowPatternState<InnerState>;
    type Event = E;
    type T = ();

    fn apply(&self, event: &Vec<Self::Event>, queue: &mut PQueue<()>, state: &mut Self::State) {
        // apply inner pattern to the input events
        self.inner.apply(event, &mut state.inner_queue, &mut state.inner_state);

        while let Some(IdxValue {
                           start:_,
                           end,
                           result: x,
                       }) = state.inner_queue.dequeue_option()
        {
            assert!(state.last_end < end);
            match x {
                PatternResult::Failure => {
                    queue.enqueue_joined(IdxValue::new(
                        state.last_end + 1,
                        end,
                        PatternResult::Failure,
                    ));
                    state.last_end = end;
                }
                PatternResult::Success(()) => {
                    let new_start = state.last_end + self.window.size as u64;
                    if new_start <= end {
                        queue.enqueue_joined(IdxValue::new(
                            new_start,
                            end,
                            PatternResult::Success(()),
                        ));
                        state.last_end = end;
                    }
                }
            }
        }
    }

    type W = Idx;

    fn width(&self) -> Self::W {
        (self.window.size - 1) as u64 + self.inner.width()
    }
}
