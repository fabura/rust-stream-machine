use std::cmp::{max, min};
use std::marker::PhantomData;

use crate::tsp::patterns::pattern::{Idx, IdxValue, Pattern, PatternResult, PQueue, WithIndex};
use crate::tsp::patterns::common::NoState;

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
