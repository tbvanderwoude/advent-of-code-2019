use std::env;
use std::thread;
use std::sync::mpsc::{channel, sync_channel, Receiver, SyncSender, Sender};
use console::Term;
use std::collections::HashMap;
use std::cmp::{max, min};
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

extern crate rand;
extern crate petgraph;
use petgraph::{Graph, Undirected};
use petgraph::csr::NodeIndex;
use petgraph::graph::Node;

mod async_intcode;
mod intcode;
mod explorer;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
        return;
    }
    let filename: String = args[1].as_str().parse().unwrap();

    env::set_var("RUST_BACKTRACE", "1");

    let mut program = intcode::load_program(&filename);
    explorer::explore(program);
}
