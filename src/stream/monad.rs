use crate::stream::pattern::*;
use std::marker::PhantomData;

pub struct MapPattern<E, S, T1, T2, A, F>
where
    A: Pattern<E, S, T1>,
    S: Default,
    F: Fn(T1) -> T2,
{
    inner: A,
    f: F,
    e: PhantomData<E>,
    st1: PhantomData<S>,
    t1: PhantomData<T1>,
    t2: PhantomData<T2>,
}

impl<E, S, T1, T2, A, F> Pattern<E, S, T2> for MapPattern<E, S, T1, T2, A, F>
where
    A: Pattern<E, S, T1>,
    S: Default,
    F: Fn(T1) -> T2,
{
    fn apply(&self, event: &E, state: &mut S) -> ParseResult<T2> {
        self.inner.apply(event, state).map(|t1| (self.f)(t1))
    }
}

pub struct FlatMapPattern<E, S1, S2, T1, T2, A, B, F>
where
    A: Pattern<E, S1, T1>,
    B: Pattern<E, S2, T2>,
    S1: Default,
    S2: Default,
    F: Fn(T1) -> B,
{
    inner: A,
    f: F,
    e: PhantomData<E>,
    st1: PhantomData<S1>,
    st2: PhantomData<S2>,
    t1: PhantomData<T1>,
    t2: PhantomData<T2>,
    b: PhantomData<B>,
}

impl<E, S1, S2, T1, T2, A, B, F> FlatMapPattern<E, S1, S2, T1, T2, A, B, F>
where
    A: Pattern<E, S1, T1>,
    B: Pattern<E, S2, T2>,
    S1: Default,
    S2: Default,
    F: Fn(T1) -> B,
{
}

impl<E, S1, S2, T1, T2, A, B, F> Pattern<E, (S1, Option<B>, S2), T2>
    for FlatMapPattern<E, S1, S2, T1, T2, A, B, F>
where
    A: Pattern<E, S1, T1>,
    B: Pattern<E, S2, T2>,
    S1: Default,
    S2: Default,
    F: Fn(T1) -> B,
{
    fn apply(&self, _event: &E, _state: &mut (S1, Option<B>, S2)) -> ParseResult<T2> {
        //        state.1 = {
        //            let (s1, opt, s2) = state;
        //            match opt {
        //                None => {
        //                    let res1 = self.inner.apply(event, s1);
        //                    match res1 {
        //                        ParseResult::Stay => ParseResult::Stay,
        //                        ParseResult::Failure { message } => ParseResult::Failure { message },
        //                        ParseResult::Success(t) => {
        //                            {
        //                                opt = Some((self.f)(t));
        //                            }
        //                            let next_result = match opt {
        //                                Some(ref mut f2) => f2.apply(event, &mut s2),
        //                                None => unimplemented!(),
        //                            };
        //                            opt
        //                        }
        //                    }
        //                }
        //                Some(ref p) => unimplemented!(),
        //            }
        //        }
        unimplemented!()
    }
}

pub trait MonadPatternTrait<E, S, T1, T2> {
    fn map<F: Fn(T1) -> T2, B>(self, f: F) -> MapPattern<E, S, T1, T2, Self, F>
    where
        Self: Pattern<E, S, T1>,
        Self: Sized,
        S: Default;

    fn flat_map<S2, B, F>(self, f: F) -> FlatMapPattern<E, S, S2, T1, T2, Self, B, F>
    where
        B: Pattern<E, S2, T2>,
        F: Fn(T1) -> B,
        Self: Sized,
        Self: Pattern<E, S, T1>,
        S: Default,
        S2: Default;
}

impl<E, S, T1, T2, L> MonadPatternTrait<E, S, T1, T2> for L
where
    L: Pattern<E, S, T1>,
    S: Default,
{
    fn map<F: Fn(T1) -> T2, B>(self, f: F) -> MapPattern<E, S, T1, T2, Self, F>
    where
        Self: Pattern<E, S, T1>,
        Self: Sized,
        S: Default,
    {
        MapPattern {
            inner: self,
            f,
            e: PhantomData,
            st1: PhantomData,
            t1: PhantomData,
            t2: PhantomData,
        }
    }

    fn flat_map<S2, B, F>(self, f: F) -> FlatMapPattern<E, S, S2, T1, T2, Self, B, F>
    where
        B: Pattern<E, S2, T2>,
        F: Fn(T1) -> B,
        Self: Sized,
        Self: Pattern<E, S, T1>,
        S: Default,
        S2: Default,
    {
        FlatMapPattern {
            inner: self,
            f: f,
            e: PhantomData,
            st1: PhantomData,
            st2: PhantomData,
            t1: PhantomData,
            t2: PhantomData,
            b: PhantomData,
        }
    }
}
