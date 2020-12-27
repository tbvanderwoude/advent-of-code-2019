extern crate rand;

use std::{io, thread};
use std::io::Read;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

use console::Term;

use aoc::computer::ChannelComputer;
use aoc::intcode::{load_program, run_int_code_on_computer};

pub struct SpringDroid {
    in_channel: Receiver<i64>,
    out_channel: Sender<i64>,
    term: console::Term,
    buffer: Vec<Vec<char>>,
}

impl SpringDroid {
    fn lookup(&self, x: i32, y: i32) -> bool {
        if (y < 0 || y >= self.buffer.len() as i32)
            || (x < 0 || x >= self.buffer[y as usize].len() as i32)
            || (self.buffer[y as usize][x as usize] == '.')
        {
            return false;
        }
        return true;
    }

    fn execute_program(&mut self) -> i64 {
        loop {
            let mut line_buffer: Vec<char> = vec![];
            loop {
                let res = self.in_channel.recv_timeout(Duration::from_millis(20));
                if res.is_ok() {
                    if res.unwrap() > 255 {
                        return res.unwrap();
                    } else {
                        let info = (res.unwrap() as u8) as char;
                        if info == '\n' {
                            println!("{}", line_buffer.iter().collect::<String>());
                            line_buffer.clear();
                        } else {
                            line_buffer.push(info);
                        }
                    }
                } else {
                    break;
                }
            }
            let mut chars = vec![];
            loop {
                let read_char = self.term.read_char().unwrap();
                chars.push(read_char);
                if read_char == '\n' {
                    break;
                }
            }
            self.term.clear_screen();
            thread::sleep(Duration::from_millis(16));
            self.upload_string(chars.into_iter().collect());
        }
    }
    fn upload_string(&self, string: String) {
        for c in string.chars() {
            self.out_channel.send(c as i64);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut program = load_program(input);
    let (comp_out, main_in): (Sender<i64>, Receiver<i64>) = channel();
    let (main_out, comp_in): (Sender<i64>, Receiver<i64>) = channel();
    let mut explorer: SpringDroid = SpringDroid {
        in_channel: main_in,
        out_channel: main_out,
        term: Term::stdout(),
        buffer: vec![],
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
