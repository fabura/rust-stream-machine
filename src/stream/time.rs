use time::Timespec;
use time::Duration;
use crate::stream::pattern::*;
use std::marker::PhantomData;

pub trait TimeExtractor<E> {
    fn timestamp(self) -> Timespec;
}

pub trait AndThen<E, S1, T1> {
    fn and_then<R, S2, T2>(self, rhs: R) -> AndThenPattern<E, S1, S2, T1, T2, Self, R>
        where
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>, {
        AndThenPattern {
            first: self,
            second: rhs,
            e: PhantomData,
            st1: PhantomData,
            st2: PhantomData,
            t1: PhantomData,
            t2: PhantomData,
        }
    }
}

pub struct AndThenPattern<E, S1, S2, T1, T2, A, B>
    where
        A: Pattern<E, S1, T1>,
        B: Pattern<E, S2, T2>,
        S1: Default,
        S2: Default {
    first: A,
    second: B,
    e: PhantomData<E>,
    st1: PhantomData<S1>,
    st2: PhantomData<S2>,
    t1: PhantomData<T1>,
    t2: PhantomData<T2>,
}


impl<E, S1, S2, T1, T2, A, B> Pattern<E, (Option<T1>, S1, S2), (T1, T2)>
for AndThenPattern<E, S1, S2, T1, T2, A, B>
    where
        S1: Default,
        S2: Default,
        A: Pattern<E, S1, T1>,
        B: Pattern<E, S2, T2>,
{
    fn apply(&self, event: &E, state: &mut (Option<T1>, S1, S2)) -> ParseResult<(T1, T2)> {
        let (opt, s1, s2) = state;

        match opt {
            None => {
                let r = self.first.apply(event, s1);
                let r_final = match r {
                    ParseResult::Failure { message } => ParseResult::Failure { message },
                    ParseResult::Stay => ParseResult::Stay,
                    ParseResult::Success(t1) => {
                        // ParseResult::Success(t1)
ParseResult::Stay
                    }
                };
                r_final
            }
            Some(_) => ParseResult::Stay , // todo fix it later
        }
    }
}