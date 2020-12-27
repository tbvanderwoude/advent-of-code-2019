use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use console::Term;

use aoc::computer::ChannelComputer;
use aoc::intcode::{load_program, run_int_code_on_computer};

extern crate petgraph;
extern crate rand;

pub struct Tractor {
    in_channel: Receiver<i64>,
    out_channel: Sender<i64>,
    term: console::Term,
    x: i64,
    y: i64,
    count: i64,
    map: HashMap<(i64, i64), i64>,
}

impl Tractor {
    fn explore(&mut self) -> i64 {
        self.out_channel.send(self.x);
        self.out_channel.send(self.y);
        loop {
            let res = self.in_channel.recv();
            if res.is_ok() {
                let info = res.unwrap();
                if info == 1 {
                    self.count += 1;
                }
                self.map.insert((self.x, self.y), info);
                break;
            }
        }
        self.count
    }

    fn render(&mut self) {
        self.term.clear_screen();
        if !self.map.is_empty() {
            let max_x = *self.map.keys().map(|(a, _b)| a).max().unwrap();
            let max_y = *self.map.keys().map(|(_a, b)| b).max().unwrap();
            let min_x = *self.map.keys().map(|(a, _b)| a).min().unwrap();
            let min_y = *self.map.keys().map(|(_a, b)| b).min().unwrap();
            let w = (max_x - min_x) as usize;
            for y in min_y..max_y {
                let mut line: Vec<char> = vec![' '; w + 2];
                for x in min_x..max_x {
                    if self.map.contains_key(&(x, y)) {
                        match *self.map.get(&(x, y)).unwrap() {
                            0 => line[(x - min_x) as usize] = '0',
                            1 => line[(x - min_x) as usize] = '1',
                            2 => line[(x - min_x) as usize] = 'X',
                            _ => line[(x - min_x) as usize] = '#',
                        }
                    }
                }
                println!("{}", line.iter().collect::<String>());
            }
        }
    }
}

pub fn deploy(mut program: Vec<i64>, x: i64, y: i64) -> i64 {
    let (comp_out, main_in): (Sender<i64>, Receiver<i64>) = channel();
    let (main_out, comp_in): (Sender<i64>, Receiver<i64>) = channel();
    let mut explorer: Tractor = Tractor {
        in_channel: main_in,
        out_channel: main_out,
        term: Term::stdout(),
        map: HashMap::new(),
        x,
        y,
        count: 0,
    };
    let mut comp = ChannelComputer {
        receiver: comp_in,
        sender: comp_out,
    };
    thread::spawn(move || {
        let mut iterator = 0;
        run_int_code_on_computer(&mut iterator, &mut program, &mut comp, false);
    });

    explorer.explore()
}

fn check(program: Vec<i64>, cursor_x: i64, cursor_y: i64) -> bool {
    deploy(program.clone(), cursor_x, cursor_y) == 1
}

fn vert_fits(program: Vec<i64>, cursor_x: i64, cursor_y: i64) -> bool {
    let mut y_fits = true;
    for y in cursor_y..(cursor_y + 100) {
        let res = deploy(program.clone(), cursor_x, y);
        if res == 0 {
            y_fits = false;
            break;
        }
    }
    y_fits
}

fn check_beam_start(program: &Vec<i64>) -> usize {
    let mut count = 0;
    for x in 0..50 {
        for y in 0..50 {
            if check(program.clone(), x, y) {
                count += 1;
            }
        }
    }
    return count;
}

pub fn fit_rectangle(program: &Vec<i64>) -> i64 {
    let mut cursor_x = 0;
    let mut cursor_y = 10;
    let size = 100;
    let size_min = size - 1;
    loop {
        loop {
            if check(program.clone(), cursor_x, cursor_y) {
                break;
            }
            cursor_x += 1;
        }
        if check(program.clone(), cursor_x + size_min, cursor_y - size_min) {
            return cursor_x * 10000 + (cursor_y - size_min);
        }
        cursor_y += 1;
    }
    return -1;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let program = load_program(input);
    let part1 = check_beam_start(&program);
    let part2 = fit_rectangle(&program);
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
