use std::io;
use std::io::Read;
extern crate regex;
use regex::Regex;

#[derive(Clone, Copy)]
pub struct Moon {
    p: (i32, i32, i32),
    v: (i32, i32, i32),
}

impl Moon {
    fn equal_x(&self, other: Moon) -> bool {
        return self.p.0 == other.p.0 && self.v.0 == other.v.0;
    }
    fn equal_y(&self, other: Moon) -> bool {
        return self.p.1 == other.p.1 && self.v.1 == other.v.1;
    }
    fn equal_z(&self, other: Moon) -> bool {
        return self.p.2 == other.p.2 && self.v.2 == other.v.2;
    }
    fn compute_energy(&self) -> i32 {
        return (self.p.0.abs() + self.p.1.abs() + self.p.2.abs())
            * (self.v.0.abs() + self.v.1.abs() + self.v.2.abs());
    }
    fn apply_velocity(&mut self) {
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
        self.p.2 += self.v.2;
    }
    fn to_string(&self) -> String {
        return format!(
            "pos=<x = {0}, y = {1}>, z = {2}>, vel=<x = {3}, y = {4} z = {5}>",
            self.p.0, self.p.1, self.p.2, self.v.0, self.v.1, self.v.2
        );
    }
}

pub fn full_alignment(moons: Vec<Moon>) -> i64 {
    let al = component_wise_alignment(moons);
    return lcm(lcm(al.0, al.1), al.2);
}

pub fn component_wise_alignment(mut moons: Vec<Moon>) -> (i64, i64, i64) {
    let initial_moons = moons.clone();
    //Apply gravity
    let mut equal_x: i64 = -1;
    let mut equal_y: i64 = -1;
    let mut equal_z: i64 = -1;
    for iter in 1..100000000 {
        simulate_moons(&mut moons);
        if equal_x == -1 {
            for i in 0..moons.len() {
                if !moons[i].equal_x(initial_moons[i]) {
                    break;
                }
                if i == moons.len() - 1 {
                    equal_x = iter as i64;
                }
            }
        }
        if equal_y == -1 {
            for i in 0..moons.len() {
                if !moons[i].equal_y(initial_moons[i]) {
                    break;
                }
                if i == moons.len() - 1 {
                    equal_y = iter as i64;
                }
            }
        }
        if equal_z == -1 {
            for i in 0..moons.len() {
                if !moons[i].equal_z(initial_moons[i]) {
                    break;
                }
                if i == moons.len() - 1 {
                    equal_z = iter as i64
                }
            }
        }
        if equal_x != -1 && equal_y != -1 && equal_z != -1 {
            break;
        }
    }
    return (equal_x, equal_y, equal_z);
}

pub fn simulate_moons(moons: &mut Vec<Moon>) {
    for i in 0..(&moons).len() {
        //println!("{}",moons[i].to_string());
        for j in 0..moons.len() {
            moons[i].v.0 += (moons[j].p.0 - moons[i].p.0).signum();
            moons[i].v.1 += (moons[j].p.1 - moons[i].p.1).signum();
            moons[i].v.2 += (moons[j].p.2 - moons[i].p.2).signum();
        }
    }
    //Apply velocity and print
    for moon in moons {
        moon.apply_velocity();
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    if a < b {
        return a * b / gcd(b, a);
    }
    return a * b / gcd(a, b);
}

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

pub fn load_moons(contents: String) -> Vec<Moon> {
    let split = contents.split("\n");
    let mut moons = vec![];
    let rg = Regex::new(r"x=(-?\d+), y=(-?\d+), z=(-?\d+)").unwrap();
    for s in split {
        let caps = rg.captures(s).unwrap();
        //Thanks I hate it
        let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let z = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
        moons.push(Moon {
            p: (x, y, z),
            v: (0, 0, 0),
        });
    }
    return moons;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let moons = load_moons(input);
    let mut sim_copy = moons.clone();
    for _ in 0..1000 {
        simulate_moons(&mut sim_copy);
    }
    let part1: i32 = sim_copy.iter().map(|m| m.compute_energy()).sum();
    let part2 = full_alignment(moons);
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
