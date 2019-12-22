use std::fs;


#[derive(Copy,Clone)]
pub enum Operation{ Reverse, Increment(i128), Cut(i128)}

pub fn shuffle_deck(filename: &str, len:i128)
{
    let ops = load_instructions(filename);
    let answer = ops.iter().fold(2019,|pos,&op| match op{
        Operation::Increment(x)=>(pos*x+len)%len,
        Operation::Cut(x)=>(pos-x+len)%len,
        Operation::Reverse=>len-1-pos,
    });
    println!("{}",answer);
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
    use std::error::Error;
    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        load_instructions("data/card_instructions.txt");
        Ok(())
    }
    #[test]
    fn alt_shuffle()  {
        shuffle_deck("data/card_instructions.txt",10007);
    }
    #[test]
    fn small_shuffle()  {
        shuffle_deck("data/small_shuffle.txt",10);
    }
}
