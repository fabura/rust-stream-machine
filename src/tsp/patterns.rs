#![feature(specialization)]

use crate::tsp::pattern::PatternResult::Success;
use crate::tsp::pattern::{Idx, IdxValue, PQueue, Pattern, PatternResult, WithIndex};
use std::convert::TryInto;
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
        let mut iter = event.iter();

        let mut last = match iter.next() {
            Some(x) => IdxValue::new(x.index(), x.index(), PatternResult::Success((self.func)(x))),
            None => return,
        };

        while let e_opt = iter.next() {
            match e_opt {
                Some(e) => {
                    let value = PatternResult::Success((self.func)(&e));
                    if value == last.result {
                        last.end = e.index()
                    } else {
                        queue.enqueue_one(last);
                        let idx = e.index();
                        last = IdxValue::new(idx, idx, value)
                    }
                }
                None => {
                    queue.enqueue_one(last);
                    break;
                }
            }
        }
    }

    type W = Idx;

    fn width(&self) -> Self::W {
        1
    }
}
