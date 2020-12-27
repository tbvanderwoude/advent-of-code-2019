use aoc::intcode::{load_program, run_int_code_on_computer, InteractiveComputer};
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let program = load_program(input);
    let mut program_clone = program.clone();
    let mut computer = InteractiveComputer {};
    let mut instr_pointer: usize = 0;
    program_clone[1] = 12;
    program_clone[2] = 2;
    run_int_code_on_computer(&mut instr_pointer, &mut program_clone, &mut computer, false);
    let part1 = program_clone[0];
    let mut part2 = -1;
    for noun in 0..100 {
        for verb in 0..100 {
            instr_pointer = 0;
            let mut cln = program.clone();
            let mut comp = InteractiveComputer {};
            cln[1] = noun;
            cln[2] = verb;
            run_int_code_on_computer(&mut instr_pointer, &mut cln, &mut comp, false);
            if cln[0] == 19690720 {
                part2 = 100 * noun + verb;
                break;
            }
        }
        if part1 == -1 {
            break;
        }
    }
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
