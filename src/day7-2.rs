use permutohedron;
use permutohedron::Heap;
use std::{env, io};
use std::cmp::max;
use std::fs;

fn set_phase_of_program(phase: i32, mem: &mut Vec<i32>)
{
    let arg1: usize = mem[1] as usize;
    mem[arg1] = phase;
}

fn run_iter(mut input: i32, i: &mut usize, mem: &mut Vec<i32>) -> i32 {
    while *i < mem.len() {
        let mut opcode: i32 = 0;
        let mut mode1: i32 = 0;
        let mut mode2: i32 = 0;
        let mut mode23: i32 = 0;
        let strOp = mem[*i].to_string();
        let len = strOp.len();
        if len >= 1
        {
            opcode = strOp.parse::<i32>().unwrap();
            if len >= 3
            {
                opcode = (strOp[1..]).parse::<i32>().unwrap();
                mode1 = (strOp[..1]).parse::<i32>().unwrap();
                if len >= 4
                {
                    opcode = (strOp[2..]).parse::<i32>().unwrap();
                    mode1 = (strOp[1..2]).parse::<i32>().unwrap();
                    mode2 = (strOp[..1]).parse::<i32>().unwrap();
                }
            }
        }
        //println!("Opcode {0} (mode1 {1} mode2 {2})",opcode,mode1,mode2);
        match opcode {
            1 | 2 => {
                let mut arg1: i32 = mem[*i + 1];
                let mut arg2: i32 = mem[*i + 2];
                if mode1 == 0
                {
                    arg1 = mem[arg1 as usize];
                }
                if mode2 == 0
                {
                    arg2 = mem[arg2 as usize];
                }
                let index: i32 = mem[*i + 3];
                if opcode == 1
                {
                    mem[index as usize] = (arg1 + arg2);
                } else {
                    mem[index as usize] = (arg1 * arg2);
                }
                *i += 4;
            }
            3 | 4 => {
                let mut arg1: usize = mem[*i + 1] as usize;
                *i += 2;
                if (opcode == 3)
                {
                    mem[arg1] = input;
                }
                if (opcode == 4)
                {
                    return mem[arg1];
                }
            }
            5 | 6 | 7 | 8 =>
                {
                    let mut arg1: i32 = mem[*i + 1];
                    let mut arg2: i32 = mem[*i + 2];
                    if mode1 == 0
                    {
                        arg1 = mem[arg1 as usize];
                    }
                    if mode2 == 0
                    {
                        arg2 = mem[arg2 as usize];
                    }
                    if (opcode == 7 || opcode == 8)
                    {
                        let index: i32 = mem[*i + 3];
                        mem[index as usize] = (opcode == 7 && arg1 < arg2 || opcode == 8 && arg1 == arg2) as i32;
                        *i += 4;
                    } else {
                        if (opcode == 5 && arg1 != 0 || opcode == 6 && arg1 == 0)
                        {
                            *i = arg2 as usize;
                        } else {
                            *i += 3;
                        }
                    }
                }
            99 => break,
            _ => break,
        }
    }
    return -99;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split = contents.split(",");
    let mut program: Vec<i32> = vec![];

    for s in split {
        program.push(s.to_string().parse::<i32>().unwrap());
    }
    let mut phases: Vec<i32> = vec![5, 6, 7, 8, 9];
    let heap = Heap::new(&mut phases);
    let mut permutations = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }


    let mut max_signal: i32 = 0;
    for permutation in permutations {
        let mut programs = Vec::new();
        let mut program_counters: Vec<usize> = vec![2, 2, 2, 2, 2];
        let mut current_output = vec![0, 0, 0, 0, 0];
        for i in 0..5 {
            programs.push(program.clone());
            set_phase_of_program(permutation[i], &mut programs[i]);
        }
        let mut i: i32 = 0;
        while true
            {
                let mut previous: i32 = ((i - 1) % 5);
                if previous < 0
                {
                    previous += 5;
                }
                let output = run_iter(current_output[previous as usize], &mut program_counters[i as usize], &mut programs[i as usize]);
                if (output == -99 && i == 4)
                {
                    break;
                } else {
                    current_output[i as usize] = output;
                }
                i = (i + 1) % 5;
            }
        if current_output[4] > max_signal
        {
            max_signal = current_output[4];
        }
    }
    print!("{}", max_signal);
}
