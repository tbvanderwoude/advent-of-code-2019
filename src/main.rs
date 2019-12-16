use aoc::fft;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let filename: String = args[1].as_str().parse().unwrap();
    env::set_var("RUST_BACKTRACE", "1");

    let mut signal: Vec<i32> = aoc::fft::read_signal(filename);
    //We are essentially taking frequency bands skip-skip+7
    aoc::fft::sol_1(signal.clone());
    aoc::fft::sol_2(signal.clone());
}
