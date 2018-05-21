use geotree::stream_machine;
use std::iter::Iterator;
use std::option::Iter;

pub enum ParseResult<T> {
    Stay,
    Failure {
        message: String
    },
    Success(T),
}

pub trait StreamPattern<Event, State, T>
    where State: Default
{
    fn apply(&self, event: &Event, state: &mut State) -> ParseResult<T>;
}

struct NoState;

impl Default for NoState { fn default() -> Self { NoState } }

pub struct ConstantParser<T: 'static>(T);

impl<T> ConstantParser<T> { fn new(t: T) -> ConstantParser<T> { ConstantParser(t) } }

impl<Event, T: Clone> StreamPattern<Event, NoState, T> for ConstantParser<T> {
    fn apply(&self, event: &Event, _state: &mut NoState) -> ParseResult<T> {
        ParseResult::Success(self.0.clone())
    }
}

type BoxedFn<Event, T> = Box<Fn(&Event) -> T>;

pub struct FunctionParser<Event, T: 'static> { func: BoxedFn<Event, T> }

impl<Event, T> FunctionParser<Event, T> {
    fn new<C>(f: C) -> FunctionParser<Event, T>
        where C: Fn(&Event) -> T + 'static {
        FunctionParser {
            func: Box::new(f),
        }
    }
}

impl<Event, T> StreamPattern<Event, NoState, T> for FunctionParser<Event, T> {
    fn apply(&self, event: &Event, _state: &mut NoState) -> ParseResult<T> {
        ParseResult::Success::<T>((&(self.func))(event))
    }
}