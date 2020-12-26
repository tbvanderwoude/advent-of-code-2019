use std::io;
use std::io::Read;

extern crate rand;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

use console::Term;

use aoc::intcode::{load_program, ChannelComputer, run_int_code_on_computer};

pub struct Camera {
    in_channel: Receiver<i64>,
    out_channel: Sender<i64>,
    headless: bool,
    term: console::Term,
    buffer: Vec<Vec<char>>,
}

impl Camera {
    fn lookup(&self, x: i32, y: i32) -> bool {
        if (y < 0 || y >= self.buffer.len() as i32)
            || (x < 0 || x >= self.buffer[y as usize].len() as i32)
            || (self.buffer[y as usize][x as usize] == '.')
        {
            return false;
        }
        return true;
    }

    fn command(&mut self) -> i64 {
        let mut i = 0;
        //Greedy algorithm for general solution?
        let prog = "A,B,A,B,A,C,B,C,A,C\n";
        let sub_a = "R,4,L,10,L,10\n";
        let sub_b = "L,8,R,12,R,10,R,4\n";
        let sub_c = "L,8,L,8,R,10,R,4\n";
        self.upload_string(prog);
        self.upload_string(sub_a);
        self.upload_string(sub_b);
        self.upload_string(sub_c);
        self.upload_string("n\n");

        self.buffer.push(vec![]);
        let mut stream_buffer: Vec<char> = vec![];
        let mut total_dust = 0;
        loop {
            let res = self.in_channel.recv_timeout(Duration::from_millis(500));
            if res.is_ok() {
                if res.unwrap() > 255 {
                    total_dust = res.unwrap();
                } else {
                    let info = (res.unwrap() as u8) as char;
                    if info == '\n' {
                        if !self.headless{
                            println!("{}", stream_buffer.iter().collect::<String>());
                        }
                        stream_buffer.clear();
                    } else {
                        stream_buffer.push(info);
                    }
                }
            } else {
                break;
            }
            if i % 10000 == 0 {}
            i += 1;
        }
        total_dust
    }
    fn upload_string(&self, string: &str) {
        for c in string.chars() {
            self.out_channel.send(c as i64);
        }
    }
    fn explore(&mut self) -> usize{
        let mut i = 0;
        let mut final_total = 0;
        self.buffer.push(vec![]);
        loop {
            let res = self.in_channel.recv();
            if res.is_ok() {
                let info = (res.unwrap() as u8) as char;
                if info == '\n' {
                    self.buffer.push(vec![]);
                    if !self.headless{
                        self.render();
                    }

                    let mut total = 0;
                    for y in 0..self.buffer.len() {
                        for x in 0..self.buffer[y].len() {
                            if self.buffer[y as usize][x as usize] != '.' {
                                let mut neighbours = 0;
                                if self.lookup(x as i32 - 1, y as i32) {
                                    neighbours += 1;
                                }
                                if self.lookup(x as i32 + 1, y as i32) {
                                    neighbours += 1;
                                }
                                if self.lookup(x as i32, y as i32 + 1) {
                                    neighbours += 1;
                                }
                                if self.lookup(x as i32, y as i32 - 1) {
                                    neighbours += 1;
                                }
                                if neighbours > 2 {
                                    total += x * y;
                                }
                            }
                        }
                    }
                    final_total = total;
                } else {
                    let n = self.buffer.len() - 1;
                    self.buffer[n].push(info);
                }
            }
            else{
                break;
            }
            i += 1;
        }
        final_total
    }
    fn render(&self) {
        self.term.clear_screen();
        for line in &self.buffer {
            println!("{}", line.iter().collect::<String>());
        }
    }
}

fn program_camera(mut program: Vec<i64>, part: i64) -> Camera{
    let (comp_out, main_in): (Sender<i64>, Receiver<i64>) = channel();
    let (main_out, comp_in): (Sender<i64>, Receiver<i64>) = channel();
    let mut explorer: Camera = Camera {
        in_channel: main_in,
        headless: true,
        out_channel: main_out,
        term: Term::stdout(),
        buffer: vec![],
    };
    program[0] = part;
    let mut comp = ChannelComputer {
        receiver: comp_in,
        sender: comp_out,
    };
    thread::spawn(move || {
        let mut iterator = 0;
        run_int_code_on_computer(
            &mut iterator,
            &mut program,
            &mut comp,
            false,
        );
    });
    explorer
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut program = load_program(input);
    let mut cam1 = program_camera(program.clone(), 1);
    let part1 = cam1.explore();
    let mut cam2 = program_camera(program.clone(), 2);
    let part2 = cam2.command();
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
