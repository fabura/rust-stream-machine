use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

pub trait Partitioner {
    type Event;
    type T: Debug + Clone + Eq + Hash;

    fn partition_key(&self, event: &Self::Event) -> Self::T;
}

#[derive(Default)]
pub struct NoPartitioner<E> {
    pf: PhantomData<E>,
}

impl<E> NoPartitioner<E> {
    pub fn new() -> Self {
        NoPartitioner { pf: PhantomData }
    }
}

pub struct FunctionPartitioner<'a, E, T, F>
where
    F: Fn(&'a E) -> T,
    T: Clone + Eq + Hash,
{
    func: F,
    ph: PhantomData<(&'a E, T)>,
}

impl<'a, E, T, F> FunctionPartitioner<'a, E, T, F>
where
    F: Fn(&'a E) -> T,
    T: Clone + Eq + Hash,
{
    pub fn new(func: F) -> Self {
        FunctionPartitioner {
            func,
            ph: PhantomData,
        }
    }
}

impl<E> Partitioner for NoPartitioner<E> {
    type Event = E;
    type T = ();

    fn partition_key(&self, _event: &Self::Event) -> Self::T {}
}

impl<'a, E, T, F> Partitioner for FunctionPartitioner<'a, E, T, F>
where
    F: Fn(&'a E) -> T,
    T: Debug + Clone + Eq + Hash,
{
    type Event = &'a E;
    type T = T;

    fn partition_key(&self, event: &Self::Event) -> Self::T {
        (self.func)(event)
    }
}
