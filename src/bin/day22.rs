use std::io;
use std::io::Read;

#[derive(Copy,Clone)]
pub enum Operation{ Reverse, Increment(i128), Cut(i128)}
// Computes parameters for inverse mapping of the applied operations.
// let (increment,offset) = ops_to_inv_func(ops.clone(),len);
// println!("{} -> {}",5472,(5472*increment+offset).rem_euclid(len));
pub fn ops_to_inv_func(ops: Vec<Operation>, len: i128) ->(i128,i128)
{
    let mut offset=0;
    let mut increment =1;
    for op in ops.into_iter() {
        match op{
            Operation::Reverse=>{
                increment = (-increment as i128).rem_euclid(len);
                offset = (offset+increment).rem_euclid(len);
            },
            Operation::Cut(x)=>{
                offset = (offset+x*increment).rem_euclid(len);
            },
            Operation::Increment(x)=>{
                increment = (increment*mod_exp::mod_exp(x,len-2,len)).rem_euclid(len);
            }
        }
    }
    (increment,offset)
}
pub fn part_one(ops: Vec<Operation>) -> i128
{
    let len: i128 =10007;
    ops.iter().fold(2019,|pos,&op| match op{
        Operation::Increment(x)=>(pos*x).rem_euclid(len),
        Operation::Cut(x)=>(pos-x).rem_euclid(len),
        Operation::Reverse=>len-1-pos,
    })
}
pub fn part_two(ops: Vec<Operation>) ->i128
{
    let m: i128 =119315717514047;
    let n: i128 =101741582076661;
    // Should look into modular sometime.
    let (a,b) = ops_to_inv_func(ops, m);
    let term1 = 2020 * mod_exp::mod_exp(a, n, m) % m;
    let tmp = (mod_exp::mod_exp(a, n, m) - 1) * mod_exp::mod_exp(a - 1, m -2, m) % m;
    let term2 = b * tmp % m;
    (term1 + term2) % m
}
pub fn load_instructions(contents: String) -> Vec<Operation> {
    let split = contents.split("\n");
    let mut ops: Vec<Operation> = vec![];
    for s in split {
        let space_delimited = s.split(" ").into_iter().collect::<Vec<&str>>();
        let second_last = space_delimited[space_delimited.len()-2];
        match second_last {
            "cut" => {ops.push(Operation::Cut(space_delimited.last().unwrap().parse::<i128>().unwrap()));},
            "increment" => {ops.push(Operation::Increment(space_delimited.last().unwrap().parse::<i128>().unwrap()));},
            _ => (ops.push(Operation::Reverse))
        }
    }
    return ops;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let operations = load_instructions(input);
    let part1 = part_one(operations.clone());
    let part2 = part_two(operations.clone());
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
