use std::{env, io};
use std::fs;
use permutohedron;
use permutohedron::Heap;
use std::cmp::max;

fn set_phase_of_program(phase:i32, mem: &mut Vec<String>)
{

}
fn run_str_program(mut inputs: (i32,i32), mem: &mut Vec<String>) -> i32 {
    let mut i:usize = 0;
    let mut inputJat=0;
    while i < mem.len() {
        let len=mem[i].len();
        let mut opcode:i32=0;
        let mut mode1:i32=0;
        let mut mode2:i32=0;
        let mut mode23:i32=0;
        if len>=1
        {
            opcode=(mem[i]).parse::<i32>().unwrap();
            if len>=3
            {
                opcode=(&mem[i][1..]).parse::<i32>().unwrap();
                mode1=(&mem[i][..1]).parse::<i32>().unwrap();
                if len>=4
                {
                    opcode=(&mem[i][2..]).parse::<i32>().unwrap();
                    mode1=(&mem[i][1..2]).parse::<i32>().unwrap();
                    mode2=(&mem[i][..1]).parse::<i32>().unwrap();
                }
            }
        }
        //println!("Opcode {0} (mode1 {1} mode2 {2})",opcode,mode1,mode2);
        match opcode {
            1|2 => {
                let mut arg1:i32=mem[i+1].parse::<i32>().unwrap();
                let mut arg2:i32=mem[i+2].parse::<i32>().unwrap();
                if mode1==0
                {
                    arg1 = mem[arg1 as usize].parse::<i32>().unwrap();
                }
                if mode2==0
                {
                    arg2 = mem[arg2 as usize].parse::<i32>().unwrap();
                }
                let index:i32 = mem[i+3].parse::<i32>().unwrap();
                if opcode==1
                {
                    mem[index as usize]=(arg1+arg2).to_string();
                }
                else {
                    mem[index as usize]=(arg1*arg2).to_string();
                }
                i+=4;
            },
            3|4 => {
                let mut arg1:usize=mem[i+1].parse::<i32>().unwrap() as usize;
                if(opcode==3)
                {
                    /*
                    let mut ret = String::new();
                    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
                    ret=ret.trim().to_string();
                    println!("Writing {0} to {1}",ret,arg1);
                    */
                    if(inputJat==0)
                    {
                        mem[arg1]=inputs.1.to_string();
                    }
                    else
                    {
                        mem[arg1]=inputs.0.to_string();
                    }
                    inputJat+=1;
                }
                if(opcode==4)
                {
                    return mem[arg1].parse::<i32>().unwrap();
                    //println!("{}",mem[arg1])
                }
                i+=2;
            },
            5|6|7|8 =>
                {
                    let mut arg1:i32=mem[i+1].parse::<i32>().unwrap();
                    let mut arg2:i32=mem[i+2].parse::<i32>().unwrap();
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
                        let index:i32 = mem[i+3].parse::<i32>().unwrap();
                        mem[index as usize]=((opcode==7&&arg1<arg2||opcode==8&&arg1==arg2)as i32).to_string();
                        i+=4;
                    }
                    else
                    {
                        if(opcode==5&&arg1!=0||opcode==6&&arg1==0)
                        {
                            i=arg2 as usize;
                        }
                        else {
                            i+=3;
                        }
                    }
                }
            99 => break,
            _ => {i+=1},
        }
    }
    return -1;
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
    let mut phases:Vec<i32> = vec![0,1,2,3,4];
    //    let mut phases:Vec<i32> = vec![5,6,7,8,9];
    let heap = Heap::new(&mut phases);

    let mut permutations = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }
    let mut programs = Vec::new();
    for i in 0..5 {
        programs.push(program.clone());
    }
    let mut maxSignal:i32=0;
    for permutation in permutations {
        let output0:i32=run_str_program((0,permutation[0]),&mut programs[0]);
        let output1:i32=run_str_program((output0,permutation[1]),&mut programs[1]);
        let output2:i32=run_str_program((output1,permutation[2]),&mut programs[2]);
        let output3:i32=run_str_program((output2,permutation[3]),&mut programs[3]);
        let output4:i32=run_str_program((output3,permutation[4]),&mut programs[4]);
        if output4>maxSignal
        {
            maxSignal=output4;
        }
    }
    print!("{}",maxSignal);
}
