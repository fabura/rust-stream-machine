use std::collections::VecDeque;
use std::marker::PhantomData;

use crate::tsp::patterns::pattern::{Idx, WithIndex};

pub trait Projection {
    type Event: WithIndex;
    type State: Default;
    type T: Clone;
    /// This function is called by query.rs for every chunk of input events to update State.
    /// `events` - chunk of input Events
    /// `state` - state of this Projection to be updated.
    fn update(&self, events: &[Self::Event], state: &mut Self::State);

    /// Returns extracted projection for window from `start` to `end`. Also can modify `state`.
    /// The maintained invariant is that for two sequential calls of `extract` start will be not
    /// decreasing (can be the same or greater).
    fn extract(&self, state: &mut Self::State, start: Idx, end: Idx) -> Self::T;
}

pub struct ConstantProjection<E, T>(T, PhantomData<E>);

#[derive(Default, Debug)]
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


#[derive(Debug)]
pub struct QueueProjectionState<T> {
    queue: VecDeque<T>,
    first_idx: Idx,
}

impl<T> Default for QueueProjectionState<T> {
    fn default() -> Self {
        QueueProjectionState { queue: VecDeque::new(), first_idx: 0 }
    }
}

pub struct FirstProjection<E, F, T>(F, PhantomData<E>, PhantomData<T>);

impl<E, F, T> FirstProjection<E, F, T> where F: Fn(E) -> T {
    pub fn new(field0: F) -> Self {
        FirstProjection(field0, PhantomData, PhantomData)
    }
}

impl<E: WithIndex, F, T: Clone> Projection for FirstProjection<E, F, T> {
    type Event = E;
    type State = QueueProjectionState<T>;
    type T = T;

    fn update(&self, events: &[Self::Event], state: &mut Self::State) {
        state.queue.append(events.iter().map(|x| self.0(x)).collect())
    }

    fn extract(&self, state: &mut Self::State, start: u64, end: u64) -> Self::T {
        assert!(state.queue.len() > (start - state.first_idx) as usize);
        state.queue.drain(..(start - state.first_idx));
        state.first_idx = start;
        state.queue.front().unwrap()
    }
}

