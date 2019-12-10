use std::{env, io};
use std::cmp::max;
use std::fs;
mod asteroid_blaster;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()!=2
    {
        return;
    }
    let filename=&args[1];
    let mut asteroids=asteroid_blaster::load_asteroids(filename);
    println!("{}",asteroid_blaster::compute_two_hundreth_coord(asteroids));
    //Insight: order does not matter until the last one.
}
