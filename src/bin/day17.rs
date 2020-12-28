use std::io;
use std::io::Read;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use aoc::computer::{ChannelComputer, TextInterface};
use aoc::intcode::{load_program, run_int_code_on_computer};

extern crate rand;

pub struct Camera {
    text_inter: TextInterface,
}

impl Camera {
    fn lookup(&self, x: i32, y: i32) -> bool {
        return if let Some(c) = self.text_inter.get(x, y) {
            c != '.'
        } else {
            false
        };
    }

    fn command(&mut self) -> i64 {
        //Greedy algorithm for general solution?
        let prog = "A,B,A,B,A,C,B,C,A,C\n";
        let sub_a = "R,4,L,10,L,10\n";
        let sub_b = "L,8,R,12,R,10,R,4\n";
        let sub_c = "L,8,L,8,R,10,R,4\n";
        self.text_inter.upload_string(prog);
        self.text_inter.upload_string(sub_a);
        self.text_inter.upload_string(sub_b);
        self.text_inter.upload_string(sub_c);
        self.text_inter.upload_string("n\n");
        self.text_inter.buffered_reading()
    }
    fn explore(&mut self) -> usize {
        self.text_inter.buffered_reading();
        let mut total = 0;
        for y in 0..self.text_inter.buffer.len() {
            for x in 0..self.text_inter.buffer[y].len() {
                if self.text_inter.buffer[y as usize][x as usize] != '.' {
                    let mut neighbours = 0;
                    if x > 0 && self.lookup(x as i32 - 1, y as i32) {
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
        total
    }
}

fn program_camera(mut program: Vec<i64>, part: i64) -> Camera {
    let (comp_out, main_in): (Sender<i64>, Receiver<i64>) = channel();
    let (main_out, comp_in): (Sender<i64>, Receiver<i64>) = channel();
    let interface: TextInterface = TextInterface {
        in_channel: main_in,
        out_channel: main_out,
        buffer: vec![],
    };
    let explorer: Camera = Camera {
        text_inter: interface,
    };
    program[0] = part;
    let mut comp = ChannelComputer {
        receiver: comp_in,
        sender: comp_out,
    };
    thread::spawn(move || {
        let mut iterator = 0;
        run_int_code_on_computer(&mut iterator, &mut program, &mut comp, false);
    });
    explorer
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let program = load_program(input);
    let mut cam1 = program_camera(program.clone(), 1);
    let part1 = cam1.explore();
    let mut cam2 = program_camera(program.clone(), 2);
    let part2 = cam2.command();
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
