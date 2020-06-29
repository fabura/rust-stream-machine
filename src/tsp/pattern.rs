use std::collections::VecDeque;

pub trait Pattern<Event, State, T> where
    State: Default,
    Event: WithIndex
{
    fn apply(&self, event: &Vec<Event>, queue: &mut PQueue<T>, state: &mut State) -> bool;

    type W: Width;

    fn width(&self) -> Self::W;
}

pub trait Width {}

impl Width for u64 {}

pub type Idx = u64;

pub trait WithIndex {
    fn index(&self) -> Idx;
}

#[derive(Debug)]
pub enum Result<T: Sized> {
    Failure,
    Success(T), //todo make result fixed size
}

#[derive(Debug)]
struct IdxValue<T> {
    start: Idx,
    end: Idx,
    result: Result<T>,
}

#[derive(Debug)]
pub struct PQueue<T> {
    queue: std::collections::VecDeque<IdxValue<T>>
}

impl<T> Default for PQueue<T>{
    fn default() -> Self {
        PQueue{
            queue: VecDeque::default()
        }
    }
}

// В каждый момент времени мы знаем начало и конец сработки, какие внутри ивенты