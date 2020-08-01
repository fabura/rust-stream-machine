use std::collections::HashMap;

use crate::tsp::partitioners::Partitioner;

/// Contains partitioning key K and &Vec[E] with elements
#[derive(PartialEq, Debug)]
pub struct Chunk<K, E> {
    pub key: K,
    pub elements: Vec<E>,
}

pub(crate) trait PartitionIterTool: Iterator + Sized {
    fn partition_by<P>(
        self,
        partitioner: &P,
        chunk_max_size: usize,
        total_size_limit: usize,
    ) -> PartitionIterator<Self, P>
    where
        P: Partitioner<Event = Self::Item>;
}

impl<T: Iterator> PartitionIterTool for T {
    fn partition_by<P>(
        self,
        partitioner: &P,
        chunk_max_size: usize,
        total_size_limit: usize,
    ) -> PartitionIterator<Self, P>
    where
        P: Partitioner<Event = Self::Item>,
    {
        assert!(chunk_max_size > 0);
        assert!(total_size_limit > 0);
        PartitionIterator {
            iter: self,
            partitioner,
            chunk_max_size,
            map: HashMap::new(),
            total_size: 0,
            total_size_limit,
        }
    }
}

pub struct PartitionIterator<'a, J, Part>
where
    J: Iterator,
    Part: Partitioner<Event = J::Item>,
{
    iter: J,
    partitioner: &'a Part,
    chunk_max_size: usize,
    map: std::collections::HashMap<Part::T, Vec<J::Item>>,
    total_size: usize,
    total_size_limit: usize,
}

