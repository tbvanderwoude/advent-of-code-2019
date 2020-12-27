use std::io;
use std::io::Read;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

use console::Term;

use aoc::computer::{ChannelComputer, TextInterface};
use aoc::intcode::{load_program, run_int_code_on_computer};

extern crate rand;

pub struct SpringDroid {
    text_inter: TextInterface
}

impl SpringDroid {
    fn execute_program(&mut self, program: &str) -> i64 {
        self.text_inter.upload_string(program);
        return self.text_inter.buffered_reading();
    }
}

pub fn run_program(program: &Vec<i64>, prog: &str) -> i64 {
    let (comp_out, main_in): (Sender<i64>, Receiver<i64>) = channel();
    let (main_out, comp_in): (Sender<i64>, Receiver<i64>) = channel();
    let mut interface: TextInterface = TextInterface{
        in_channel: main_in,
        out_channel: main_out,
        buffer: vec![],
    };
    let mut comp = ChannelComputer {
        receiver: comp_in,
        sender: comp_out,
    };
    let mut cln = program.clone();
    thread::spawn(move || {
        let mut iterator = 0;
        run_int_code_on_computer(&mut iterator, &mut cln, &mut comp, false);
    });
    interface.upload_string(prog);
    interface.buffered_reading()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let program = load_program(input);
    let part1 = run_program(
        &program,
        "NOT A J\nNOT J J\nAND B J\nAND C J\nNOT J J\nAND D J\nWALK\n",
    );
    let part2 = run_program(
        &program,
        "NOT C J\nAND H J\nNOT B T\nOR T J\nNOT A T\nOR T J\nAND D J\nRUN\n",
    );
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
