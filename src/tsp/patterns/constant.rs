use std::marker::PhantomData;

use crate::tsp::patterns::common::NoState;
use crate::tsp::patterns::pattern::{Idx, IdxValue, PQueue, Pattern, PatternResult};

#[derive(Clone)]
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

impl<E, T: Clone> Pattern for ConstantPattern<E, T> {
    type State = NoState;
    type Event = E;
    type T = T;

    fn apply(
        &self,
        start_idx: Idx,
        event: &[Self::Event],
        queue: &mut PQueue<Self::T>,
        _state: &mut Self::State,
    ) {
        if !event.is_empty() {
            queue.enqueue_one(IdxValue::new(
                start_idx,
                start_idx + event.len() as Idx - 1,
                self.value.clone(),
            ));
        }
    }

    type W = Idx;

    fn width(&self) -> Self::W {
        0u64
    }
}
