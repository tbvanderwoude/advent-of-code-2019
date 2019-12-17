extern crate rand;

use std::cmp::{max, min};
use std::collections::HashMap;
use std::env;
use std::sync::mpsc::{channel, Receiver, Sender, sync_channel, SyncSender};
use std::thread;

use console::Term;
use crate::async_intcode;
use crate::intcode;
use crate::space_image::render;
use std::time::Duration;

pub struct Camera {
    in_channel: Receiver<i64>,
    out_channel: Sender<i64>,
    term: console::Term,
    buffer: Vec<Vec<char>>,

}

impl Camera {
    fn lookup(&self,x: i32, y: i32) -> bool
    {
        if y<0||y>=self.buffer.len() as i32
        {
            return false;
        }
        else if x<0||x>=self.buffer[y as usize].len() as i32
        {
            return false;
        }
        else if self.buffer[y as usize][x as usize]=='.'
        {
            return false;
        }
        return true;
    }

    fn command(&mut self) {
        let mut i = 0;
        //"R,4,L,10,L,10,L,8,R,12,R,10,R,4,R,4,L,10,L,10,L,8,
        // R,12,R,10,R,4,R,12,R,10,R,4,"
        let prog = "A,B,A,B,A,C,B,C,A,C\n";
        let sub_a = "R,4,L,10,L,10\n";
        let sub_b = "L,8,R,12,R,10,R,4\n";
        let sub_c = "L,8,L,8,R,10,R,4\n";
        self.upload_string(prog);
        self.upload_string(sub_a);
        self.upload_string(sub_b);
        //placeholder
        self.upload_string(sub_c);
        self.upload_string("n\n");

        self.buffer.push(vec![]);
        let mut stream_buffer: Vec<char> = vec![];
        loop {
            let res = self.in_channel.recv();
            if res.is_ok() {
                if res.unwrap()>255 {
                    println!("{}", res.unwrap());
                }
                else {
                    let info = (res.unwrap() as u8) as char;
                    if info=='\n'
                    {
                        println!("{}", stream_buffer.iter().collect::<String>());
                        stream_buffer.clear();
                    }
                    else {
                        stream_buffer.push(info);
                    }
                }
            }
            if i%10000==0
            {
            }
            i+=1;
        }


    }
    fn upload_string (&self,string: &str)
    {
        for c in string.chars(){
            self.out_channel.send(c as i64);
        }
    }
    fn show(&mut self) {
        let mut i = 0;
        self.buffer.push(vec![]);
        loop {
            let res = self.in_channel.recv();
            if res.is_ok() {
                let info = (res.unwrap() as u8) as char;
                if info=='\n'
                {
                    self.buffer.push(vec![]);
                }
                else {
                    let n =self.buffer.len() -1;
                    self.buffer[n].push(info);
                }
            }
            self.render();
            thread::sleep(Duration::from_millis(10));
            if i%10000==0
            {
            }
            i+=1;
        }
    }
    fn render(&self) {
        self.term.clear_screen();
        for line in &self.buffer{
            println!("{}", line.iter().collect::<String>());
        }
        let mut total = 0;
        for y in 0..self.buffer.len(){
            for x in 0..self.buffer[y].len(){
                if self.buffer[y as usize][x as usize]!='.'
                {
                    let mut neighbours = 0;
                    if self.lookup(x as i32-1,y as i32)
                    {
                        neighbours+=1;
                    }
                    if self.lookup(x as i32+1,y as i32)
                    {
                        neighbours+=1;
                    }
                    if self.lookup(x as i32,y as i32+1)
                    {
                        neighbours+=1;
                    }
                    if self.lookup(x as i32,y as i32-1)
                    {
                        neighbours+=1;
                    }
                    if neighbours>2
                    {
                        total+=x*y;
                    }
                }
            }
        }
        println!("Total: {}",total);
    }
}

pub fn view(mut program: Vec<i64>) {
    let (computerOut, mainIn): (Sender<i64>, Receiver<i64>) = channel();
    let (mainOut, computerIn): (Sender<i64>, Receiver<i64>) = channel();
    let mut explorer: Camera = Camera {
        in_channel: mainIn,
        out_channel: mainOut,
        term: Term::stdout(),
        buffer: vec![],
    };
    program[0]=2;
    thread::spawn(move || {
        let mut iterator = 0;
        async_intcode::run_int_code_on_computer(
            &mut iterator,
            &mut program,
            computerIn,
            computerOut,
            false,
        );
    });
    explorer.command();
}
