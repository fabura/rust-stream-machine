extern crate time;

use crate::tsp::patterns::and_then::AndThenPattern;
use crate::tsp::patterns::assert::AssertPattern;
use crate::tsp::patterns::bi_pattern::BiPattern;
use crate::tsp::patterns::constant::ConstantPattern;
use crate::tsp::patterns::function::FunctionPattern;
use crate::tsp::patterns::pattern::{PatternResult, WithIndex};
use crate::tsp::patterns::window::WindowPattern;
use crate::tsp::query::*;

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
    let window = WindowPattern::new(assert.clone(), 2);

    let and_then = AndThenPattern::new(assert.clone(), window);

    let state_machine = tsp::query::SimpleMachineMapper::new(and_then);
    // tsp::tsp::SimpleMachineMapper::new(constant_pattern);

    let iter = state_machine.run(ints.iter(), 10);
    {
        for x in iter {
            println!("{:?}", x)
        }
    }
    //     run_rule();
}
