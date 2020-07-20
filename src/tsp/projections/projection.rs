use std::marker::PhantomData;

use crate::tsp::patterns::pattern::{Idx, WithIndex};

pub trait Projection {
    type Event: WithIndex;
    type State: Default;
    type T: Clone;
    fn update(&self, events: &[Self::Event], state: &mut Self::State);

    fn extract(&self,  state: &mut Self::State, start: Idx, end: Idx) -> Self::T;
}

pub struct ConstantProjection<E, T>(T, PhantomData<E>);

#[derive(Default)]
pub struct NoProjectionState;

impl<E, T: Clone> ConstantProjection<E, T> {
    pub fn new(t: T) -> Self {
        ConstantProjection(t, PhantomData)
    }
}

impl<Event: WithIndex, T: Clone> Projection for ConstantProjection<Event, T> {
    type Event = Event;
    type State = NoProjectionState;
    type T = T;

    fn update(&self, _events: &[Self::Event], _state: &mut Self::State) {}

    fn extract(&self, _state: &mut Self::State, _start: Idx, _end: Idx) -> Self::T {
        self.0.clone()
    }
}