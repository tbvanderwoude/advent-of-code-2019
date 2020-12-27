extern crate petgraph;
extern crate rand;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::{io, thread};

use aoc::intcode::{load_program, run_int_code_on_computer, NetworkComputer};
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let program = load_program(input);
    let mut part1 = -1;
    let mut part2 = -1;
    let mut inputs: Vec<Receiver<i64>> = vec![];
    let mut outputs: Vec<Sender<i64>> = vec![];
    let n = 50;
    for i in 0..n {
        let (comp_out, main_in): (Sender<i64>, Receiver<i64>) = channel();
        let (main_out, comp_in): (Sender<i64>, Receiver<i64>) = channel();
        //Send id
        inputs.push(main_in);
        outputs.push(main_out);
        outputs[i].send(i as i64);
        let mut comp = NetworkComputer {
            receiver: comp_in,
            sender: comp_out,
        };
        let mut prog = program.clone();
        thread::spawn(move || {
            let mut iterator = 0;
            run_int_code_on_computer(&mut iterator, &mut prog, &mut comp, false);
        });
    }
    let mut old_x = 0;
    let mut old_y = 0;
    let mut nat_x = 0;
    let mut nat_y = 0;
    let mut idle_count = 0;
    loop {
        let mut idle = true;
        for i in 0..n {
            let address = inputs[i].try_recv();
            if address.is_ok() {
                idle = false;
                let packet = inputs[i].iter().take(2).collect::<Vec<i64>>();
                if address.unwrap() == 255 {
                    nat_x = packet[0];
                    nat_y = packet[1];
                    if part1 == -1 {
                        part1 = nat_y;
                    }
                } else {
                    outputs[address.unwrap() as usize].send(packet[0]);
                    outputs[address.unwrap() as usize].send(packet[1]);
                }
            }
        }
        if idle {
            if idle_count > 10000 {
                if old_y == nat_y {
                    part2 = nat_y;
                    break;
                }
                outputs[0].send(nat_x);
                outputs[0].send(nat_y);
                old_x = nat_x;
                old_y = nat_y;
                idle_count = 0;
            }
            idle_count += 1;
        } else {
            idle_count = 0;
        }
    }
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
