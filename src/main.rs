use std::{env, io};
use std::cmp::max;
mod intcode;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()!=2
    {
        return;
    }
    let filename=&args[1];
    let mut program=intcode::load_program(filename);
    let mut program_counter:usize=0;
    intcode::run_int_code_from_here(&mut program_counter,&mut program);
}
