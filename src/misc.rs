use std::{env, fs};

pub fn total_fuel(weights: &Vec<i32>, rec: bool) -> i32 {
    return weights
        .iter()
        .map(|x| fuel_cost(x, rec))
        .fold(0, |a, b| a + b);
}

pub fn load_weights(filename: &String) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let mut weights: Vec<i32> = vec![];
    for s in split {
        weights.push(s.to_string().parse::<i32>().unwrap());
    }
    return weights;
}

pub fn fuel_cost(arg: &i32, rec: bool) -> i32 {
    let fuel = arg / 3 - 2;
    if rec {
        if fuel < 0 {
            return 0;
        }
        return fuel + fuel_cost(&fuel, rec);
    } else {
        return fuel;
    }
}

pub fn valid_code(x: &i32) -> bool {
    let str = x.to_string();
    let mut old_char: char = 'q';
    let mut has_pair = false;
    let mut counts: i32 = 0;
    for c in str.chars() {
        if old_char != 'q' {
            if c.to_digit(10) < old_char.to_digit(10) {
                return false;
            }
            if c == old_char && !has_pair {
                counts += 1;
                print!("{0} has a pair", str);
            } else {
                if (counts == 1) {
                    has_pair = true;
                }
                counts = 0;
            }
        }
        old_char = c;
    }
    if (counts == 1) {
        has_pair = true;
    }
    println!(" and is monotonic");
    return has_pair;
}
