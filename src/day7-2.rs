use std::{env, io};
use std::fs;
use permutohedron;
use permutohedron::Heap;
use std::cmp::max;

fn set_phase_of_program(phase:i32, mem: &mut Vec<String>)
{
    let mut arg1:usize=mem[1].parse::<i32>().unwrap() as usize;
    mem[arg1]=phase.to_string();
}
fn run_iter(mut input:i32, i: &mut usize, mem: &mut Vec<String>) -> i32 {
    while *i < mem.len() {
        let len=mem[*i].len();
        let mut opcode:i32=0;
        let mut mode1:i32=0;
        let mut mode2:i32=0;
        let mut mode23:i32=0;
        if len>=1
        {
            opcode=(mem[*i]).parse::<i32>().unwrap();
            if len>=3
            {
                opcode=(&mem[*i][1..]).parse::<i32>().unwrap();
                mode1=(&mem[*i][..1]).parse::<i32>().unwrap();
                if len>=4
                {
                    opcode=(&mem[*i][2..]).parse::<i32>().unwrap();
                    mode1=(&mem[*i][1..2]).parse::<i32>().unwrap();
                    mode2=(&mem[*i][..1]).parse::<i32>().unwrap();
                }
            }
        }
        //println!("Opcode {0} (mode1 {1} mode2 {2})",opcode,mode1,mode2);
        match opcode {
            1|2 => {
                let mut arg1:i32=mem[*i+1].parse::<i32>().unwrap();
                let mut arg2:i32=mem[*i+2].parse::<i32>().unwrap();
                if mode1==0
                {
                    arg1 = mem[arg1 as usize].parse::<i32>().unwrap();
                }
                if mode2==0
                {
                    arg2 = mem[arg2 as usize].parse::<i32>().unwrap();
                }
                let index:i32 = mem[*i+3].parse::<i32>().unwrap();
                if opcode==1
                {
                    mem[index as usize]=(arg1+arg2).to_string();
                }
                else {
                    mem[index as usize]=(arg1*arg2).to_string();
                }
                *i+=4;
            },
            3|4 => {
                let mut arg1:usize=mem[*i+1].parse::<i32>().unwrap() as usize;
                if(opcode==3)
                {
                    *i+=2;
                    mem[arg1]=input.to_string();
                }
                if(opcode==4)
                {
                    *i+=2;
                    return mem[arg1].parse::<i32>().unwrap();
                    //println!("{}",mem[arg1])
                }
            },
            5|6|7|8 =>
                {
                    let mut arg1:i32=mem[*i+1].parse::<i32>().unwrap();
                    let mut arg2:i32=mem[*i+2].parse::<i32>().unwrap();
                    if mode1==0
                    {
                        arg1 = mem[arg1 as usize].parse::<i32>().unwrap();
                    }
                    if mode2==0
                    {
                        arg2 = mem[arg2 as usize].parse::<i32>().unwrap();
                    }
                    if(opcode==7||opcode==8)
                    {
                        let index:i32 = mem[*i+3].parse::<i32>().unwrap();
                        mem[index as usize]=((opcode==7&&arg1<arg2||opcode==8&&arg1==arg2)as i32).to_string();
                        *i+=4;
                    }
                    else
                    {
                        if(opcode==5&&arg1!=0||opcode==6&&arg1==0)
                        {
                            *i=arg2 as usize;
                        }
                        else {
                            *i+=3;
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
    if args.len()!=2
    {
        return;
    }
    let filename=&args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split = contents.split(",");
    let mut program: Vec<String> = vec![];

    for s in split {
        program.push(s.to_string());
    }
    let mut phases:Vec<i32> = vec![5,6,7,8,9];
    let heap = Heap::new(&mut phases);
    let mut permutations = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }


    let mut max_signal:i32=0;
    for permutation in permutations {
        let mut programs = Vec::new();
        let mut program_counters:Vec<usize> = vec![2,2,2,2,2];
        let mut current_output = vec![0,0,0,0,0];
        for i in 0..5 {
            programs.push(program.clone());
            set_phase_of_program(permutation[i],&mut programs[i]);
        }
        while true
            {
                current_output[0]=run_iter(current_output[4],&mut program_counters[0],&mut programs[0]);
                current_output[1]=run_iter(current_output[0],&mut program_counters[1],&mut programs[1]);
                current_output[2]=run_iter(current_output[1],&mut program_counters[2],&mut programs[2]);
                current_output[3]=run_iter(current_output[2],&mut program_counters[3],&mut programs[3]);
                let mut output:i32=run_iter(current_output[3],&mut program_counters[4],&mut programs[4]);
                if(output==-99)
                {
                    break;
                }
                else {
                    current_output[4]=output;
                }
            }
        if current_output[4]>max_signal
        {
            max_signal=current_output[4];
        }
    }
    print!("{}",max_signal);
}
