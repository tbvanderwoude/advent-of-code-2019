use std::io;
use std::io::Read;

pub fn read_signal(contents: String) -> Vec<i32> {
    contents
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect()
}

pub fn sample_square(phase_width: i32, mut x: i32) -> i32 {
    while x < 0 {
        x += phase_width;
    }
    let phased_x = x % phase_width;
    return if phased_x < phase_width / 4 {
        0
    } else if phased_x < phase_width / 2 {
        1
    } else if phased_x < phase_width * 3 / 4 {
        0
    } else {
        -1
    };
}

pub fn part_1(mut signal: Vec<i32>) -> String {
    for _i in 0..100 {
        let old_signal: Vec<i32> = signal.clone();
        for j in 0..old_signal.len() {
            signal[j] = old_signal
                .iter()
                .enumerate()
                .map(|(index, x)| x * sample_square(((j + 1) * 4) as i32, (index + 1) as i32))
                .sum();
            signal[j] = signal[j].abs() % 10;
        }
    }
    (&signal)
        .into_iter()
        .map(|x| x.to_string())
        .take(8)
        .collect::<String>()
}

pub fn part_2(signal: Vec<i32>) -> String {
    let n = signal.len();
    let skip: usize = signal.iter().take(7).fold(0, |a, b| a * 10 + *b) as usize;
    let mut long_signal: Vec<i32> = signal.iter().cycle().take(n * 10000).map(|x| *x).collect();
    for _i in 0..100 {
        for j in (skip..long_signal.len()).rev() {
            if j != long_signal.len() - 1 {
                long_signal[j] = (long_signal[j] + long_signal[j + 1]) % 10;
            }
        }
    }
    (&long_signal)
        .into_iter()
        .skip(skip)
        .take(8)
        .map(|x| x.to_string())
        .collect::<String>()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let signal = read_signal(input);
    let part1 = part_1(signal.clone());
    let part2 = part_2(signal.clone());
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
