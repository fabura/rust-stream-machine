use std::cmp::{Ordering, PartialOrd};
use std::marker::PhantomData;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Sub};
use stream::stream_machine::*;

pub trait Binary<Event, State, T, RightT> {
    fn combine<R, RightState, T3, F: Fn(T, RightT) -> T3 + 'static>(
        self,
        rhs: R,
        f: F,
    ) -> BinaryPattern<Event, State, RightState, T, RightT, T3, Self, R>
    where
        R: StreamPattern<Event, RightState, RightT>,
        State: Default,
        RightState: Default,
        Self: Sized,
        Self: StreamPattern<Event, State, T>;

    fn plus<R, RightState, T3>(
        self,
        rhs: R,
    ) -> BinaryPattern<Event, State, RightState, T, RightT, T3, Self, R>
    where
        T: Add<RightT, Output = T3>,
        R: StreamPattern<Event, RightState, RightT>,
        State: Default,
        RightState: Default,
        Self: Sized,
        Self: StreamPattern<Event, State, T>,
    {
        self.combine(rhs, |t1, t2| t1 + t2)
    }

    fn sub<R, RightState, T3>(
        self,
        rhs: R,
    ) -> BinaryPattern<Event, State, RightState, T, RightT, T3, Self, R>
    where
        T: Sub<RightT, Output = T3>,
        R: StreamPattern<Event, RightState, RightT>,
        State: Default,
        RightState: Default,
        Self: Sized,
        Self: StreamPattern<Event, State, T>,
    {
        self.combine(rhs, |t1, t2| t1 - t2)
    }

    fn mul<R, RightState, T3>(
        self,
        rhs: R,
    ) -> BinaryPattern<Event, State, RightState, T, RightT, T3, Self, R>
    where
        T: Mul<RightT, Output = T3>,
        R: StreamPattern<Event, RightState, RightT>,
        State: Default,
        RightState: Default,
        Self: Sized,
        Self: StreamPattern<Event, State, T>,
    {
        self.combine(rhs, |t1, t2| t1 * t2)
    }

    fn div<R, RightState, T3>(
        self,
        rhs: R,
    ) -> BinaryPattern<Event, State, RightState, T, RightT, T3, Self, R>
    where
        T: Div<RightT, Output = T3>,
        R: StreamPattern<Event, RightState, RightT>,
        State: Default,
        RightState: Default,
        Self: Sized,
        Self: StreamPattern<Event, State, T>,
    {
        self.combine(rhs, |t1, t2| t1 / t2)
    }

    fn rem<R, RightState, T3>(
        self,
        rhs: R,
    ) -> BinaryPattern<Event, State, RightState, T, RightT, T3, Self, R>
    where
        T: Rem<RightT, Output = T3>,
        R: StreamPattern<Event, RightState, RightT>,
        State: Default,
        RightState: Default,
        Self: Sized,
        Self: StreamPattern<Event, State, T>,
    {
        self.combine(rhs, |t1, t2| t1 % t2)
    }

    fn and<R, RightState, T3>(
        self,
        rhs: R,
    ) -> BinaryPattern<Event, State, RightState, T, RightT, T3, Self, R>
    where
        T: BitAnd<RightT, Output = T3>,
        R: StreamPattern<Event, RightState, RightT>,
        State: Default,
        RightState: Default,
        Self: Sized,
        Self: StreamPattern<Event, State, T>,
    {
        self.combine(rhs, |t1, t2| t1 & t2)
    }

    fn or<R, RightState, T3>(
        self,
        rhs: R,
    ) -> BinaryPattern<Event, State, RightState, T, RightT, T3, Self, R>
    where
        T: BitOr<RightT, Output = T3>,
        R: StreamPattern<Event, RightState, RightT>,
        State: Default,
        RightState: Default,
        Self: Sized,
        Self: StreamPattern<Event, State, T>,
    {
        self.combine(rhs, |t1, t2| t1 | t2)
    }

    fn xor<R, RightState, T3>(
        self,
        rhs: R,
    ) -> BinaryPattern<Event, State, RightState, T, RightT, T3, Self, R>
    where
        T: BitXor<RightT, Output = T3>,
        R: StreamPattern<Event, RightState, RightT>,
        State: Default,
        RightState: Default,
        Self: Sized,
        Self: StreamPattern<Event, State, T>,
    {
        self.combine(rhs, |t1, t2| t1 ^ t2)
    }
}

impl<Event, State, T, RightT, L> Binary<Event, State, T, RightT> for L
where
    L: StreamPattern<Event, State, T>,
    State: Default,
{
    fn combine<R, RightState, T3, F: Fn(T, RightT) -> T3 + 'static>(
        self,
        rhs: R,
        f: F,
    ) -> BinaryPattern<Event, State, RightState, T, RightT, T3, Self, R>
    where
        R: StreamPattern<Event, RightState, RightT>,
        State: Default,
        RightState: Default,
        Self: Sized,
        Self: StreamPattern<Event, State, T>,
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

pub struct BinaryPattern<Event, State1, State2, T1, T2, T3, A, B>
where
    A: StreamPattern<Event, State1, T1>,
    B: StreamPattern<Event, State2, T2>,
    State1: Default,
    State2: Default,
{
    left: A,
    right: B,
    f: Box<Fn(T1, T2) -> T3>,
    e: PhantomData<Event>,
    st1: PhantomData<State1>,
    st2: PhantomData<State2>,
    t1: PhantomData<T1>,
    t2: PhantomData<T2>,
}

impl<Event, State1, State2, T1, T2, T3, A, B> StreamPattern<Event, (State1, State2), T3>
    for BinaryPattern<Event, State1, State2, T1, T2, T3, A, B>
where
    State1: Default,
    State2: Default,
    A: StreamPattern<Event, State1, T1>,
    B: StreamPattern<Event, State2, T2>,
{
    fn apply(&self, event: &Event, state: &mut (State1, State2)) -> ParseResult<T3> {
        let (state_l, state_r) = state;
        let result_l = self.left.apply(event, state_l);
        let result_r = self.right.apply(event, state_r);
        result_l.flat_map(move |t1| result_r.map(move |t2| (self.f)(t1, t2)))
    }
}
