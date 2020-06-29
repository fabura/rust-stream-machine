use crate::tsp::pattern::*;
use std::rc::Rc;
use std::collections::LinkedList;

type BoxedPattern<Event, State, Out> = Box<dyn Pattern<Event, State, Out, W=u64>>;

pub trait Counter<Event, T> {
    fn extract(&self, events: &Vec<Event>) -> T;
}

pub struct SimpleMachineMapper<Event: WithIndex, State: Default, T, Out> {
    rule: BoxedPattern<Event, State, T>,
    counter: Box<dyn Counter<Event, Out>>,

}

impl<Event: WithIndex, State: Default, Out, T> SimpleMachineMapper<Event, State, T, Out> {
    pub fn run(
        &self,
        state: &mut State,
        events: &Vec<Event>,
    ) -> (Vec<Out>) {
        let _changed = self.rule.apply(events, &mut PQueue::default(), state);

        // if changed {
        //
        // }
        vec![]
    }

    pub fn new(rule: BoxedPattern<Event, State, T>, counter: Box<dyn Counter<Event, Out>>) -> SimpleMachineMapper<Event, State, T, Out> {
        SimpleMachineMapper { rule, counter }
    }

    pub fn initial_state(&self) -> State {
        State::default()
    }
}