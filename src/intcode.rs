



pub fn load_program(program: String) -> Vec<i64> {
    let split = program.split(",");
    let mut program: Vec<i64> = vec![];
    for s in split {
        program.push(s.to_string().parse::<i64>().unwrap());
    }
    while program.len() < 65536 {
        program.push(0);
    }
    return program;
}

pub trait Computer {
    fn input(&mut self) -> i64;
    fn output(&mut self, x: i64);
}
pub fn run_int_code_on_computer(
    i: &mut usize,
    mem: &mut Vec<i64>,
    robot: &mut dyn Computer,
    print_debug: bool,
) -> i64 {
    let mut rel_base: i64 = 0;
    while *i < mem.len() {
        let mut opcode: i64 = 0;
        let mut mode1: i64 = 0;
        let mut mode2: i64 = 0;
        let mut mode3: i64 = 0;
        if mem[*i] == 99 {
            break;
        }
        //Todo: make this into a true Vec<i64> without optionals
        let mut chars: Vec<i64> = mem[*i]
            .to_string()
            .chars()
            .map(|x| match x.to_digit(10) {
                Some(d) => return d as i64,
                None => return 0,
            })
            .collect();
        let len = chars.len();

        if len >= 1 {
            opcode = chars.pop().unwrap();
            if len >= 3 {
                chars.pop();
                mode1 = chars.pop().unwrap();
                if len >= 4 {
                    mode2 = chars.pop().unwrap();
                    if len >= 5 {
                        mode3 = chars.pop().unwrap();
                    }
                }
            }
        }
        if print_debug {
            println!(
                "Opcode {0} (mode1 {1} mode2 {2} mode3 {3} base {4})",
                opcode, mode1, mode2, mode3, rel_base
            );
        }

        match opcode {
            1 | 2 => {
                let mut arg1: i64 = mem[*i + 1];
                let mut arg2: i64 = mem[*i + 2];
                if mode1 == 0 {
                    arg1 = mem[arg1 as usize];
                } else if mode1 == 2 {
                    arg1 = mem[(rel_base + arg1) as usize];
                }
                if mode2 == 0 {
                    arg2 = mem[arg2 as usize];
                } else if mode2 == 2 {
                    arg2 = mem[(rel_base + arg2) as usize];
                }
                let index: i64 = mem[*i + 3];
                if opcode == 1 {
                    if mode3 == 2 {
                        mem[(rel_base + index) as usize] = arg1 + arg2;
                    } else if mode3 == 0 {
                        mem[index as usize] = arg1 + arg2;
                    }
                } else {
                    if mode3 == 2 {
                        mem[(rel_base + index) as usize] = arg1 * arg2;
                    } else if mode3 == 0 {
                        mem[index as usize] = arg1 * arg2;
                    }
                }
                *i += 4;
            }
            3 | 4 | 9 => {
                let mut arg1: i64 = mem[*i + 1];

                if opcode == 9 {
                    if mode1 == 0 {
                        rel_base += mem[arg1 as usize];
                    } else if mode1 == 2 {
                        rel_base += mem[(rel_base + arg1) as usize];
                    } else {
                        rel_base = rel_base + arg1;
                    }
                } else if opcode == 3 {
                    let input: i64 = robot.input();
                    if mode1 == 0 {
                        mem[arg1 as usize] = input;
                    } else if mode1 == 2 {
                        mem[(rel_base + arg1) as usize] = input;
                    }
                } else if opcode == 4 {
                    if mode1 == 0 {
                        arg1 = mem[arg1 as usize];
                    } else if mode1 == 2 {
                        arg1 = mem[(rel_base + arg1) as usize];
                    }
                    &robot.output(arg1);
                }
                *i += 2;
            }
            5 | 6 | 7 | 8 => {
                let mut arg1: i64 = mem[*i + 1];
                let mut arg2: i64 = mem[*i + 2];
                if mode1 == 0 {
                    arg1 = mem[arg1 as usize];
                } else if mode1 == 2 {
                    arg1 = mem[(arg1 + rel_base) as usize];
                }

                if mode2 == 0 {
                    arg2 = mem[arg2 as usize];
                } else if mode2 == 2 {
                    arg2 = mem[(arg2 + rel_base) as usize];
                }

                if opcode == 7 || opcode == 8 {
                    let index: i64 = mem[*i + 3];
                    if mode3 == 2 {
                        mem[(index + rel_base) as usize] =
                            (opcode == 7 && arg1 < arg2 || opcode == 8 && arg1 == arg2) as i64;
                    } else {
                        mem[index as usize] =
                            (opcode == 7 && arg1 < arg2 || opcode == 8 && arg1 == arg2) as i64;
                    }
                    *i += 4;
                } else {
                    if opcode == 5 && arg1 != 0 || opcode == 6 && arg1 == 0 {
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
