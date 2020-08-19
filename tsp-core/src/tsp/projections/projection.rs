use std::collections::VecDeque;
use std::marker::PhantomData;

use crate::tsp::patterns::pattern::Idx;

pub trait Projection {
    type Event;
    type State: Default;
    type T: Clone;
    /// This function is called by query.rs for every chunk of input events to update State.
    /// `events` - chunk of input Events
    /// `start_idx` - index of the first element in `events`
    /// `state` - state of this Projection to be updated.
    fn update(&self, start_idx: Idx, events: &[Self::Event], state: &mut Self::State);

    /// Returns extracted projection for window from `start` to `end`. Also can modify `state`.
    /// The maintained invariant is that for two sequential calls of `extract` start will be not
    /// decreasing (can be the same or greater).
    fn extract(&self, state: &mut Self::State, start: Idx, end: Idx) -> Self::T;
}

pub struct ConstantProjection<E, T>(T, PhantomData<E>);

#[derive(Default, Debug, PartialEq)]
pub struct NoProjectionState;

impl<E, T: Clone> ConstantProjection<E, T> {
    #[allow(dead_code)]
    pub fn new(t: T) -> Self {
        ConstantProjection(t, PhantomData)
    }
}

impl<Event, T: Clone> Projection for ConstantProjection<Event, T> {
    type Event = Event;
    type State = NoProjectionState;
    type T = T;

    fn update(&self, _start_idx: u64, _events: &[Self::Event], _state: &mut Self::State) {}

    fn extract(&self, _state: &mut Self::State, _start: Idx, _end: Idx) -> Self::T {
        self.0.clone()
    }
}

macro_rules! queue_projection {
    ( $name:ident, $extract:expr) => {
        pub struct $name<E, F: Fn(&E) -> T, T>(F, PhantomData<E>, PhantomData<T>);
        impl<E, F, T> $name<E, F, T>
        where
            F: Fn(&E) -> T,
        {
            pub fn new(field0: F) -> Self {
                $name(field0, PhantomData, PhantomData)
            }
        }
        impl<E, F: Fn(&E) -> T, T: Clone> Projection for $name<E, F, T> {
            type Event = E;
            type State = QueueProjectionState<T>;
            type T = T;

            fn update(&self, _start_idx: Idx, events: &[Self::Event], state: &mut Self::State) {
                state
                    .queue
                    .append(&mut events.iter().map(|x| self.0(x)).collect())
            }

            fn extract(&self, state: &mut Self::State, start: u64, end: u64) -> Self::T {
                assert!(state.queue.len() > (end - state.first_idx) as usize);
                assert!(start <= end);
                assert!(state.first_idx <= start);
                $extract(state, start, end)
            }
        }
    };
}

#[derive(Debug, PartialEq)]
pub struct QueueProjectionState<T> {
    queue: VecDeque<T>,
    first_idx: Idx,
}

impl<T> Default for QueueProjectionState<T> {
    fn default() -> Self {
        QueueProjectionState {
            queue: VecDeque::new(),
            first_idx: 0,
        }
    }
}

fn first<T: Clone>(state: &mut QueueProjectionState<T>, start: u64, end: u64) -> T {
    state.queue.drain(..(start - state.first_idx) as usize);
    let res = state.queue.front().unwrap().clone();
    state.queue.drain(..(end - start + 1) as usize);
    state.first_idx = end + 1;
    res
}

queue_projection!(FirstProjection, first);

fn last<T: Clone>(state: &mut QueueProjectionState<T>, start: u64, end: u64) -> T {
    let res = state.queue.get((end - state.first_idx) as usize).unwrap().clone();
    state.queue.drain(..(end - state.first_idx + 1) as usize);
    state.first_idx = end + 1;
    res
}

queue_projection!(LastProjection, last);


#[cfg(test)]
mod tests {
    use super::*;

    fn run_projection<P: Projection>(p: &P, events: &[P::Event]) -> <P as Projection>::State {
        let mut state = default_state(p);
        p.update(0, events, &mut state);
        state
    }

    fn default_state<P: Projection>(p: &P) -> <P as Projection>::State {
        P::State::default()
    }

    #[test]
    fn const_projection() {
        let expected = 3;
        let constant_projection = ConstantProjection::new(expected);
        let mut updated_state = run_projection(&constant_projection, &[(0, 0), (1, 1)]);

        let extracted_value = constant_projection.extract(&mut updated_state, 0, 1);
        assert_eq!(extracted_value, expected);
        assert_eq!(updated_state, NoProjectionState);
    }

    struct TE(usize, usize);

    #[test]
    fn first_projection() {
        let expected = 13;
        let first_projection = FirstProjection::new(|e: &TE| e.1);
        let mut updated_state =
            run_projection(&first_projection, &[TE(0, 20), TE(1, 13), TE(5, 34)]);

        let extracted_value = first_projection.extract(&mut updated_state, 1, 2);
        assert_eq!(extracted_value, expected);
        assert_eq!(updated_state.first_idx, 3);
        assert!(updated_state.queue.is_empty());
    }


    #[test]
    fn last_projection() {
        let expected = 34;
        let last_projection = LastProjection::new(|e: &TE| e.1);
        let mut updated_state =
            run_projection(&last_projection, &[TE(0, 20), TE(1, 13), TE(2, 34), TE(3, 567)]);

        let extracted_value = last_projection.extract(&mut updated_state, 1, 2);
        assert_eq!(extracted_value, expected);
        assert_eq!(updated_state.first_idx, 3);
        assert_eq!(updated_state.queue.len(), 1);
    }
}
