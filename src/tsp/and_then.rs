use std::cmp::{max, min};
use std::marker::PhantomData;

use crate::tsp::pattern::{Idx, IdxValue, Pattern, PatternResult, PQueue, WithIndex};

#[derive(Clone)]
pub struct AndThenPattern<P1, P2> {
    first: P1,
    second: P2,
}

impl<E, P1, P2> AndThenPattern<P1, P2>
    where
        E: WithIndex,
        P1: Pattern<Event=E, T=()>,
        P2: Pattern<Event=E, T=()>,
{
    pub fn new(first: P1, second: P2) -> Self {
        AndThenPattern { first, second }
    }
}

#[derive(Default)]
pub struct AndThenPatternState<S1: Default, S2: Default> {
    first_state: S1,
    first_queue: PQueue<()>,
    second_state: S2,
    second_queue: PQueue<()>,
}

impl<E, P1, S1, P2, S2> Pattern for AndThenPattern<P1, P2>
    where
        E: WithIndex,
        S1: Default,
        S2: Default,
        P1: Pattern<Event=E, State=S1, T=(), W=Idx>,
        P2: Pattern<Event=E, State=S2, T=(), W=Idx>,
{
    type State = AndThenPatternState<S1, S2>;
    type Event = E;
    type T = ();

    fn apply(
        &self,
        event: &Vec<Self::Event>,
        queue: &mut PQueue<Self::T>,
        state: &mut Self::State,
    ) {
        self.first
            .apply(event, &mut state.first_queue, &mut state.first_state);
        self.second
            .apply(event, &mut state.second_queue, &mut state.second_state);

        let offset = self.second.width() + 1;

        // while we have results in first_queue
        while let Some(IdxValue {
                           start: first_start,
                           end: first_end,
                           result: first_result,
                       }) = state.first_queue.head_option()
        {
            let result_begin = first_start + offset;
            let result_end = first_end + offset;

            //todo should we emit Failure for the beginning of stream?
            // state.second_queue.rewind_to(dbg!(result_begin));
            let mut end = result_begin; // we store end for the last emitted result.
            loop {
                match state.second_queue.head_option() {
                    // we produce result using head of second_queue
                    Some(IdxValue {
                             start: second_start,
                             end: second_end,
                             result: second_result,
                         }) if second_start <= &result_end => {
                        if second_end < &result_begin {
                            state.second_queue.rewind_to(result_begin);
                            continue;
                        }

                        let start = max(&result_begin, second_start);
                        end = min(result_end, *second_end);

                        let result = match (first_result, second_result) {
                            (PatternResult::Success(()), PatternResult::Success(())) => PatternResult::Success(()),
                            _ => PatternResult::Failure,
                        };
                        queue.enqueue_joined(IdxValue::new(*start, end, result));
                        // update second queue
                        state.second_queue.rewind_to(end + 1);
                    }
                    None => return,
                    _ => break,
                }
            }
            // update first queue
            state.first_queue.rewind_to(end + 1 - offset);
        }
    }

    type W = Idx;

    fn width(&self) -> Self::W {
        // we add 1 here due to results of first and second must be divided by one message.
        1u64 + self.first.width() + self.second.width()
    }
}
