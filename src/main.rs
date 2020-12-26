mod async_intcode;

use std::env;
use aoc::cryobot::run_program;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    run_program("input/intcode/cryobot.txt");
}
