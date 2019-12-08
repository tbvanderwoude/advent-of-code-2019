
fn find_noun_verb(mem:Vec<i32>) -> i32 {
    for i in 0..100 {
        for j in 0..100
            {
                if test_noun_verb(i,j,mem.clone()){
                    return i*100+j;
                }
            }
    }
    return 0;
}
fn test_noun_verb(noun:i32, verb:i32, mem:Vec<i32>)->bool
{
    return run_program(noun, verb, mem)==19690720;
}
fn run_program(noun:i32, verb:i32, mut mem:Vec<i32>)->i32 {
    mem[1]=noun;
    mem[2]=verb;
    let mut i = 0;
    while i < mem.len() {
        match mem[i] {
            1|2 => {
                let arg1 = mem[mem[i+1] as usize];
                let arg2 = mem[mem[i+2] as usize];
                let index = mem[i+3];
                if mem[i]==1
                {
                    mem[index as usize]=arg1+arg2;
                }
                else {
                    mem[index as usize]=arg1*arg2;
                }
                i+=4;
            },
            99 => break,
            _ => println!("DINGDONGYOUROPCODEISWRONG"),
        }
    }
    return mem[0];
}
#[derive(Copy, Clone, PartialEq,Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Copy, Clone, PartialEq, Eq)]
struct Edge {
    p1: Point,
    p2: Point,
}
impl Point{
    fn manhattanNorm(&self) -> i32
    {
        return self.x.abs()+self.y.abs();
    }
    fn manhattanDist(&self, other: Point) -> i32
    {
        return (self.x-other.x).abs()+(self.y-other.y).abs();
    }
}
impl Edge {
    fn intervalIntersect(x1: i32, y1: i32, x2: i32, y2:i32) ->bool
    {
        let mut a1=x1;
        let mut b1=y1;
        let mut a2=x2;
        let mut b2=y2;
        if y1<x1
        {
            swap(&mut a1,&mut b1);
        }
        if y2<x2
        {
            swap(&mut a2,&mut b2);
        }
        return (a1>=a2&&a1<=b2)
            ||(b1>=a2&&b1<=b2)
            ||(a2>=a1&&a2<=b1)
            ||(b2>=a1&&b2<=b1);
    }
    fn intersect(&self, other:Edge) ->bool
    {
        return Edge::intervalIntersect(self.p1.x,self.p2.x,other.p1.x,other.p2.x)
            &&Edge::intervalIntersect(self.p1.y,self.p2.y,other.p1.y,other.p2.y);
    }
}
fn build_wires(contents: String) -> Vec<Vec<Edge>> {
    let mut wires: Vec<Vec<Edge>>=vec![];
    let wireStrs = contents.split("\n");
    for wireStr in wireStrs {
        let edgeStrs = wireStr.split(",");
        let mut oldPoint = Point{x:0, y:0};
        let mut wire: Vec<Edge> = vec![];
        for edgeStr in edgeStrs{
            let (code, diststr)=edgeStr.trim().split_at(1);
            let mut new_point: Point = Point{x:0,y:0};
            let dist: i32 = diststr.parse::<i32>().unwrap();
            match code {
                "U" => new_point=Point{x: oldPoint.x,y:oldPoint.y+dist},
                "D" => new_point=Point{x: oldPoint.x,y:oldPoint.y-dist},
                "L" => new_point=Point{x: oldPoint.x-dist,y:oldPoint.y},
                "R" => new_point=Point{x: oldPoint.x+dist,y:oldPoint.y},
                _ => println!("WUT"),
            }
            let e: Edge = Edge{p1:oldPoint,p2:new_point};
            wire.push(e);
            oldPoint=new_point;
        }
        wires.push(wire);
    }
    return wires;
}
fn find_intersections(wires:Vec<Vec<Edge>>)->Vec<i32> {
    let mut intersections: Vec<i32>=vec![];
    //For every wire
    for i in 0..wires.len() {
        let mut i_wire_cost=0;
        for j in 0..wires[i].len() {
            //For every other wire
            let mut k_wire_cost=0;
            for k in 0..wires.len() {
                if i!=k
                {
                    //For every edge of every other wire
                    for l in 0..wires[k].len() {

                        if wires[i][j].intersect(wires[k][l])
                        {
                            let mut inter = Point{x:wires[i][j].p1.x,y:wires[k][l].p1.y};
                            if wires[i][j].p1.x!=wires[i][j].p2.x
                            {
                                inter=Point{x:wires[k][l].p1.x,y:wires[i][j].p1.y};
                            }
                            intersections.push(i_wire_cost+wires[i][j].p1.manhattanDist(inter)+k_wire_cost+wires[k][l].p1.manhattanDist(inter));
                        }
                        k_wire_cost+=wires[k][l].p1.manhattanDist(wires[k][l].p2);

                    }
                }
            }
            i_wire_cost+=wires[i][j].p1.manhattanDist(wires[i][j].p2);
        }
    }
    intersections.sort();
    return intersections;
}

