use std::{env, io};
use std::cmp::max;
use std::fs;
use std::collections::HashMap;
use crate::intcode::DefaultComputer;

mod intcode;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()!=2
    {
        return;
    }
    let filename=&args[1];
    let mut program=intcode::load_program(filename);
    let mut counter: usize = 0;
    let mut robot: intcode::Robot = intcode::Robot{map: HashMap::new(),dir: 0,paint:true,x:0,y:0};
    let mut computer: intcode::DefaultComputer = DefaultComputer{};
    println!("{}",intcode::run_int_code_from_here_custom(&mut counter,&mut program, &mut computer));
    //println!("{}",robot.map.len());
    //Insight: order does not matter until the last one.
}
