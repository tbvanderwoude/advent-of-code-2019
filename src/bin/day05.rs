use aoc::intcode::{load_program, run_int_code_on_computer, TestComputer};
use std::io;
use std::io::Read;

fn run_with_system_id(mem: &Vec<i64>, sys_id: i64) -> i64 {
    let mut program_clone = mem.clone();
    let mut computer = TestComputer::new(vec![sys_id].into_iter().collect());
    let mut instr_pointer: usize = 0;
    run_int_code_on_computer(&mut instr_pointer, &mut program_clone, &mut computer, false);
    *computer.output_buffer.last().unwrap()
}
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut program = load_program(input);
    let part1 = run_with_system_id(&program, 1);
    let part2 = run_with_system_id(&program, 5);
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
