use crate::tsp::pattern::{PatternResult, Pattern, PQueue, Idx, WithIndex, IdxValue};

#[derive(Debug, Default)]
pub struct NoState;

//todo do we need static here?
struct ConstantPattern<T:Clone > {
    value: PatternResult<T>
}

impl<E: WithIndex, T:Clone> Pattern<E, NoState, T> for ConstantPattern<T> {
    fn apply(&self, event: &Vec<E>, queue: &mut PQueue<T>, _state: &mut NoState) -> bool {
        //todo change it to return intervals with the same value
        queue.enqueue(event.into_iter().map(|e| {
            let idx = e.index();
            IdxValue::new(idx, idx, self.value.clone())// todo is it safe to clone?
        }));

        // we never change _state for ConstantPattern
        false
    }

    type W = Idx;

    fn width(&self) -> Self::W {
        1u64
    }
}