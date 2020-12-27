use aoc::common::parse_numbers;
use std::io;
use std::io::Read;
use console::Term;

use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use aoc::intcode;
use aoc::intcode::{Computer, load_program};

#[derive(Clone)]
pub struct Cabinet {
    pub term: console::Term,
    pub map: HashMap<(i64, i64), i64>,
    pub new_x: i64,
    pub new_y: i64,
    pub paddle_x: i64,
    pub paddle_y: i64,
    pub ball_x: i64,
    pub ball_y: i64,
    pub score: i64,
    pub headless: bool,
    pub autoplay: bool,
    pub game_exiting: bool,
}
impl Cabinet {
    fn render(&mut self) {
        self.term.clear_screen();
        for y in 0..20 {
            let mut line: [char; 40] = [' '; 40];
            for x in 0..40 {
                if self.map.contains_key(&(x, y)) {
                    match *self.map.get(&(x, y)).unwrap() {
                        1 | 2 => line[x as usize] = '■',
                        3 => line[x as usize] = '_',
                        4 => line[x as usize] = 'o',
                        _ => (),
                    }
                }
            }
            println!("{}", line.iter().collect::<String>());
        }
    }
}

impl Computer for Cabinet {
    fn input(&mut self) -> i64 {
        if !self.headless{
            self.render();
            thread::sleep(Duration::from_millis(20));
        }

        if self.autoplay {
            if self.paddle_x < self.ball_x {
                return 1;
            }
            if self.paddle_x > self.ball_x {
                return -1;
            }
            return 0;
        } else {
            let input = self.term.read_char();
            match input.unwrap() {
                'd' => return 1,
                'a' => return -1,
                _ => return 0,
            }
        }
    }
    fn output(&mut self, num: i64) {
        if self.new_x == -42 {
            self.new_x = num;
        } else if self.new_y == -42 {
            self.new_y = num;
        } else {
            if self.new_x == -1 && self.new_y == 0 {
                self.score = num;
                self.new_x = -42;
                self.new_y = -42;
            } else {
                if num == 3 {
                    self.paddle_x = self.new_x;
                    self.paddle_y = self.new_y;
                }
                if num == 4 {
                    self.ball_x = self.new_x;
                    self.ball_y = self.new_y;
                }
                self.map.insert((self.new_x, self.new_y), num);
                self.new_x = -42;
                self.new_y = -42;
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut program = load_program(input);

    let mut counter: usize = 0;
    let mut arcade: Cabinet = Cabinet {
        autoplay: false,
        term: Term::stdout(),
        map: HashMap::new(),
        new_x: -42,
        new_y: -42,
        headless: false,
        game_exiting: false,
        score: 0,
        ball_x: -1,
        ball_y: -1,
        paddle_x: -1,
        paddle_y: -1,
    };
    let mut prog_cln = program.clone();
    intcode::run_int_code_on_computer(&mut counter, &mut prog_cln, &mut arcade, false);
    let part1 = arcade
        .map
        .iter()
        .map(|(&k, &v)| v)
        .filter(|v| *v == 2)
        .count();
    counter = 0;
    program[0] = 2;
    intcode::run_int_code_on_computer(&mut counter, &mut program, &mut arcade, false);
    let part2 = arcade.score;
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
