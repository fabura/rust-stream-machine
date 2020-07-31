use std::hash::Hash;
use std::marker::PhantomData;

pub trait Partitioner {
    type Event;
    type T: Clone + Eq + Hash;

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

// todo: Can we avoid Box here?
type BoxedFn<'a, E, T> = Box<dyn Fn(&'a E) -> T>;

pub struct FunctionPartitioner<'a, E, T> where T: Clone + Eq + Hash {
    func: BoxedFn<'a, E, T>,
}

impl<'a, E, T> FunctionPartitioner<'a, E, T> where T: Clone + Eq + Hash {
    pub fn new<F>(func: F) -> Self where F: Fn(&E) -> T + 'static {
        FunctionPartitioner { func: Box::new(func) }
    }
}

impl<'a, F, E, T> From<F> for FunctionPartitioner<'a, E, T> where F: Fn(&E) -> T + 'static, T: Clone + Eq + Hash {
    fn from(f: F) -> Self {
        FunctionPartitioner::new(f)
    }
}

impl<E> Partitioner for NoPartitioner<E> {
    type Event = E;
    type T = ();

    fn partition_key(&self, _event: &Self::Event) -> Self::T {}
}

impl<'a, E, T> Partitioner for FunctionPartitioner<'a, E, T> where T: Clone + Eq + Hash {
    type Event = &'a E;
    type T = T;

    fn partition_key(&self, event: &Self::Event) -> Self::T {
        (self.func)(event)
    }
}