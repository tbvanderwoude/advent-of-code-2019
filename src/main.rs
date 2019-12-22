use std::env;

use aoc::rogue;
use std::error::Error;
use aoc::space_cards::unshuffle_index;

//use aoc::rogue;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let filename: String = args[1].as_str().parse().unwrap();
    env::set_var("RUST_BACKTRACE", "1");
    unshuffle_index("data/small_shuffle.txt",10,0,101741582076661,true);
}
