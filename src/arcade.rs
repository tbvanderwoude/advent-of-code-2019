use std::collections::HashMap;
use std::io::Read;
use crate::intcode;
use crate::intcode::Computer;
use std::ops::Add;
use console::Term;
use std::thread;
use std::time::Duration;

pub struct Cabinet {
    pub term : console::Term,
    pub map: HashMap<(i64, i64), i64>,
    pub new_x: i64,
    pub new_y: i64,
    pub paddle_x: i64,
    pub paddle_y: i64,
    pub ball_x: i64,
    pub ball_y: i64,
    pub game_exiting: bool
}

pub fn render_screen(mut program: Vec<i64>)
{
    let mut counter: usize = 0;
    let mut arcade: Cabinet = Cabinet { term:Term::stdout(), map: HashMap::new(), new_x: -42,new_y:-42 , game_exiting:false,ball_x:-1,ball_y:-1,paddle_x:-1,paddle_y:-1};
    program[0]=2;
    println!("{}", intcode::run_int_code_on_computer(&mut counter, &mut program, &mut arcade, false));
    println!("{}", arcade.map.iter().map(|(&k,&v)|v).filter(|v|*v==2).fold(0,|a,b| a+b/2));

}

impl Cabinet{

    fn render(&mut self)
    {
        self.term.clear_screen();
        for y in 0..20 {
            let mut line: [char;40]=[' '; 40];
            for x in 0..40 {
                if self.map.contains_key(&(x, y))
                {
                    match *self.map.get(&(x, y)).unwrap() {
                        1|2 => line[x as usize]='■',
                        3 =>line[x as usize]='_',
                        4 =>line[x as usize]='o',
                        _ => ()
                    }
                }
            }
            println!("{}",line.iter().collect::<String>());
        }
    }
}
impl Computer for Cabinet {
    fn input(&mut self) -> i64 {
        self.render();
        thread::sleep(Duration::from_millis(20));
        /*
        let mut input: Option<char> = None;
        while(input.is_none()) {
                input = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as char);
        }
        match input.unwrap() {
            'd' => return 1,
            'a' => return -1,
            _ => return 0
        }*/
        if self.paddle_x<self.ball_x
        {
            return 1;
        }
        if self.paddle_x>self.ball_x
        {
            return -1;
        }
        return 0;
    }
    fn output(&mut self, num: i64) {
        if self.new_x==-42
        {
            self.new_x=num;
        }
        else if self.new_y==-42
        {
            self.new_y=num;
        }
        else {
            if self.new_x==-1&&self.new_y==0
            {
                println!("The player scored {} points", num);
                self.new_x=-42;
                self.new_y=-42;
            }
            else {
                if num==3
                {
                    self.paddle_x=self.new_x;
                    self.paddle_y=self.new_y;
                }
                if num==4
                {
                    self.ball_x=self.new_x;
                    self.ball_y=self.new_y;
                }
                self.map.insert((self.new_x, self.new_y), num);
                self.new_x=-42;
                self.new_y=-42;
            }
        }

    }
}
