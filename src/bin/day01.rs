use aoc::common::parse_numbers;
use std::io;
use std::io::Read;

pub fn total_fuel(weights: &Vec<i32>, rec: bool) -> i32 {
    weights
        .iter()
        .map(|x| fuel_cost(x, rec))
        .fold(0, |a, b| a + b)
}
pub fn fuel_cost(arg: &i32, rec: bool) -> i32 {
    let fuel = arg / 3 - 2;
    if rec {
        if fuel < 0 {
            0
        } else {
            fuel + fuel_cost(&fuel, rec)
        }
    } else {
        fuel
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers = parse_numbers(input);
    let part1 = total_fuel(&numbers, false);
    let part2 = total_fuel(&numbers, true);
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