impl<J, Part> Iterator for PartitionIterator<'_, J, Part>
where
    J: Iterator,
    Part: Partitioner<Event = J::Item>,
{
    type Item = Chunk<Part::T, J::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.iter.next() {
            let key = self.partitioner.partition_key(&x);
            let key_clone = key.clone();
            let chunk = self.map.entry(key).or_default();

            if chunk.capacity() <= self.chunk_max_size {
                chunk.reserve(self.chunk_max_size);
            }
            chunk.push(x);
            self.total_size += 1;

            if chunk.len() >= self.chunk_max_size {
                let elements = self.map.remove(&key_clone).expect("Illegal state");
                self.total_size -= elements.len();
                return Some(Chunk {
                    key: key_clone,
                    elements,
                });
            }

            // if we overcome the limit, then return first value.
            if self.total_size >= self.total_size_limit {
                let (key, _) = self.map.iter().next().expect("Illegal state");
                let key = key.clone();
                let (key, elements) = self.map.remove_entry(&key).expect("Illegal state");
                self.total_size -= elements.len();
                return Some(Chunk { key, elements });
            }
        }

        // if there is not more elements in inner iterator, we start emitting all keys.
        if !self.map.is_empty() {
            let key = self
                .map
                .keys()
                .take(1)
                .next()
                .expect("Illegal state")
                .clone();
            let (key, elements) = self.map.remove_entry(&key).expect("Illegal state");
            self.total_size -= elements.len();
            return Some(Chunk { key, elements });
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use super::super::partitioner::*;
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestEvent {
        partition_key: usize,
        value: usize,
    }

    impl TestEvent {
        pub fn new(partition_key: usize, value: usize) -> Self {
            TestEvent {
                partition_key,
                value,
            }
        }
    }

    #[test]
    fn works_for_empty_iterator() {
        let empty: Vec<TestEvent> = vec![];
        assert_eq!(
            empty
                .iter()
                .partition_by(&NoPartitioner::<&TestEvent>::new(), 1, 1)
                .into_iter()
                .next(),
            None
        )
    }

    #[test]
    fn returns_chunks() {
        let input = vec![
            TestEvent::new(0, 1),
            TestEvent::new(0, 2),
            TestEvent::new(0, 3),
            TestEvent::new(0, 4),
            TestEvent::new(0, 5),
            TestEvent::new(0, 6),
        ];

        let partitioner = NoPartitioner::new();
        let mut iterator = input.iter().partition_by(&partitioner, 2, 100).into_iter();
        assert_eq!(
            iterator.next(),
            Some(Chunk {
                key: (),
                elements: vec![&TestEvent::new(0, 1), &TestEvent::new(0, 2)],
            })
        );
        assert_eq!(
            iterator.next(),
            Some(Chunk {
                key: (),
                elements: vec![&TestEvent::new(0, 3), &TestEvent::new(0, 4)],
            })
        );
        assert_eq!(
            iterator.next(),
            Some(Chunk {
                key: (),
                elements: vec![&TestEvent::new(0, 5), &TestEvent::new(0, 6)],
            })
        );
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn returns_non_completed_chunks() {
        let input = vec![TestEvent::new(0, 1)];

        let partitioner = NoPartitioner::new();
        let mut iterator = input
            .iter()
            .partition_by(&partitioner, 100, 100)
            .into_iter();
        assert_eq!(
            iterator.next(),
            Some(Chunk {
                key: (),
                elements: vec![&TestEvent::new(0, 1)],
            })
        );
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn respects_total_size_limit() {
        let input = vec![
            TestEvent::new(0, 1),
            TestEvent::new(0, 2),
            TestEvent::new(0, 3),
            TestEvent::new(0, 4),
            TestEvent::new(0, 5),
            TestEvent::new(0, 6),
        ];

        let partitioner = NoPartitioner::new();
        let mut iterator = input.iter().partition_by(&partitioner, 100, 2).into_iter();
        assert_eq!(
            iterator.next(),
            Some(Chunk {
                key: (),
                elements: vec![&TestEvent::new(0, 1), &TestEvent::new(0, 2)],
            })
        );
        assert_eq!(
            iterator.next(),
            Some(Chunk {
                key: (),
                elements: vec![&TestEvent::new(0, 3), &TestEvent::new(0, 4)],
            })
        );
        assert_eq!(
            iterator.next(),
            Some(Chunk {
                key: (),
                elements: vec![&TestEvent::new(0, 5), &TestEvent::new(0, 6)],
            })
        );
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn partition_using_function() {
        let input = vec![
            TestEvent::new(0, 1),
            TestEvent::new(1, 2),
            TestEvent::new(0, 3),
            TestEvent::new(1, 4),
            TestEvent::new(0, 5),
            TestEvent::new(1, 6),
        ];
        fn partition_fn(t: &TestEvent) -> usize {
            t.partition_key
        }

        let partitioner = FunctionPartitioner::new(partition_fn);
        let mut iterator = input.iter().partition_by(&partitioner, 2, 100).into_iter();
        assert_eq!(
            iterator.next(),
            Some(Chunk {
                key: 0,
                elements: vec![&TestEvent::new(0, 1), &TestEvent::new(0, 3)],
            })
        );
        assert_eq!(
            iterator.next(),
            Some(Chunk {
                key: 1,
                elements: vec![&TestEvent::new(1, 2), &TestEvent::new(1, 4)],
            })
        );
        let chunk = iterator.next().expect("");
        assert_eq!(chunk.elements.len(), 1);
        let chunk = iterator.next().expect("");
        assert_eq!(chunk.elements.len(), 1);
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn partition_using_function_total_size_limited() {
        let input = vec![
            TestEvent::new(0, 1),
            TestEvent::new(1, 2),
            TestEvent::new(0, 3),
            TestEvent::new(1, 4),
            TestEvent::new(0, 5),
            TestEvent::new(1, 6),
        ];
        fn partition_fn(t: &TestEvent) -> usize {
            t.partition_key
        }

        let partitioner = FunctionPartitioner::new(partition_fn);
        let mut iterator = input.iter().partition_by(&partitioner, 2, 1).into_iter();
        assert!(iterator.next().is_some());
        assert!(iterator.next().is_some());
        assert!(iterator.next().is_some());
        assert!(iterator.next().is_some());
        assert!(iterator.next().is_some());
        assert!(iterator.next().is_some());
        assert!(iterator.next().is_none());
    }
}
