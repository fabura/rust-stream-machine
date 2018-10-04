use std::cmp::PartialOrd;
use std::marker::PhantomData;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Sub};
use stream::pattern::*;

pub trait Binary<E, S1, T1, T2> {
    fn combine<R, S2, T3, F: Fn(T1, T2) -> T3 + 'static>(
        self,
        rhs: R,
        f: F,
    ) -> BinaryPattern<E, S1, S2, T1, T2, T3, Self, R>
        where
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>;

    fn plus<R, S2, T3>(self, rhs: R) -> BinaryPattern<E, S1, S2, T1, T2, T3, Self, R>
        where
            T1: Add<T2, Output=T3>,
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>,
    {
        self.combine(rhs, |t1, t2| t1 + t2)
    }

    fn sub<R, S2, T3>(self, rhs: R) -> BinaryPattern<E, S1, S2, T1, T2, T3, Self, R>
        where
            T1: Sub<T2, Output=T3>,
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>,
    {
        self.combine(rhs, |t1, t2| t1 - t2)
    }

    fn mul<R, S2, T3>(self, rhs: R) -> BinaryPattern<E, S1, S2, T1, T2, T3, Self, R>
        where
            T1: Mul<T2, Output=T3>,
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>,
    {
        self.combine(rhs, |t1, t2| t1 * t2)
    }

    fn div<R, S2, T3>(self, rhs: R) -> BinaryPattern<E, S1, S2, T1, T2, T3, Self, R>
        where
            T1: Div<T2, Output=T3>,
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>,
    {
        self.combine(rhs, |t1, t2| t1 / t2)
    }

    fn rem<R, S2, T3>(self, rhs: R) -> BinaryPattern<E, S1, S2, T1, T2, T3, Self, R>
        where
            T1: Rem<T2, Output=T3>,
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>,
    {
        self.combine(rhs, |t1, t2| t1 % t2)
    }

    fn and<R, S2, T3>(self, rhs: R) -> BinaryPattern<E, S1, S2, T1, T2, T3, Self, R>
        where
            T1: BitAnd<T2, Output=T3>,
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>,
    {
        self.combine(rhs, |t1, t2| t1 & t2)
    }

    fn or<R, S2, T3>(self, rhs: R) -> BinaryPattern<E, S1, S2, T1, T2, T3, Self, R>
        where
            T1: BitOr<T2, Output=T3>,
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>,
    {
        self.combine(rhs, |t1, t2| t1 | t2)
    }

    fn xor<R, S2, T3>(self, rhs: R) -> BinaryPattern<E, S1, S2, T1, T2, T3, Self, R>
        where
            T1: BitXor<T2, Output=T3>,
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>,
    {
        self.combine(rhs, |t1, t2| t1 ^ t2)
    }

    fn less<R, S2>(self, rhs: R) -> BinaryPattern<E, S1, S2, T1, T2, bool, Self, R>
        where
            T1: PartialOrd<T2>,
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>,
    {
        self.combine(rhs, |t1, t2| t1 < t2)
    }

    fn more<R, S2>(self, rhs: R) -> BinaryPattern<E, S1, S2, T1, T2, bool, Self, R>
        where
            T1: PartialOrd<T2>,
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>,
    {
        self.combine(rhs, |t1, t2| t1 > t2)
    }

    fn eq<R, S2>(self, rhs: R) -> BinaryPattern<E, S1, S2, T1, T2, bool, Self, R>
        where
            T1: PartialEq<T2>,
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>,
    {
        self.combine(rhs, |t1, t2| t1 == t2)
    }

    fn ne<R, S2>(self, rhs: R) -> BinaryPattern<E, S1, S2, T1, T2, bool, Self, R>
        where
            T1: PartialEq<T2>,
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>,
    {
        self.combine(rhs, |t1, t2| t1 != t2)
    }
}

impl<E, S1, T1, T2, L> Binary<E, S1, T1, T2> for L
    where
        L: Pattern<E, S1, T1>,
        S1: Default,
{
    fn combine<R, S2, T3, F: Fn(T1, T2) -> T3 + 'static>(
        self,
        rhs: R,
        f: F,
    ) -> BinaryPattern<E, S1, S2, T1, T2, T3, Self, R>
        where
            R: Pattern<E, S2, T2>,
            S1: Default,
            S2: Default,
            Self: Sized,
            Self: Pattern<E, S1, T1>,
    {
        BinaryPattern {
            left: self,
            right: rhs,
            f: Box::new(f),
            e: PhantomData,
            st1: PhantomData,
            st2: PhantomData,
            t1: PhantomData,
            t2: PhantomData,
        }
    }
}

pub struct BinaryPattern<E, S1, S2, T1, T2, T3, A, B>
    where
        A: Pattern<E, S1, T1>,
        B: Pattern<E, S2, T2>,
        S1: Default,
        S2: Default,
{
    left: A,
    right: B,
    f: Box<Fn(T1, T2) -> T3>,
    e: PhantomData<E>,
    st1: PhantomData<S1>,
    st2: PhantomData<S2>,
    t1: PhantomData<T1>,
    t2: PhantomData<T2>,
}

impl<E, S1, S2, T1, T2, T3, A, B> Pattern<E, (S1, S2), T3>
for BinaryPattern<E, S1, S2, T1, T2, T3, A, B>
    where
        S1: Default,
        S2: Default,
        A: Pattern<E, S1, T1>,
        B: Pattern<E, S2, T2>,
{
    fn apply(&self, event: &E, state: &mut (S1, S2)) -> ParseResult<T3> {
        let (state_l, state_r) = state;
        let result_l = self.left.apply(event, state_l);
        let result_r = self.right.apply(event, state_r);
        result_l.flat_map(move |t1| result_r.map(move |t2| (self.f)(t1, t2)))
    }
}