fn run_str_program(mut mem:Vec<String>) {
    let mut i:usize = 0;
    while i < mem.len() {
        let len=mem[i].len();
        let mut opcode:i32=0;
        let mut mode1:i32=0;
        let mut mode2:i32=0;
        let mut mode23:i32=0;
        if len>=1
        {
            opcode=(mem[i]).parse::<i32>().unwrap();
            if len>=3
            {
                opcode=(&mem[i][1..]).parse::<i32>().unwrap();
                mode1=(&mem[i][..1]).parse::<i32>().unwrap();
                if len>=4
                {
                    opcode=(&mem[i][2..]).parse::<i32>().unwrap();
                    mode1=(&mem[i][1..2]).parse::<i32>().unwrap();
                    mode2=(&mem[i][..1]).parse::<i32>().unwrap();
                }
            }
        }
        println!("Opcode {0} (mode1 {1} mode2 {2})",opcode,mode1,mode2);
        match opcode {
            1|2 => {
                let mut arg1:i32=mem[i+1].parse::<i32>().unwrap();
                let mut arg2:i32=mem[i+2].parse::<i32>().unwrap();
                if mode1==0
                {
                    arg1 = mem[arg1 as usize].parse::<i32>().unwrap();
                }
                if mode2==0
                {
                    arg2 = mem[arg2 as usize].parse::<i32>().unwrap();
                }
                let index:i32 = mem[i+3].parse::<i32>().unwrap();
                if opcode==1
                {
                    mem[index as usize]=(arg1+arg2).to_string();
                }
                else {
                    mem[index as usize]=(arg1*arg2).to_string();
                }
                i+=4;
            },
            3|4 => {
                let mut arg1:usize=mem[i+1].parse::<i32>().unwrap() as usize;
                if(opcode==3)
                {
                    let mut ret = String::new();
                    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
                    ret=ret.trim().to_string();
                    println!("Writing {0} to {1}",ret,arg1);
                    mem[arg1]=ret;
                }
                if(opcode==4)
                {
                    println!("{}",mem[arg1])
                }
                i+=2;
            },
            5|6|7|8 =>
                {
                    let mut arg1:i32=mem[i+1].parse::<i32>().unwrap();
                    let mut arg2:i32=mem[i+2].parse::<i32>().unwrap();
                    if mode1==0
                    {
                        arg1 = mem[arg1 as usize].parse::<i32>().unwrap();
                    }
                    if mode2==0
                    {
                        arg2 = mem[arg2 as usize].parse::<i32>().unwrap();
                    }
                    if(opcode==7||opcode==8)
                    {
                        let index:i32 = mem[i+3].parse::<i32>().unwrap();
                        mem[index as usize]=((opcode==7&&arg1<arg2||opcode==8&&arg1==arg2)as i32).to_string();
                        i+=4;
                    }
                    else
                    {
                        if(opcode==5&&arg1!=0||opcode==6&&arg1==0)
                        {
                            i=arg2 as usize;
                        }
                        else {
                            i+=3;
                        }
                    }
                }
            99 => break,
            _ => {i+=1},
        }
    }
}