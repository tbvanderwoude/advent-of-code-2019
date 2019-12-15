use std::env;
use std::thread;
use std::sync::mpsc::{channel, sync_channel, Receiver, SyncSender, Sender};
use console::Term;
use std::collections::HashMap;
use std::cmp::{max, min};
use rand::Rng;

extern crate rand;
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
        loop {

            if i%10000==0
            {
                self.render();
            }

            let mut mov_instr =0;
            mov_instr= rand::thread_rng().gen_range::<i64,i64,i64>(1,5);
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
                        self.x=next_x;
                        self.y=next_y;
                    }
                    self.map.insert((next_x,next_y),info);
                    break;
                }
            }
            i+=1;
        }
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
