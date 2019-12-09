use std::io;
use std::fs;

pub fn load_program(filename: &String) -> Vec<i64>
{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split = contents.split(",");
    let mut program: Vec<i64> = vec![];
    for s in split {
        program.push(s.to_string().parse::<i64>().unwrap());
    }
    while (program.len() < 100000) {
        program.push(0);
    }
    return program;
}
pub fn run_int_code_from_here(i: &mut usize, mem: &mut Vec<i64>) -> i64 {
    let mut rel_base: i64 = 0;
    while *i < mem.len() {
        let mut opcode: i64 = 0;
        let mut mode1: i64 = 0;
        let mut mode2: i64 = 0;
        let mut mode3: i64 = 0;
        let mut arg1=0;
        let mut arg2=0;
        let mut arg3=0;
        //Todo: make this into a true Vec<i64> without optionals
        let mut chars: Vec<i64> = mem[*i].to_string().chars().map(|x| x.to_digit(10).unwrap() as i64).collect();

        let len = chars.len();
        if len >= 1
        {
            opcode = chars.pop().unwrap();
            if len >= 3
            {
                chars.pop();
                mode1 = chars.pop().unwrap();
                if len >= 4
                {
                    mode2 = chars.pop().unwrap();
                    if len >= 5
                    {
                        mode3 = chars.pop().unwrap();
                    }
                }
            }
        }
        if(opcode!=99)
        {
            arg1 = mem[*i + 1];
            if(opcode!=9&&opcode!=3&&opcode!=4)
            {
                arg2 = mem[*i + 2];
                if(opcode==1||opcode==2||opcode==7||opcode==8)
                {
                    arg3 = mem[*i + 3];
                }
            }
        }
        //println!("Opcode {0} (mode1 {1} mode2 {2} mode3 {3} base {4})",opcode,mode1,mode2,mode3,rel_base);
        match opcode {
            1 | 2 => {
                if mode1 == 0
                {
                    arg1 = mem[arg1 as usize];
                } else if mode1 == 2
                {
                    arg1 = mem[(rel_base + arg1) as usize];
                }
                if mode2 == 0
                {
                    arg2 = mem[arg2 as usize];
                } else if mode2 == 2
                {
                    arg2 = mem[(rel_base + arg2) as usize];
                }
                if opcode == 1 {
                    if (mode3 == 2)
                    {
                        mem[(rel_base + arg3) as usize] = (arg1 + arg2);
                    } else if mode3 == 0
                    {
                        mem[arg3 as usize] = (arg1 + arg2);
                    }
                } else {
                    if (mode3 == 2)
                    {
                        mem[(rel_base + arg3) as usize] = (arg1 * arg2);
                    } else if mode3 == 0
                    {
                        mem[arg3 as usize] = (arg1 * arg2);
                    }
                }
                *i += 4;
            }
            3 | 4 | 9 => {
                if (opcode == 9)
                {
                    if mode1 == 0
                    {
                        rel_base += mem[arg1 as usize];
                    } else if mode1 == 2
                    {
                        rel_base += mem[(rel_base + arg1) as usize];
                    } else {
                        rel_base += arg1;
                    }
                } else if (opcode == 3)
                {
                    let mut ret = String::new();
                    io::stdin().read_line(&mut ret).unwrap();
                    let input: i64 = ret.trim().to_string().parse::<i64>().unwrap();
                    if mode1 == 0
                    {
                        mem[arg1 as usize] = input;
                    } else if mode1 == 2
                    {
                        mem[(rel_base + arg1) as usize] = input;
                    }
                } else if (opcode == 4)
                {
                    if mode1 == 0
                    {
                        arg1 = mem[arg1 as usize];
                    } else if mode1 == 2
                    {
                        arg1 = mem[(rel_base + arg1) as usize];
                    }
                    println!("{}", arg1);
                }
                *i += 2;
            }
            5 | 6 | 7 | 8 =>
                {
                    let mut arg1: i64 = mem[*i + 1];
                    let mut arg2: i64 = mem[*i + 2];
                    if mode1 == 0
                    {
                        arg1 = mem[arg1 as usize];
                    } else if mode1 == 2
                    {
                        arg1 = mem[(arg1 + rel_base) as usize];
                    }

                    if mode2 == 0
                    {
                        arg2 = mem[arg2 as usize];
                    } else if mode2 == 2
                    {
                        arg2 = mem[(arg2 + rel_base) as usize];
                    }

                    if (opcode == 7 || opcode == 8)
                    {
                        if mode3 == 2
                        {
                            mem[(arg3 + rel_base) as usize] = (opcode == 7 && arg1 < arg2 || opcode == 8 && arg1 == arg2) as i64;
                        } else {
                            mem[arg3 as usize] = (opcode == 7 && arg1 < arg2 || opcode == 8 && arg1 == arg2) as i64;
                        }
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