use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use console::Term;
use petgraph::{Graph, Undirected};

use aoc::computer::ChannelComputer;
use aoc::intcode::{load_program, run_int_code_on_computer};

extern crate petgraph;
extern crate rand;

pub struct Explorer {
    in_channel: Receiver<i64>,
    out_channel: Sender<i64>,
    term: console::Term,
    oxygen: (i64, i64),
    x: i64,
    y: i64,
    map: HashMap<(i64, i64), i64>,
}

impl Explorer {
    fn explore(&mut self) -> usize {
        let mut i: i32 = 0;
        let mut moves_to_oxygen = 0;
        let mut breadcrumbs: Vec<i64> = vec![];
        loop {
            let mut mov_instr = 0;
            if !self.map.contains_key(&(self.x, self.y - 1)) {
                mov_instr = 1;
                breadcrumbs.push(2);
            } else if !self.map.contains_key(&(self.x, self.y + 1)) {
                mov_instr = 2;
                breadcrumbs.push(1);
            } else if !self.map.contains_key(&(self.x + 1, self.y)) {
                mov_instr = 4;
                breadcrumbs.push(3);
            } else if !self.map.contains_key(&(self.x - 1, self.y)) {
                mov_instr = 3;
                breadcrumbs.push(4);
            } else {
                let last_move = breadcrumbs.pop();
                if last_move.is_none() {
                    break;
                } else {
                    mov_instr = last_move.unwrap();
                }
            }
            self.out_channel.send(mov_instr);
            loop {
                let res = self.in_channel.recv();
                if res.is_ok() {
                    let info = res.unwrap();
                    let mut next_x = self.x;
                    let mut next_y = self.y;
                    match mov_instr {
                        1 => next_y -= 1,
                        2 => next_y += 1,
                        3 => next_x -= 1,
                        4 => next_x += 1,
                        _ => (),
                    }

                    if info != 0 {
                        if info == 2 {
                            self.oxygen = (next_x, next_y);
                            moves_to_oxygen = breadcrumbs.len();
                        }
                        self.x = next_x;
                        self.y = next_y;
                    } else {
                        breadcrumbs.pop();
                    }
                    self.map.insert((next_x, next_y), info);
                    break;
                }
            }
            i += 1;
        }
        moves_to_oxygen
    }
    fn spread_oxygen(&self) -> usize {
        let mut nodes = HashMap::new();
        let mut g: Graph<(i64, i64), f64, Undirected> = Graph::new_undirected();
        for (k, v) in self.map.iter() {
            if *v != 0 {
                nodes.insert(k, g.add_node(*k));
            }
        }
        let oxygen_node = *(nodes.get(&self.oxygen).unwrap());
        for ((x, y), _v) in nodes.iter() {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                if nodes.contains_key(&(*x + dx, *y + dy)) {
                    g.add_edge(
                        *(nodes.get(&(*x, *y)).unwrap()),
                        *(nodes.get(&(*x + dx, *y + dy)).unwrap()),
                        1f64,
                    );
                }
            }
        }
        petgraph::algo::bellman_ford(&g, oxygen_node)
            .unwrap()
            .0
            .iter()
            .map(|&x| x as usize)
            .max()
            .unwrap()
    }

    fn render(&mut self) {
        self.term.clear_screen();
        if !self.map.is_empty() {
            let max_x = self.map.keys().max_by_key(|x| x.0).unwrap().0 + 3;
            let max_y = self.map.keys().max_by_key(|x| x.1).unwrap().1 + 3;
            let min_x = self.map.keys().min_by_key(|x| x.0).unwrap().0 - 3;
            let min_y = self.map.keys().min_by_key(|x| x.1).unwrap().1 - 3;
            let w = (max_x - min_x) as usize;
            for y in min_y..max_y {
                let mut line: Vec<char> = vec!['#'; w + 2];
                for x in min_x..max_x {
                    if self.map.contains_key(&(x, y)) {
                        line[(x - min_x) as usize] = match *self.map.get(&(x, y)).unwrap() {
                            0 => '■',
                            1 => ' ',
                            2 => 'X',
                            _ => '#',
                        };
                        if self.x == x && self.y == y {
                            line[(x - min_x) as usize] = '@';
                        } else if x == 0 && y == 0 {
                            line[(x - min_x) as usize] = '$';
                        }
                    }
                }
                println!("{}", line.iter().collect::<String>());
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut program = load_program(input);

    let (comp_out, main_in): (Sender<i64>, Receiver<i64>) = channel();
    let (main_out, comp_in): (Sender<i64>, Receiver<i64>) = channel();
    let mut explorer: Explorer = Explorer {
        in_channel: main_in,
        out_channel: main_out,
        oxygen: (0, 0),
        term: Term::stdout(),
        map: HashMap::new(),
        x: 0,
        y: 0,
    };
    let mut comp = ChannelComputer {
        receiver: comp_in,
        sender: comp_out,
    };
    thread::spawn(move || {
        let mut iterator = 0;
        run_int_code_on_computer(&mut iterator, &mut program, &mut comp, false);
    });
    let part1 = explorer.explore();
    let part2 = explorer.spread_oxygen();
    explorer.render();
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
