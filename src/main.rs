// #![deny(warnings, /*missing_docs*/)]

extern crate time;

use crate::tsp::partitioners::partitioner::FunctionPartitioner;
use crate::tsp::partitioners::*;
use crate::tsp::patterns::*;
use crate::tsp::projections::*;

// use crate::tsp::query::*;

mod tsp;

#[derive(Debug)]
struct TestEvent {
    idx: u64,
    value: u64,
}

impl TestEvent {
    pub fn new(idx: u64, value: u64) -> Self {
        TestEvent { idx, value }
    }
}

#[allow(dead_code)]
fn main() {
    let ints = &[
        TestEvent::new(0, 33),
        TestEvent::new(1, 34),
        TestEvent::new(2, 34),
        TestEvent::new(3, 36),
        TestEvent::new(4, 36),
        TestEvent::new(5, 34),
    ];

    let function = FunctionPattern::new(|e: &&TestEvent| e.idx);
    let constant = ConstantPattern::new(PatternResult::Success(35));
    let bi_pattern = BiPattern::new(function.clone(), constant, |a, b| a < b);
    let assert = AssertPattern::new(bi_pattern);
    let window = WindowPattern::new(assert.clone(), 2);

    let _and_then = AndThenPattern::new(assert.clone(), window);

    let projection = FirstProjection::new(|e: &&TestEvent| e.value);
    let state_machine = tsp::query::SimpleMachineMapper::new(
        projection,
        function.clone(),
        FunctionPartitioner::new(|e: &TestEvent| e.idx),
    );
    // tsp::tsp::SimpleMachineMapper::new(constant_pattern);

    let iter = state_machine.run(ints.iter(), 10);
    {
        for x in iter {
            println!("{:?}", x)
        }
    }
}
