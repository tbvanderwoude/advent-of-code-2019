use aoc::common::parse_numbers;
use std::io;
use std::io::Read;
use std::collections::HashMap;
use aoc::intcode;
use aoc::intcode::{Computer, load_program};


pub struct Turtle {
    pub dir: i64,
    pub map: HashMap<(i64, i64), i64>,
    pub paint: bool,
    pub x: i64,
    pub y: i64,
}

impl Computer for Turtle {
    fn input(&mut self) -> i64 {
        let mut color: i64 = 0;
        if self.map.contains_key(&(self.x, self.y)) {
            color = *self.map.get(&(self.x, self.y)).unwrap();
        }
        return color;
    }
    fn output(&mut self, num: i64) {
        if self.paint {
            self.map.insert((self.x, self.y), num);
            self.paint = false;
        } else {
            self.dir += num * 2 - 1;
            if self.dir < 0 {
                self.dir += 4;
            }
            if self.dir > 3 {
                self.dir -= 4;
            }
            match self.dir {
                0 => self.y -= 1,
                1 => {
                    self.x += 1;
                }
                2 => {
                    self.y += 1;
                }
                3 => {
                    self.x -= 1;
                }
                _ => (),
            }
            self.paint = true;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut program = load_program(input);
    let mut robot: Turtle = Turtle {
        map: HashMap::new(),
        dir: 0,
        paint: true,
        x: 0,
        y: 0,
    };
    robot.map.insert((0, 0), 1);
    let mut counter: usize = 0;
    intcode::run_int_code_on_computer(&mut counter, &mut program, &mut robot, false);
    let part1 = robot.map.len();
    println!("Part 1: {}\nPart 2: ", part1);
    for y in 0..6 {
        for x in 0..42 {
            if robot.map.contains_key(&(x, y)) && *robot.map.get(&(x, y)).unwrap() == 1 {
                print!("■");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
