extern crate petgraph;
extern crate rand;

use std::sync::mpsc::{channel,Receiver, Sender};
use std::thread;
use console::Term;

use crate::intcode;
use crate::network_intcode;


pub fn view(mut program: Vec<i64>) ->i64 {
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
        let mut prog = program.clone();
        thread::spawn(move || {
            let mut iterator = 0;
            network_intcode::run_int_code_on_computer(
                &mut iterator,
                &mut prog,
                comp_in,
                comp_out,
                false,
            );
        });
    }
    println!("Finished setting up computers");
    let mut old_x=0;
    let mut old_y=0;
    let mut nat_x = 0;
    let mut nat_y = 0;
    let mut idle_count=0;

    //What is going wrong here?
    //NAT does not get the true last value. It can be any of the values it receives.
    loop {
        let mut idle=true;
        for i in 0..n {
            let address = inputs[i].try_recv();
            if address.is_ok() {
                idle=false;
                let packet = inputs[i].iter().take(2).collect::<Vec<i64>>();
                if address.unwrap() == 255
                {
                    //println!("NAT: ({}, {})",packet[0],packet[1]);
                    nat_x=packet[0];
                    nat_y=packet[1];
                }
                else {
                    //println!("({}, {}) to {} from {}",packet[0],packet[1],address.unwrap(),i);
                    outputs[address.unwrap() as usize].send(packet[0]);
                    outputs[address.unwrap() as usize].send(packet[1]);
                }
            }
        }
        //println!("Idle count: {}",idle_count);
        if idle
        {
            idle_count+=1;
        }
        else {
            idle_count=0;
        }
        if idle_count>10000
        {
            if old_y==nat_y
            {
                //println!("Duplicate Y: {}",nat_y);
                return nat_y;
            }
            outputs[0].send(nat_x);
            outputs[0].send(nat_y);
            old_x=nat_x;
            old_y=nat_y;
            idle_count=0;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test()  {
        println!("{}",view(intcode::load_program("input/intcode/network.txt")));
    }
}
