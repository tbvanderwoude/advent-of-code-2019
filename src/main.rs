use std::env;

use aoc::network::view;
use aoc::intcode::load_program;
use aoc::intcode;
//use aoc::rogue;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    println!("{}",view(intcode::load_program("data/intcode/network.txt")));
}
