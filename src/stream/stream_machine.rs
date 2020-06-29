use std::borrow::BorrowMut;
use std::boxed::Box;
use std::iter::Iterator;
use std::rc::Rc;
use crate::stream::operators::Binary;
use crate::stream::functions::*;
use crate::stream::pattern::*;

pub struct ConstantParser<T: 'static>(T);

impl<T> ConstantParser<T> {
    fn new(t: T) -> ConstantParser<T> {
        ConstantParser(t)
    }
}

impl<Event, T: Clone> Pattern<Event, NoState, T> for ConstantParser<T> {
    fn apply(&self, _event: &Event, _state: &mut NoState) -> ParseResult<T> {
        ParseResult::Success(self.0.clone())
    }
}


type BoxedPattern<Event, State, Out> = Box<Rc<dyn Pattern<Event, State, Out>>>;

struct SimpleMachineMapper<Event, State: Default, Out> {
    rule: BoxedPattern<Event, State, Out>,
}

impl<Event, State: Default, Out> SimpleMachineMapper<Event, State, Out> {
    pub fn run(
        &self,
        old_states: Vec<State>,
        event: &Event,
    ) -> (Vec<ParseResult<Out>>, Vec<State>) {
        let mut results = old_states
            .into_iter()
            .map(move |mut st| (self.rule.apply(event, st.borrow_mut()), st));

        let to_emit = results
            .by_ref()
            .map(|tuple| tuple.0)
            .take_while(|parse_result| parse_result.is_terminal())
            .collect::<Vec<_>>();

        let new_states = results.map(|tuple| tuple.1).collect::<Vec<_>>();

        (to_emit, new_states)
    }

    pub fn initial_state(&self) -> State {
        State::default()
    }
}

// test part below this line

struct TEvent {
    time: u64,
    v: f64,
}

impl TEvent {
    fn new(time: u64, v: f64) -> TEvent {
        TEvent { time, v }
    }
}

pub fn run_rule() -> () {
    let v = FunctionPattern::new(|e: &TEvent| -> f64 { e.v });
    let time = FunctionPattern::new(|e: &TEvent| -> f64 { e.time as f64});
    let _rule1 = FunctionPattern::new(|e: &TEvent| -> bool { e.v > 12.0 });
    let _rule2 = FunctionPattern::new(|e: &TEvent| -> bool { e.time < 100 });

    let rule3 = v.plus(time).more(ConstantParser::new(3.0));
    let machine_mapper = SimpleMachineMapper {
        rule: Box::new(Rc::new(rule3)),
    };
    let mut results =
        machine_mapper.run(vec![machine_mapper.initial_state()], &TEvent::new(23, 34.2));
    println!("{:?}", &results);
    results.1.push(machine_mapper.initial_state());
    let results = machine_mapper.run(results.1, &TEvent::new(25, 45.2));
    println!("{:?}", &results);
}
