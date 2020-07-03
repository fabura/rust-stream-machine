use crate::tsp::pattern::*;

// use std::marker::PhantomData;
//
// pub trait Counter< Event, T> {
//     fn extract<'b>(&'a self, events: &'b Vec<Event>) -> T;
// }

// pub struct EmptyCounter<T> {
//     phantom: PhantomData<T>
// }
//
// impl<T> EmptyCounter<T> {
//     pub fn new() -> Self {
//         EmptyCounter { phantom: PhantomData }
//     }
// }

// impl< Event> Counter< Event, i32> for EmptyCounter<Event> {
//     fn extract<'b>(&'a self, events: &'b Vec<Event>) -> i32 {
//         -1
//     }
// }

pub struct SimpleMachineMapper<P>
where
    P: Pattern,
{
    rule: P,
    // counter: Box<dyn Counter< Event>>,
}

impl<P> SimpleMachineMapper<P>
where
    P: Pattern,
{
    pub fn new(rule: P /*, counter: Box<dyn Counter< Event>>*/) -> SimpleMachineMapper<P> {
        SimpleMachineMapper { rule }
    }

    pub fn run<J>(&self, events_iter: J, chunks_size: usize) -> TSPIter<P, J>
    where
        J: Iterator<Item = P::Event>,
    {
        TSPIter::new(self, Chunker::new(events_iter, chunks_size))
    }
}

pub struct TSPIter<'a, P, J>
where
    J: Iterator<Item = P::Event>,
    P: Pattern,
{
    mapper: &'a SimpleMachineMapper<P>,
    chunker: Chunker<J>,
    //todo Maybe need to add something more complicated here
    results_queue: PQueue<P::T>,
    state: P::State,
}

impl<P, J> TSPIter<'_, P, J>
where
    J: Iterator<Item = P::Event>,
    P: Pattern,
{
    pub fn new(mapper: &SimpleMachineMapper<P>, chunker: Chunker<J>) -> TSPIter<P, J> {
        TSPIter {
            mapper,
            chunker,
            results_queue: PQueue::default(),
            state: P::State::default(),
        }
    }
}

impl<P, J> Iterator for TSPIter<'_, P, J>
where
    P: Pattern,
    J: Iterator<Item = P::Event>,
{
    type Item = IdxValue<P::T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.results_queue.dequeue_option() {
                v @ Some(_) => return v,
                None => {
                    self.mapper.rule.apply(
                        &self.chunker.next()?,
                        &mut self.results_queue,
                        &mut self.state,
                    );
                }
            }
        }
    }
}

pub struct Chunker<I> {
    iter: I,
    chunks_size: usize,
}

impl<I> Chunker<I>
where
    I: Iterator,
{
    pub(crate) fn new(iter: I, chunks_size: usize) -> Chunker<I> {
        Chunker { iter, chunks_size }
    }
}

impl<I> Iterator for Chunker<I>
where
    I: Iterator,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut counter = 0;
        let mut result = Vec::new();
        while let Some(item) = self.iter.next() {
            result.push(item);
            counter += 1;
            if counter >= self.chunks_size {
                return Some(result);
            }
        }
        if result.len() > 0 {
            Some(result)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.iter.size_hint() {
            (lower, Some(upper)) => (lower, Some(upper / self.chunks_size)),
            h @ (_, _) => h,
        }
    }
}
