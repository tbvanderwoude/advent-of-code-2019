extern crate rand;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

use console::Term;

use crate::async_intcode;
use crate::intcode;

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

    fn execute_program(&mut self, program: &str) -> i64{
        let mut i = 0;
        self.upload_string(program);

        self.buffer.push(vec![]);
        let mut stream_buffer: Vec<char> = vec![];
        loop {
            let res = self.in_channel.recv_timeout(Duration::from_millis(5000));
            if res.is_ok() {
                if res.unwrap() > 255 {
                    return res.unwrap();
                } else {
                    let info = (res.unwrap() as u8) as char;
                    if info == '\n' {
                        println!("{}", stream_buffer.iter().collect::<String>());
                        stream_buffer.clear();
                    } else {
                        stream_buffer.push(info);
                    }
                }
            }
            else{
                break;
            }
        }
        return 0;
    }
    fn upload_string(&self, string: &str) {
        for c in string.chars() {
            self.out_channel.send(c as i64);
        }
    }
    fn render(&self) {
        self.term.clear_screen();
        for line in &self.buffer {
            println!("{}", line.iter().collect::<String>());
        }
    }
}

pub fn run_program(filename: &String, prog: &str) -> i64{
    let (comp_out, main_in): (Sender<i64>, Receiver<i64>) = channel();
    let (main_out, comp_in): (Sender<i64>, Receiver<i64>) = channel();
    let mut explorer: SpringDroid = SpringDroid {
        in_channel: main_in,
        out_channel: main_out,
        term: Term::stdout(),
        buffer: vec![],
    };
    let mut program = intcode::load_program(filename);
    thread::spawn(move || {
        let mut iterator = 0;
        async_intcode::run_int_code_on_computer(
            &mut iterator,
            &mut program,
            comp_in,
            comp_out,
            false,
        );
    });
    explorer.execute_program(prog)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    #[test]
    fn run() {
        assert_eq!(run_program(&"input/intcode/springdroid.txt".to_string(),"NOT C J\nAND H J\nNOT B T\nOR T J\nNOT A T\nOR T J\nAND D J\nRUN\n"),1141997803);
    }
    #[test]
    fn walk() {
        assert_eq!(run_program(&"input/intcode/springdroid.txt".to_string(),"NOT A J\nNOT J J\nAND B J\nAND C J\nNOT J J\nAND D J\nWALK\n"),19354928);
    }
}
