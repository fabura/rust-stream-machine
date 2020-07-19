use crate::tsp::patterns::pattern::{Idx, IdxValue, Pattern, PatternResult, PQueue, WithIndex};

#[derive(Debug, Copy, Clone)]
pub struct Window {
    size: u32,
}

#[derive(Clone)]
pub struct WindowPattern<P> {
    inner: P,
    window: Window,
}

impl<P> WindowPattern<P>
    where
        P: Pattern<T=()>,
{
    pub fn new(inner: P, size: u32) -> Self {
        assert!(size > 0);
        WindowPattern {
            inner,
            window: Window { size },
        }
    }
}

#[derive(Default)]
pub struct WindowPatternState<S: Default> {
    inner_state: S,
    inner_queue: PQueue<()>,
    last_end: Idx,
    last_success: bool,
}

impl<E, P, InnerState> Pattern for WindowPattern<P>
    where
        E: WithIndex,
        InnerState: Default,
        P: Pattern<Event=E, T=(), State=InnerState, W=Idx>,
{
    type State = WindowPatternState<InnerState>;
    type Event = E;
    type T = ();

    fn apply(&self, event: &Vec<Self::Event>, queue: &mut PQueue<()>, state: &mut Self::State) {
        // apply inner pattern to the input events
        self.inner
            .apply(event, &mut state.inner_queue, &mut state.inner_state);

        while let Some(IdxValue { start, end, result }) = state.inner_queue.dequeue_option()
        {
            assert!(state.last_end < end);
            match result {
                PatternResult::Failure => {
                    queue.enqueue_joined(IdxValue::new(
                        state.last_end + 1,
                        end,
                        PatternResult::Failure,
                    ));
                    state.last_end = end;
                    state.last_success = false;
                }
                PatternResult::Success(()) => {
                    if state.last_success {
                        queue.enqueue_joined(IdxValue::new(state.last_end + 1, end, PatternResult::Success(())));
                        state.last_end = end;
                        state.last_success = true;
                    } else {
                        let new_start = state.last_end + self.window.size as u64;
                        if new_start <= end {
                            queue.enqueue_joined(IdxValue::new(
                                new_start,
                                end,
                                PatternResult::Success(()),
                            ));
                            state.last_end = end;
                            state.last_success = true;
                        }
                    }
                }
            }
        }
    }

    type W = Idx;

    fn width(&self) -> Self::W {
        (self.window.size - 1) as u64 + self.inner.width()
    }
}
