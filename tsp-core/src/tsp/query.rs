use std::collections::HashMap;

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
    partitioner: Part,
}

impl<Proj, Pat, Part> SimpleMachineMapper<Proj, Pat, Part>
where
    Proj: Projection,
    Pat: Pattern<Event = Proj::Event>,
    Part: Partitioner<Event = Proj::Event>,
{
    pub fn new(
        projection: Proj,
        rule: Pat,
        partitioner: Part,
    ) -> SimpleMachineMapper<Proj, Pat, Part> {
        SimpleMachineMapper {
            projection,
            rule,
            partitioner,
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
        TSPIter::new(
            self,
            events_iter.partition_by(&self.partitioner, chunks_size, 1000), // todo make configurable
        )
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
    partition_iterator: PartitionIterator<'a, J, Part>,
    results_queues: HashMap<Part::T, PQueue<Pat::T>>,
    projection_states: HashMap<Part::T, Proj::State>,
    states: HashMap<Part::T, (Pat::State, Idx)>,
}

impl<'a, Proj, Pat, Part, J> TSPIter<'a, Proj, Pat, Part, J>
where
    J: Iterator<Item = Proj::Event>,
    Proj: Projection,
    Pat: Pattern<Event = Proj::Event>,
    Part: Partitioner<Event = Proj::Event>,
{
    pub fn new(
        mapper: &'a SimpleMachineMapper<Proj, Pat, Part>,
        partition_iterator: PartitionIterator<'a, J, Part>,
    ) -> TSPIter<'a, Proj, Pat, Part, J> {
        TSPIter::<'a, Proj, Pat, Part, J> {
            mapper,
            partition_iterator,
            results_queues: HashMap::default(),
            projection_states: HashMap::default(),
            states: HashMap::default(),
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
        loop {
            // If there is already some results we return them.
            if !self.results_queues.is_empty() {
                let key = self
                    .results_queues
                    .keys()
                    .take(1)
                    .next()
                    .expect("Illegal state")
                    .clone();
                let results = self.results_queues.entry(key.clone()).or_default(); // this must not return empty results
                let idx_value = results
                    .dequeue_option()
                    .expect("Illegal state: empty results must be deleted from results_queues!");

                if results.is_empty() {
                    self.results_queues.remove(&key);
                }

                let projection_state = self.projection_states.entry(key.clone()).or_default();
                return Some(self.mapper.projection.extract(
                    projection_state,
                    idx_value.start,
                    idx_value.end,
                ));
            } else {
                // compute next batch
                let next_batch = &self.partition_iterator.next()?;
                let key = (&next_batch.key).clone();
                let (state, start_idx) = self.states.entry(key.clone()).or_default();
                self.mapper.rule.apply(
                    *start_idx,
                    &next_batch.elements,
                    self.results_queues.entry(key.clone()).or_default(),
                    state,
                );
                self.mapper.projection.update(
                    *start_idx,
                    &next_batch.elements,
                    self.projection_states.entry(key).or_default(),
                );
                *start_idx += next_batch.elements.len() as u64;
            }
        }
    }
}
