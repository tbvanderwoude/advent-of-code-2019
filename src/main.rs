use std::env;

use aoc::rogue;
use std::error::Error;
use aoc::spring_droid::run_program;

//use aoc::rogue;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let filename: String = args[1].as_str().parse().unwrap();
    env::set_var("RUST_BACKTRACE", "1");

    run_program(&filename,"NOT C J\nAND H J\nNOT B T\nOR T J\nNOT A T\nOR T J\nAND D J\nRUN\n");
}
