use std::env;

use aoc::network::view;
use aoc::intcode::load_program;
use aoc::intcode;
use aoc::cryobot::run_program;
//use aoc::rogue;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    run_program(&"data/intcode/cryobot.txt".to_string());
}
