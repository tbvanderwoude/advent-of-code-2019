use std::collections::HashMap;

use crate::intcode;
use crate::intcode::Computer;

enum RobotDir
{
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
}

pub struct Turtle {
    pub dir: i64,
    pub map: HashMap<(i64, i64), i64>,
    pub paint: bool,
    pub x: i64,
    pub y: i64,
}

pub fn paint_using_robot(mut program: Vec<i64>)
{
    let mut counter: usize = 0;
    let mut robot: Turtle = Turtle { map: HashMap::new(), dir: 0, paint: true, x: 0, y: 0 };
    robot.map.insert((0, 0), 1);
    println!("{}", intcode::run_int_code_on_computer(&mut counter, &mut program, &mut robot, false));
    println!("{}", robot.map.len());
    for y in 0..30 {
        for x in 0..50 {
            if robot.map.contains_key(&(x, y)) && *robot.map.get(&(x, y)).unwrap() == 1
            {
                print!("■");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

impl Computer for Turtle {
    fn input(&mut self) -> i64 {
        let mut color: i64 = 0;
        if self.map.contains_key(&(self.x, self.y))
        {
            color = *self.map.get(&(self.x, self.y)).unwrap();
        }
        if color == 1
        {
            println!("It is white at ({0}, {1})", self.x, self.y);
        } else {
            println!("It is black at ({0}, {1})", self.x, self.y);
        }
        return color;
    }
    fn output(&mut self, num: i64) {
        if self.paint
        {
            self.map.insert((self.x, self.y), num);
            if num == 1
            {
                println!("Painted ({0}, {1}) white", self.x, self.y);
            } else {
                println!("Painted ({0}, {1}) black", self.x, self.y);
            }
            self.paint = false;
        } else {
            self.dir += num * 2 - 1;
            if self.dir < 0
            {
                self.dir += 4;
            }
            if self.dir > 3
            {
                self.dir -= 4;
            }
            let mut strDir = "up";
            match self.dir
                {
                    0 => { self.y -= 1 }
                    1 => {
                        self.x += 1;
                        strDir = "right";
                    }
                    2 => {
                        self.y += 1;
                        strDir = "down";
                    }
                    3 => {
                        self.x -= 1;
                        strDir = "left";
                    }
                    _ => ()
                }
            //println!("Moved {0}",strDir);
            self.paint = true;
        }
    }
}
