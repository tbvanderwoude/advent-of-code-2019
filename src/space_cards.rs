use std::fs;


#[derive(Copy,Clone)]
pub enum Operation{ Reverse, Increment(i128), Cut(i128)}
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
pub fn part_one(filename: &str) -> i128
{
    let len: i128 =10007;
    let ops = load_instructions(filename);
    let (a,b) = ops_to_inv_func(ops.clone(),len);
    println!("Increment: {}, Offset: {}",a,b);
    println!("{} -> {}",5472,(5472*a+b).rem_euclid(len));
    ops.iter().fold(2019,|pos,&op| match op{
        Operation::Increment(x)=>(pos*x).rem_euclid(len),
        Operation::Cut(x)=>(pos-x).rem_euclid(len),
        Operation::Reverse=>len-1-pos,
    })
}
pub fn part_two(filename: &str) ->i128
{
    let M: i128 =119315717514047;
    let N: i128 =101741582076661;
    let ops = load_instructions(filename);
    let (a,b) = ops_to_inv_func(ops,M);
    //The nature of these formulae should be looked into later:
    //https://www.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbnkaju/?context=3

    let term1 = 2020 * mod_exp::mod_exp(a, N, M) % M;
    let tmp = (mod_exp::mod_exp(a, N, M) - 1) * mod_exp::mod_exp(a - 1, M-2, M) % M;
    let term2 = b * tmp % M;
    (term1 + term2) % M
}
pub fn load_instructions(filename: &str) -> Vec<Operation> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test()  {
        assert_eq!(part_one("data/card_instructions.txt"),5472);
    }
    #[test]
    fn alt_shuffle()  {
        assert_eq!(part_two("data/card_instructions.txt"),64586600795606);
    }
}
