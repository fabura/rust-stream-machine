use std::marker::PhantomData;

use crate::tsp::patterns::common::NoState;
use crate::tsp::patterns::pattern::{Idx, IdxValue, Pattern, PatternResult, PQueue, WithIndex};

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
