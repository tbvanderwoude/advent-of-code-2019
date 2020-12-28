extern crate rand;
use itertools::Itertools;
use std::{io, thread};
use std::io::Read;
use regex::Regex;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

use console::Term;

use aoc::computer::{ChannelComputer, TextInterface};
use aoc::intcode::{load_program, run_int_code_on_computer};
use std::collections::VecDeque;

pub struct Droid {
    text_inter: TextInterface,
    term: console::Term,
    command_queue: VecDeque<String>,
}
pub struct RoomInfo{
    name: String,
    items: Vec<String>,
    directions: Vec<String>,
}

impl Droid {
    fn string_from_buffer(&self) -> Vec<String>{
        let mut strings = vec![];
        for v in self.text_inter.buffer.iter(){
            strings.push(v.iter().collect::<String>());
        };
        return strings;
    }
    fn execute_program(&mut self) -> i64 {
        loop {
            self.text_inter.buffered_reading();
            self.text_inter.render();
            let mut command;
            if !self.command_queue.is_empty()
            {
                command = self.command_queue.pop_front().unwrap();
            }
            else{
                let mut chars = vec![];
                loop {
                    let read_char = self.term.read_char().unwrap();
                    chars.push(read_char);
                    if read_char == '\n' {
                        break;
                    }
                }
                command = chars.iter().collect::<String>();
            }
            self.text_inter.upload_string(command.as_str());
            self.term.clear_screen();
            thread::sleep(Duration::from_millis(50));
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut program = load_program(input);
    let (comp_out, main_in): (Sender<i64>, Receiver<i64>) = channel();
    let (main_out, comp_in): (Sender<i64>, Receiver<i64>) = channel();
    let inter = TextInterface{
        in_channel: main_in,
        out_channel: main_out,
        buffer: vec![],
    };
    let mut explorer: Droid = Droid {
        text_inter: inter,
        term: Term::stdout(),
        command_queue: ["north","take easter egg","east","take astrolabe","west","north",
            "take manifold","north","north","take hologram","north",
            "take weather machine","north","take antenna","west"].iter().map(|x|x.to_string()+"\n").collect()
    };
    let mut comp = ChannelComputer {
        receiver: comp_in,
        sender: comp_out,
    };
    thread::spawn(move || {
        let mut iterator = 0;
        run_int_code_on_computer(&mut iterator, &mut program, &mut comp, false);
    });
    explorer.execute_program();
}
