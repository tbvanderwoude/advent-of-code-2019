use std::{io, thread};
use std::io::Read;
use std::sync::mpsc::{channel, Receiver, Sender};

use permutohedron;
use permutohedron::Heap;

use aoc::computer::ChannelComputer;
use aoc::intcode::{load_program, run_int_code_on_computer};

fn simulate_amp_loop(program: &Vec<i64>, has_loop: bool) -> i64 {
    let mut phases: Vec<i64>;
    if has_loop {
        phases = vec![5, 6, 7, 8, 9]
    } else {
        phases = vec![0, 1, 2, 3, 4];
    }
    let heap = Heap::new(&mut phases);
    let mut permutations = vec![];
    for data in heap {
        permutations.push(data.clone());
    }
    let mut max_signal: i64 = 0;
    for permutation in permutations {
        let mut s = 0;
        let (send, receive): (Sender<i64>, Receiver<i64>) = channel();
        let sys_input = send;
        sys_input.send(permutation[0] as i64);
        let mut prev_rec = receive;
        let sys_output;
        let mut handles = vec![];
        let n = 5;
        for i in 0..n {
            let (send, receive): (Sender<i64>, Receiver<i64>) = channel();
            if i != n - 1 {
                send.send(permutation[i + 1] as i64);
            }
            let mut clone = program.clone();
            let mut comp = ChannelComputer {
                receiver: prev_rec,
                sender: send,
            };
            handles.push(thread::spawn(move || {
                let mut iterator = 0;
                run_int_code_on_computer(&mut iterator, &mut clone, &mut comp, false);
            }));
            prev_rec = receive;
        }
        sys_output = prev_rec;
        sys_input.send(0);
        loop {
            match sys_output.recv() {
                Ok(v) => {
                    s = v;
                    sys_input.send(s);
                    if s > max_signal {
                        max_signal = s;
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
    max_signal
}
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let program = load_program(input);
    let part1 = simulate_amp_loop(&program, false);
    let part2 = simulate_amp_loop(&program, true);
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
