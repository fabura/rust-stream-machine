#[macro_use]
extern crate lazy_static;

extern crate time;

mod stream;
use stream::stream_machine::*;

fn main() {
    run_rule();
}
