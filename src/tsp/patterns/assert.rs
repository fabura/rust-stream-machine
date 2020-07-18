use std::cmp::{max, min};
use std::marker::PhantomData;

use crate::tsp::patterns::pattern::{Idx, IdxValue, Pattern, PatternResult, PQueue, WithIndex};

#[derive(Clone)]
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
