#[derive(Debug)]
pub enum ParseResult<T> {
    Stay,
    Failure { message: String },
    Success(T), //todo make parseresult fixed size
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

pub trait Pattern<Event, State: Default, T>
{
    fn apply(&self, event: &Event, state: &mut State) -> ParseResult<T>;
}

#[derive(Debug)]
pub struct NoState;

impl Default for NoState {
    fn default() -> Self {
        NoState
    }
}