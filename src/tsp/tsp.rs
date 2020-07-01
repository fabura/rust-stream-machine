use std::collections::VecDeque;

// use itertools::{Itertools, IntoChunks, Chunks};


use crate::tsp::pattern::*;
use std::marker::PhantomData;

// type BoxedPattern<Event, State,T> = Box<dyn Pattern<Event, State,T, W=u64>>;

// pub trait Counter<'a, Event, T> {
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

// impl<'a, Event> Counter<'a, Event, i32> for EmptyCounter<Event> {
//     fn extract<'b>(&'a self, events: &'b Vec<Event>) -> i32 {
//         -1
//     }
// }

pub struct SimpleMachineMapper<P> where P: Pattern {
    rule: P,
    // counter: Box<dyn Counter<'a, Event>>,
}

impl<P> SimpleMachineMapper<P> where P: Pattern {
    pub fn new(rule: P/*, counter: Box<dyn Counter<'a, Event>>*/) -> SimpleMachineMapper<P> {
        SimpleMachineMapper { rule }
    }

    pub fn run<'a, J>(mut self, events_iter: J, chunks_size: usize) -> TSPIter<'a, P, J>
        where J: Iterator<Item=P::Event>, J: 'a
    {
        TSPIter::new(self, Chunker::new(events_iter, chunks_size))
    }
}

pub struct TSPIter<'a, P, J> where
    J: Iterator<Item=P::Event>,
    P: Pattern,
{
    mapper: SimpleMachineMapper<P>,
    chunker: Chunker<'a, J>,
    //todo Maybe need to add something more complicated here
    results_queue: PQueue<P::T>,
    state: P::State,
}

impl<'a, P, J> TSPIter<'a, P, J> where J: Iterator<Item=P::Event>, P: Pattern
{
    pub fn new(mapper: SimpleMachineMapper<P>, chunker: Chunker<'a, J>) -> TSPIter<'a, P, J> {
        TSPIter {
            mapper,
            chunker,
            results_queue: PQueue::default(),
            state: P::State::default(),
        }
    }
}


impl<'a, P, J> Iterator for TSPIter<'a, P, J> where P: Pattern, J: Iterator<Item=P::Event>
{
    type Item = IdxValue<P::T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.results_queue.dequeue_option() {
                v @ Some(_) => return v,
                None => { self.mapper.rule.apply(&self.chunker.next()?, &mut self.results_queue, &mut self.state); }
            }
        }
    }
}

pub struct Chunker<'a, I> where I: 'a {
    iter: I,
    chunks_size: usize,
    pf: PhantomData<&'a i32>,
}

impl<'a, I> Chunker<'a, I> where I: Iterator, I: 'a {
    pub(crate) fn new(iter: I, chunks_size: usize) -> Chunker<'a, I> {
        Chunker { iter, chunks_size, pf: PhantomData }
    }
}

impl<'a, I> Iterator for Chunker<'a, I> where I: Iterator, I: 'a {
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
            h @ (_, _) => h
        }
    }
}