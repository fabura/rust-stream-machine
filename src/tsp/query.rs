use crate::tsp::partitioners::*;
use crate::tsp::patterns::*;
use crate::tsp::projections::*;

pub struct SimpleMachineMapper<Proj, Pat, Part>
where
    Proj: Projection,
    Pat: Pattern,
    Part: Partitioner,
{
    projection: Proj,
    rule: Pat,
    _partitioner: Part,
}

impl<Proj, Pat> SimpleMachineMapper<Proj, Pat, NoPartitioner<Proj::Event>>
where
    Proj: Projection,
    Pat: Pattern<Event = Proj::Event>,
{
    pub fn new(
        projection: Proj,
        rule: Pat,
    ) -> SimpleMachineMapper<Proj, Pat, NoPartitioner<Proj::Event>> {
        SimpleMachineMapper {
            projection,
            rule,
            _partitioner: NoPartitioner::new(), /*, partitioner */
        }
    }
}

impl<Proj, Pat, Part> SimpleMachineMapper<Proj, Pat, Part>
where
    Proj: Projection,
    Pat: Pattern<Event = Proj::Event>,
    Part: Partitioner<Event = Proj::Event>,
{
    pub fn run<J>(&self, events_iter: J, chunks_size: usize) -> TSPIter<Proj, Pat, Part, J>
    where
        J: Iterator<Item = Proj::Event>,
    {
        TSPIter::new(self, Chunker::new(events_iter, chunks_size))
    }
}

pub struct TSPIter<'a, Proj, Pat, Part, J>
where
    J: Iterator<Item = Proj::Event>,
    Proj: Projection,
    Pat: Pattern<Event = Proj::Event>,
    Part: Partitioner<Event = Proj::Event>,
{
    mapper: &'a SimpleMachineMapper<Proj, Pat, Part>,
    chunker: Chunker<J>,
    //todo Maybe need to add something more complicated here
    results_queue: PQueue<Pat::T>,
    projection_state: Proj::State,
    state: Pat::State,
}

impl<Proj, Pat, Part, J> TSPIter<'_, Proj, Pat, Part, J>
where
    J: Iterator<Item = Proj::Event>,
    Proj: Projection,
    Pat: Pattern<Event = Proj::Event>,
    Part: Partitioner<Event = Proj::Event>,
{
    pub fn new(
        mapper: &SimpleMachineMapper<Proj, Pat, Part>,
        chunker: Chunker<J>,
    ) -> TSPIter<Proj, Pat, Part, J> {
        TSPIter {
            mapper,
            chunker,
            results_queue: PQueue::default(),
            projection_state: Proj::State::default(),
            state: Pat::State::default(),
        }
    }
}

impl<Proj, Pat, Part, J> Iterator for TSPIter<'_, Proj, Pat, Part, J>
where
    Proj: Projection,
    Pat: Pattern<Event = Proj::Event>,
    J: Iterator<Item = Proj::Event>,
    Part: Partitioner<Event = Proj::Event>,
{
    type Item = Proj::T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut start_idx = 0u64;
        loop {
            match self.results_queue.dequeue_option() {
                Some(idx_value) => {
                    return Some(self.mapper.projection.extract(
                        &mut self.projection_state,
                        idx_value.start,
                        idx_value.end,
                    ));
                }
                None => {
                    let next_batch = &self.chunker.next()?;
                    self.mapper.rule.apply(
                        start_idx,
                        next_batch,
                        &mut self.results_queue,
                        &mut self.state,
                    );
                    self.mapper.projection.update(
                        start_idx,
                        next_batch,
                        &mut self.projection_state,
                    );
                    start_idx += next_batch.len() as u64;
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
        if !result.is_empty() {
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
