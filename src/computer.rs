use std::collections::VecDeque;
use std::io;
use std::sync::mpsc::{Receiver, Sender};

use crate::intcode::Computer;

pub struct TextInterface{
    pub in_channel: Receiver<i64>,
    pub out_channel: Sender<i64>,
    pub buffer: Vec<Vec<char>>
}

impl TextInterface {
    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        if (y < 0 || y >= self.buffer.len() as i32)
            || (x < 0 || x >= self.buffer[y as usize].len() as i32)
        {
            return None;
        }
        return Some(self.buffer[y as usize][x as usize]);
    }
    pub fn upload_string(&self, string: &str) {
        for c in string.chars() {
            self.out_channel.send(c as i64).unwrap();
        }
    }
    pub fn buffered_reading(&mut self) -> i64
    {
        self.buffer.clear();
        self.buffer.push(vec![]);
        let mut retval = -1;
        loop {
            let res = self.in_channel.recv();
            if res.is_ok() {
                if res.unwrap()>255{
                    retval = res.unwrap();
                }
                else{
                    let info = (res.unwrap() as u8) as char;
                    if info == '\n' {
                        self.buffer.push(vec![]);
                    } else {
                        let n = self.buffer.len() - 1;
                        self.buffer[n].push(info);
                    }
                }

            } else {
                break;
            }
        }
        return retval;
    }
    pub fn render(&self) {
        for line in &self.buffer {
            println!("{}", line.iter().collect::<String>());
        }
    }
}

pub struct NetworkComputer {
    pub receiver: Receiver<i64>,
    pub sender: Sender<i64>,
}

impl NetworkComputer {
    pub fn new(receiver: Receiver<i64>, sender: Sender<i64>) -> NetworkComputer {
        NetworkComputer { receiver, sender }
    }
}

impl Computer for NetworkComputer {
    fn input(&mut self) -> i64 {
        let mut input: i64 = -1;
        let result = self.receiver.try_recv();
        if result.is_ok() {
            input = result.unwrap();
        }
        input
    }
    fn output(&mut self, x: i64) {
        self.sender.send(x).unwrap();
    }
}

pub struct ChannelComputer {
    pub receiver: Receiver<i64>,
    pub sender: Sender<i64>,
}

impl ChannelComputer {
    pub fn new(receiver: Receiver<i64>, sender: Sender<i64>) -> ChannelComputer {
        ChannelComputer { receiver, sender }
    }
}

impl Computer for ChannelComputer {
    fn input(&mut self) -> i64 {
        let input;
        loop {
            let result = self.receiver.recv();
            if result.is_ok() {
                input = result.unwrap();
                break;
            }
        }
        input
    }
    fn output(&mut self, x: i64) {
        self.sender.send(x).unwrap();
    }
}

pub struct TestComputer {
    input_buffer: VecDeque<i64>,
    pub output_buffer: Vec<i64>,
}

impl TestComputer {
    pub fn new(input_buffer: VecDeque<i64>) -> TestComputer {
        TestComputer {
            input_buffer,
            output_buffer: vec![],
        }
    }
}

impl Computer for TestComputer {
    fn input(&mut self) -> i64 {
        return self.input_buffer.pop_front().unwrap();
    }
    fn output(&mut self, x: i64) {
        self.output_buffer.push(x);
    }
}

pub struct InteractiveComputer;

impl Computer for InteractiveComputer {
    fn input(&mut self) -> i64 {
        let mut ret = String::new();
        return match io::stdin().read_line(&mut ret) {
            Ok(_n) => match ret.trim().to_string().parse::<i64>() {
                Ok(n) => n,
                Err(_e) => 0,
            },
            Err(_error) => 0,
        };
    }
    fn output(&mut self, x: i64) {
        println!("{}", x);
    }
}