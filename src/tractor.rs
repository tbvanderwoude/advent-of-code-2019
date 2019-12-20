extern crate petgraph;
extern crate rand;

use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use console::Term;

use crate::async_intcode;

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
            //self.render();
        }
        self.count
    }

    fn render(&mut self) {
        self.term.clear_screen();
        if !self.map.is_empty() {
            let max_x = *self.map.keys().map(|(a, b)| a).max().unwrap();
            let max_y = *self.map.keys().map(|(a, b)| b).max().unwrap();
            let min_x = *self.map.keys().map(|(a, b)| a).min().unwrap();
            let min_y = *self.map.keys().map(|(a, b)| b).min().unwrap();
            let w = (max_x - min_x) as usize;
            let h = (max_y - min_y) as usize;

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
        x: x,
        y: y,
        count: 0,
    };
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
    explorer.explore()
}

fn check(mut program: Vec<i64>, cursor_x: i64, cursor_y: i64) -> bool {
    deploy(program.clone(), cursor_x, cursor_y) == 1
}

fn vert_fits(mut program: Vec<i64>, cursor_x: i64, cursor_y: i64) -> bool {
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

pub fn view(mut program: Vec<i64>) {
    let mut total = 0;
    let mut buh = true;

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
            println!(
                "({}, {}) fits: {}",
                cursor_x,
                cursor_y - size_min,
                cursor_x * 10000 + cursor_y - size_min
            );
            break;
        } else {
        }
        cursor_y += 1;
    }
    for y in (cursor_y - size - 3)..(cursor_y + 1) {
        let mut line = false;
        for x in (cursor_x)..(cursor_x + size + 3) {
            if (y as f32) < 1.01f32 * (x as f32) {
                let res = deploy(program.clone(), x, y);
                if res == 1 {
                    if x == cursor_x && y == cursor_y {
                        print!("x");
                    } else {
                        print!("{}", res);
                    }
                    line = true;
                } else if res == 0 && line {
                    break;
                } else if (x - 1) == y {
                    print!("x");
                } else {
                    print!("{}", res);
                }
                if x == 99 && res == 1 && !buh {
                    buh = true;
                    println!("Coords({}, {})", x, y);
                }
                total += res;
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("Grand total: {}", total);
}
