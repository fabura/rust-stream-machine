#[macro_use]
extern crate lazy_static;

extern crate time;

mod stream;
mod tsp;

use crate::tsp::pattern::{PatternResult, WithIndex};
use crate::tsp::patterns::ConstantPattern;
use crate::tsp::tsp::Chunker;

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
        TestEvent::new(2, 35),
        TestEvent::new(3, 36),
        TestEvent::new(4, 37),
        TestEvent::new(5, 38),
    ];

    let state_machine =
        tsp::tsp::SimpleMachineMapper::new(ConstantPattern::new(PatternResult::Success(3)));

    let iter = state_machine.run(ints.iter().into_iter(), 10);
    {
        println!("test");
        for x in iter {
            println!("{:?}", x)
        }
    }
    //     run_rule();
}
