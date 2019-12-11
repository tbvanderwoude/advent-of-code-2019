use std::{env, io};
use std::cmp::max;
use std::fs;
use std::collections::HashMap;

mod intcode;
mod robot;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()!=2
    {
        return;
    }
    env::set_var("RUST_BACKTRACE","1");
    let filename=&args[1];
    let mut program=intcode::load_program(filename);
    robot::paint_using_robot(program);
}
