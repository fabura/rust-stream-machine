use std::collections::VecDeque;

pub trait Pattern {
    type State: Default;
    type Event: WithIndex;
    type T: Clone;
    fn apply(&self, event: &Vec<Self::Event>, queue: &mut PQueue<Self::T>, state: &mut Self::State);

    type W: Width;

    fn width(&self) -> Self::W;
}

pub trait Width {}

impl Width for u64 {}

pub type Idx = u64;

pub trait WithIndex {
    fn index(&self) -> Idx;
}

#[derive(Debug, Clone)]
pub enum PatternResult<T: Sized>
where
    T: Clone,
{
    Failure,
    Success(T), //todo make result fixed size
}

impl<T> PartialEq for PatternResult<T>
where
    T: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PatternResult::Failure, PatternResult::Failure) => true,
            (PatternResult::Success(a), PatternResult::Success(b)) if a == b => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdxValue<T: Clone> {
    pub start: Idx,
    pub end: Idx,
    pub result: PatternResult<T>,
}

impl<T: Clone> IdxValue<T> {
    pub fn new(start: Idx, end: Idx, result: PatternResult<T>) -> Self {
        IdxValue { start, end, result }
    }
}

#[derive(Debug)]
pub struct PQueue<T: Clone> {
    queue: std::collections::VecDeque<IdxValue<T>>,
}

impl<T: Clone> Default for PQueue<T> {
    fn default() -> Self {
        PQueue {
            queue: VecDeque::default(),
        }
    }
}

impl<T: Clone> PQueue<T> {
    pub(crate) fn size(&self) -> usize {
        self.queue.len()
    }

    pub(crate) fn head_option(&self) -> Option<&IdxValue<T>> {
        self.queue.front()
    }

    pub(crate) fn dequeue_option(&mut self) -> Option<IdxValue<T>> {
        self.queue.pop_front()
    }
    //  fn behead(&mut self)-> () {
    //      self.queue.emp
    //  }
    //  fn behead_option(&mut self)-> Option<&PQueue<T>>{
    //      self.queue.pop_front().map(|| self)
    //  }
    pub(crate) fn enqueue(&mut self, idx_values: impl Iterator<Item = IdxValue<T>>) -> &mut Self {
        self.queue.extend(idx_values);
        self
    }
    pub(crate) fn enqueue_one(&mut self, idx_value: IdxValue<T>) -> &mut Self {
        self.queue.push_back(idx_value);
        self
    }
    //  fn rewind_to(newStart: Idx): PQueue[T]
    //  fn clean(): PQueue[T]
    //
    // fn to_seq: Seq[IdxValue[T]]
}

// В каждый момент времени мы знаем начало и конец сработки, какие внутри ивенты
