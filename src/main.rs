extern crate time;

mod tsp;

use crate::tsp::pattern::{PatternResult, WithIndex};
use crate::tsp::patterns::{AssertPattern, BiPattern, ConstantPattern, FunctionPattern, WindowPattern, AndThenPattern};

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

impl WithIndex for &TestEvent {
    fn index(&self) -> u64 {
        self.idx
    }
}

fn main() {
    let ints = &[
        TestEvent::new(1, 34),
        TestEvent::new(2, 34),
        TestEvent::new(3, 36),
        TestEvent::new(4, 36),
        TestEvent::new(5, 34),
    ];

    let function = FunctionPattern::new(|e: &&TestEvent| e.value);
    let constant = ConstantPattern::new(PatternResult::Success(35));
    let bi_pattern = BiPattern::new(function, constant, |a, b| a < b);
    let assert = AssertPattern::new(bi_pattern);
    let window = WindowPattern::new(assert, 2);

    let function2 = FunctionPattern::new(|e: &&TestEvent| e.value);
    let constant2 = ConstantPattern::new(PatternResult::Success(35));
    let bi_pattern2 = BiPattern::new(function2, constant2, |a, b| a < b);
    let assert2 = AssertPattern::new(bi_pattern2);
    // let window2 = WindowPattern::new(assert2, 2);
    let and_then = AndThenPattern::new( assert2, window);

    let state_machine = tsp::tsp::SimpleMachineMapper::new(and_then);
    // tsp::tsp::SimpleMachineMapper::new(constant_pattern);

    let iter = state_machine.run(ints.iter(), 10);
    {
        for x in iter {
            println!("{:?}", x)
        }
    }
    //     run_rule();
}
