use std::borrow::BorrowMut;
use std::boxed::Box;
use std::iter::Iterator;
use std::ops::Add;
use std::rc::Rc;
use stream::operators::Binary;
use stream::operators::BinaryPattern;

#[derive(Debug)]
pub enum ParseResult<T> {
    Stay,
    Failure { message: String },
    Success(T),
}

impl<T> ParseResult<T> {
    pub fn is_terminal(&self) -> bool {
        match &self {
            ParseResult::Stay => false,
            ParseResult::Failure { message: _ } => true,
            ParseResult::Success(_) => true,
        }
    }

    pub fn map<T2, F>(self, f: F) -> ParseResult<T2>
    where
        F: FnOnce(T) -> T2,
    {
        match self {
            ParseResult::Success(t) => ParseResult::Success(f(t)),
            ParseResult::Stay => ParseResult::Stay,
            ParseResult::Failure { message } => ParseResult::Failure { message },
        }
    }

    pub fn flat_map<T2, F>(self, f: F) -> ParseResult<T2>
    where
        F: FnOnce(T) -> ParseResult<T2>,
    {
        match self {
            ParseResult::Success(t) => f(t),
            ParseResult::Stay => ParseResult::Stay,
            ParseResult::Failure { message } => ParseResult::Failure { message },
        }
    }
}

pub trait StreamPattern<Event, State, T>
where
    State: Default,
{
    fn apply(&self, event: &Event, state: &mut State) -> ParseResult<T>;
}

//pub trait FromSlice<State>{
//
//   //  we can use
//    fn extractState(&mut [Box<T>]) -> State
//}

#[derive(Debug)]
struct NoState;

impl Default for NoState {
    fn default() -> Self {
        NoState
    }
}

pub struct ConstantParser<T: 'static>(T);

impl<T> ConstantParser<T> {
    fn new(t: T) -> ConstantParser<T> {
        ConstantParser(t)
    }
}

impl<Event, T: Clone> StreamPattern<Event, NoState, T> for ConstantParser<T> {
    fn apply(&self, _event: &Event, _state: &mut NoState) -> ParseResult<T> {
        ParseResult::Success(self.0.clone())
    }
}

type BoxedFn<Event, T> = Box<Fn(&Event) -> T>;

pub struct FunctionParser<Event, T: 'static> {
    func: BoxedFn<Event, T>,
}

impl<Event, T> FunctionParser<Event, T> {
    fn new<C>(f: C) -> FunctionParser<Event, T>
    where
        C: Fn(&Event) -> T + 'static,
    {
        FunctionParser { func: Box::new(f) }
    }
}

impl<Event, T> StreamPattern<Event, NoState, T> for FunctionParser<Event, T> {
    fn apply(&self, event: &Event, _state: &mut NoState) -> ParseResult<T> {
        ParseResult::Success::<T>((&(self.func))(event))
    }
}

type BoxedPattern<Event, State, Out> = Box<Rc<StreamPattern<Event, State, Out>>>;

struct SimpleMachineMapper<Event, State: Default, Out> {
    rule: BoxedPattern<Event, State, Out>,
}

impl<Event, State: Default, Out> SimpleMachineMapper<Event, State, Out> {
    pub fn run<'a, 'b>(
        &'a self,
        old_states: Vec<State>,
        event: &'b Event,
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
    let rule = FunctionParser::new(|e: &TEvent| -> f64 { e.v });
    let rule1 = FunctionParser::new(|e: &TEvent| -> bool { e.v > 12.0});
    let rule2 = FunctionParser::new(|e: &TEvent| -> bool { e.time < 100 });

    let rule3 = rule1.and(rule2);
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
