use std::collections::HashMap;
use crate::intcode::Computer;

enum RobotDir
{
    UP=0,
    RIGHT=1,
    DOWN=2,
    LEFT=3
}
pub struct Robot{
    pub dir: i64,
    pub map: HashMap<(i64,i64),i64>,
    pub paint: bool,
    pub x: i64,
    pub y: i64
}
impl Computer for Robot{
    fn input(&mut self) ->i64 {
        let mut color:i64=0;
        if self.map.contains_key(&(self.x,self.y))
        {
            color = *self.map.get(&(self.x,self.y)).unwrap();
        }
        if color==1
        {
            println!("It is white at ({0}, {1})",self.x,self.y);
        }
        else {
            println!("It is black at ({0}, {1})",self.x,self.y);
        }
        return color;
    }
    fn output(&mut self,num:i64) {
        if self.paint
        {
            self.map.insert((self.x,self.y),num);
            if num==1
            {
                println!("Painted ({0}, {1}) white",self.x,self.y);
            }
            else {
                println!("Painted ({0}, {1}) black",self.x,self.y);
            }
            self.paint=false;
        }
        else {
            self.dir+=num*2-1;
            if self.dir<0
            {
                self.dir+=4;
            }
            if self.dir>3
            {
                self.dir-=4;
            }
            println!("Dir: {0}",self.dir);
            match self.dir
                {
                    0 => {self.y-=1; println!("Moved up")},
                    1 => {self.x+=1; println!("Moved right")},
                    2 => {self.y+=1; println!("Moved down")},
                    3 => {self.x-=1; println!("Moved left")},
                    _ => ()
                }
            self.paint=true;
        }
    }
}
