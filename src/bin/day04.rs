use std::io;
use std::io::Read;

pub fn is_pair(count: i32, strict: bool) -> bool {
    return (strict && count == 1) || (!strict && count >= 1);
}
pub fn valid_code(x: &i32, strict: bool) -> bool {
    let str = x.to_string();
    let mut old_char: char = str.chars().next().unwrap();
    let mut has_pair = false;
    let mut counts: i32 = 0;
    for c in str.chars().skip(1) {
        if c.to_digit(10) < old_char.to_digit(10) {
            return false;
        }
        if c == old_char && !has_pair {
            counts += 1;
        } else {
            if is_pair(counts, strict) {
                has_pair = true;
            }
            counts = 0;
        }
        old_char = c;
    }
    if is_pair(counts, strict) {
        has_pair = true;
    }
    return has_pair;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let spl = input
        .split("-")
        .map(|f| f.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let part1 = (spl[0]..spl[1]).filter(|n| valid_code(n, false)).count();
    let part2 = (spl[0]..spl[1]).filter(|n| valid_code(n, true)).count();
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
