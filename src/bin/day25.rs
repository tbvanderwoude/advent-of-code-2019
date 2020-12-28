extern crate rand;
use itertools::Itertools;
use std::{io, thread};
use std::io::Read;
use regex::Regex;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use permutohedron;
use permutohedron::Heap;
use console::Term;

use aoc::computer::{ChannelComputer, TextInterface};
use aoc::intcode::{load_program, run_int_code_on_computer};
use std::collections::VecDeque;

pub struct Droid {
    text_inter: TextInterface,
    term: console::Term,
    command_queue: VecDeque<String>,
    perm_index: i32,
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
            let buffer_s = self.string_from_buffer().iter().join("\n");
            self.text_inter.render();
            if !buffer_s.contains("Analysis complete!"){
                let mut command;
                if !self.command_queue.is_empty()
                {
                    command = self.command_queue.pop_front().unwrap();
                    if self.command_queue.is_empty()
                    {
                        let items = vec!["astrolabe","weather machine","antenna","easter egg","hologram","space law space brochure","manifold"];
                        command+= &*items.iter().map(|x| "drop ".to_owned()+x).join("\n");
                        command.push('\n');
                    }
                }
                else{
                    if self.perm_index<256{
                        let mut held = vec!["astrolabe","weather machine","space law space brochure","antenna","easter egg","hologram","manifold"];
                        let mut subset = vec![];
                        for (j,&el) in held.iter().enumerate(){
                            if self.perm_index & 1<<j == 1<<j{
                                subset.push(el);
                            }
                        }
                        command = format!("{}\nsouth\n{}\n",
                                          subset.iter().map(|x| "take ".to_owned()+x).join("\n").to_string(),
                                          subset.iter().map(|x| "drop ".to_owned()+x).join("\n").to_string());
                        self.perm_index+=1;
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
                }
                self.text_inter.upload_string(command.as_str());
                self.term.clear_screen();
                thread::sleep(Duration::from_millis(20));
            }
            else{
                let re = Regex::new(r"\d{6}").unwrap();
                let str_num =  re.find(&buffer_s).unwrap().as_str();
                return str_num.parse::<i64>().unwrap();
            }
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
        command_queue: ["north","take easter egg","east","take astrolabe","south","take space law space brochure","north","west","north",
            "take manifold","north","north","take hologram","north",
            "take weather machine","north","take antenna","west"].iter().map(|x|x.to_string()+"\n").collect(),
        perm_index: 0
    };
    let mut comp = ChannelComputer {
        receiver: comp_in,
        sender: comp_out,
    };
    thread::spawn(move || {
        let mut iterator = 0;
        run_int_code_on_computer(&mut iterator, &mut program, &mut comp, false);
    });
    println!("Part 1: {}",explorer.execute_program());
}
