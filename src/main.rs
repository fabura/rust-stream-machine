extern crate time;

mod stream;
mod tsp;

use crate::tsp::pattern::{PatternResult, WithIndex};
use crate::tsp::patterns::{BiPattern, ConstantPattern, FunctionPattern};

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
        TestEvent::new(4, 34),
        TestEvent::new(5, 34),
    ];

    let function = FunctionPattern::new(|e: &&TestEvent| e.value);
    let constant = ConstantPattern::new(PatternResult::Success(35));
    let bi_pattern = BiPattern::new(function, constant, |a, b| a > b);

    let state_machine = tsp::tsp::SimpleMachineMapper::new(bi_pattern);
    // tsp::tsp::SimpleMachineMapper::new(constant_pattern);

    let iter = state_machine.run(ints.iter().into_iter(), 10);
    {
        println!("test");
        for x in iter {
            println!("{:?}", x)
        }
    }
    //     run_rule();
}
