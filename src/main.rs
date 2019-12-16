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

pub struct Explorer
{
    in_channel: Receiver<i64>,
    out_channel: Sender<i64>,
    term: console::Term,
    x: i64,
    y: i64,
    map: HashMap<(i64, i64), i64>,
}
impl Explorer
{
    fn explore(&mut self)
    {
        let mut i: i32=0;
        let mut oxygen=(0,0);
        let mut breadcrumbs: Vec<i64> = vec![];
        loop {
            let mut mov_instr =0;
            if !self.map.contains_key(&(self.x,self.y-1))
            {
                mov_instr=1;
                breadcrumbs.push(2);
            }
            else if !self.map.contains_key(&(self.x,self.y+1))
            {
                mov_instr=2;
                breadcrumbs.push(1);
            }
            else if !self.map.contains_key(&(self.x+1,self.y))
            {
                mov_instr=4;
                breadcrumbs.push(3);
            }
            else if !self.map.contains_key(&(self.x-1,self.y))
            {
                mov_instr=3;
                breadcrumbs.push(4);
            }
            else {
                let last_move=breadcrumbs.pop();
                if last_move.is_none()
                {
                    println!("Finished exploring");
                    self.render();
                    break;
                }
                else
                {
                    mov_instr = last_move.unwrap();
                }
            }
            self.out_channel.send(mov_instr);
            loop {
                let res = self.in_channel.recv();
                if res.is_ok()
                {
                    let info = res.unwrap();
                    //println!("Received info: {}",info);
                    let mut next_x= self.x;
                    let mut next_y = self.y;
                    match mov_instr {
                        1 => next_y-=1,
                        2 => next_y+=1,
                        3 => next_x-=1,
                        4 => next_x+=1,
                        _ => ()
                    }

                    if info!=0
                    {
                        if info==2
                        {
                            oxygen=(next_x,next_y);
                            println!("Steps to oxygen: {}",breadcrumbs.len());
                        }
                        self.x=next_x;
                        self.y=next_y;
                    }
                    else {
                        breadcrumbs.pop();
                    }
                    self.map.insert((next_x,next_y),info);
                    break;
                }
            }
            i+=1;
        }
        println!("Let's build a graph!");
        let mut nodes = HashMap::new();
        let mut g: Graph<(i64,i64),f64,Undirected> = Graph::new_undirected();
        for (k, v) in self.map.iter() {
            if *v!=0
            {
                nodes.insert(k,g.add_node(*k));
            }
        }
        let oxygen_node = *(nodes.get(&oxygen).unwrap());
            for ((x,y), v) in nodes.iter() {
            if nodes.contains_key(&(*x - 1, *y))
            {
                g.add_edge(*(nodes.get(&(*x, *y)).unwrap()), *(nodes.get(&(*x - 1, *y)).unwrap()), 1f64);
            }
            if nodes.contains_key(&(*x + 1, *y))
            {
                g.add_edge(*(nodes.get(&(*x, *y)).unwrap()), *(nodes.get(&(*x + 1, *y)).unwrap()), 1f64);
            }
            if nodes.contains_key(&(*x, *y-1))
            {
                g.add_edge(*(nodes.get(&(*x, *y)).unwrap()), *(nodes.get(&(*x, *y - 1)).unwrap()), 1f64);
            }
            if nodes.contains_key(&(*x, *y+1))
            {
                g.add_edge(*(nodes.get(&(*x, *y)).unwrap()), *(nodes.get(&(*x, *y+1)).unwrap()), 1f64);
            }
        }
        let mut max_dist:f64 = -1f64;
        for x in petgraph::algo::bellman_ford(&g,oxygen_node).unwrap().0{
            if x>max_dist
            {
                max_dist=x;
            }
        }

        println!("{:?}",max_dist);
    }
    fn render(&mut self)
    {
        self.term.clear_screen();
        if !self.map.is_empty()
        {
            let maxX = *self.map.keys().map(|(a,b)|a).max().unwrap()+3;
            let maxY = *self.map.keys().map(|(a,b)|b).max().unwrap()+3;
            let minX = *self.map.keys().map(|(a,b)|a).min().unwrap()-3;
            let minY = *self.map.keys().map(|(a,b)|b).min().unwrap()-3;
            let w = (maxX-minX) as usize;
            let h = (maxY-minY) as usize;

            for y in minY..maxY {
                let mut line: Vec<char>=vec!['#';w+2];
                for x in minX..maxX {
                    if self.map.contains_key(&(x, y))
                    {
                        match *self.map.get(&(x, y)).unwrap() {
                            0 => line[(x-minX) as usize]='■',
                            1 => line[(x-minX) as usize]=' ',
                            2 => line[(x-minX) as usize]='X',
                            _ => line[(x-minX) as usize]='#',
                        }
                        if self.x==x&&self.y==y
                        {
                            line[(x-minX) as usize]='@';
                        }
                        else if x==0&&y==0
                        {
                            line[(x-minX) as usize]='$';
                        }
                    }
                }
                println!("{}",line.iter().collect::<String>());
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
        return;
    }
    let filename: String = args[1].as_str().parse().unwrap();

    env::set_var("RUST_BACKTRACE", "1");
    /*
    robot::paint_using_robot(program);
    */

    let (computerOut, mainIn):(Sender<i64>, Receiver<i64>) = channel();
    let (mainOut, computerIn): (Sender<i64>, Receiver<i64>) = channel();
    let mut explorer: Explorer = Explorer{in_channel: mainIn, out_channel: mainOut, term: Term::stdout(),map: HashMap::new(),x:0,y:0};
    thread::spawn(move ||
        {
            let mut iterator = 0;
            let mut program:Vec<i64> = intcode::load_program(&filename);
            async_intcode::run_int_code_on_computer(&mut iterator,&mut program,computerIn,computerOut,false);
        }
    );
    explorer.explore();
    /*
    let (tx, rx) = channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move|| {
            println!("Sending {}",i);
            tx.send(i).unwrap();
        });
    }

    for _ in 0..10 {
        let j = rx.recv().unwrap();
        println!("Received {}",j);
        assert!(0 <= j && j < 10);
    }
    */
}
