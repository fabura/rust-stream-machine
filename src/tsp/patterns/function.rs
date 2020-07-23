use std::marker::PhantomData;

use crate::tsp::patterns::common::NoState;
use crate::tsp::patterns::pattern::{Idx, IdxValue, PQueue, Pattern, PatternResult};

#[derive(Clone)]
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
impl<E, F, T: Clone + PartialEq> Pattern for FunctionPattern<E, F, T>
where
    F: Fn(&E) -> T,
{
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
        event
            .iter()
            .enumerate()
            .map(|(idx, e)| {
                IdxValue::new(
                    start_idx + idx as Idx,
                    start_idx + idx as Idx,
                    PatternResult::Success((self.func)(e)),
                )
            })
            .for_each(|x| {
                queue.enqueue_joined(x);
            })
    }

    type W = Idx;

    fn width(&self) -> Self::W {
        0
    }
}
