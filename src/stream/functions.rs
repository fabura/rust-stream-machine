use stream::pattern::*;

type BoxedFn<Event, T> = Box<Fn(&Event) -> T>;

pub struct FunctionPattern<Event, T: 'static> {
    func: BoxedFn<Event, T>,
}

impl<Event, T> FunctionPattern<Event, T> {
    pub fn new<C>(f: C) -> FunctionPattern<Event, T>
        where
            C: Fn(&Event) -> T + 'static,
    {
        FunctionPattern { func: Box::new(f) }
    }
}

impl<Event, T> Pattern<Event, NoState, T> for FunctionPattern<Event, T> {
    fn apply(&self, event: &Event, _state: &mut NoState) -> ParseResult<T> {
        ParseResult::Success::<T>((&(self.func))(event))
    }
}