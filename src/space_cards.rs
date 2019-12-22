use std::fs;
use regex::Regex;
use std::collections::HashMap;

#[derive(Copy,Clone)]
pub enum OperationType
{
    Reverse,
    Increment,
    Cut
}

#[derive(Copy,Clone)]
pub struct Operation{
    op: OperationType,
    number: i64
}
pub fn undo_op_index(op: Operation,mut index: usize, n: usize)->usize
{
    match op.op {
        OperationType::Reverse => {
            index = (n - 1) - index
        },
        OperationType::Increment => {
            while index%(op.number as usize)!=0
                {
                    index+=n;
                }
            index = index/(op.number as usize);
        }
        OperationType::Cut => {
            index = ((index as i64+op.number+n as i64) as usize)%n;
        }
    }
    index
}
pub fn undo_op(op: Operation,deck: Vec<usize>)->Vec<usize>
{
    let mut new_deck = deck.clone();
    let n =deck.len();
    match op.op{
        OperationType::Reverse=>{
            new_deck.reverse();
        },
        OperationType::Increment=>{ for i in 0..n{
            new_deck[i] = deck[i*(op.number as usize)%n]; } },
        OperationType::Cut => {
            new_deck = deck.into_iter().cycle().skip(((n as i64-op.number) as usize)%n).take(n).collect();
        }
    }
    new_deck
}
pub fn apply_op(op: Operation, mut deck: Vec<usize>) ->Vec<usize>
{
    let n =deck.len();
    match op.op{
        OperationType::Reverse=>{
            for i in 0..n{
                deck[i] = n-1-i;
            }
        },
        OperationType::Increment=>{ for i in 0..n{
            deck[i] = i*(op.number as usize)%n} },
        OperationType::Cut => {
            for i in 0..n{
                deck[i] = (((i as i64 - op.number)+n as i64) as usize)%n;
            }
        }
    }
    deck
}
pub fn unshuffle_index(filename: &str, len:usize, mut index: usize, count: usize,print: bool)
{
    let mut ops = load_instructions(filename);
    ops.reverse();
    let no_ops =ops.len();
    let org_index=index;
    println!("Building hashmap...");
    let mut map: HashMap<usize,usize> = HashMap::new();
    for i in 0..len{
        let mut array_index=i;
        for op in &ops{
            array_index = undo_op_index(*op,array_index,len);
        }
        map.insert(array_index,i);
    }
    println!("Finished hashmap...");
    if print{
        for i in 0..len{
            print!("{} ",i);
        }
        println!();
        for i in 0..len{
            print!("{} ",map.get(&i).unwrap())
        }
        println!();
    }
    let mut iter:usize = 0;
    while iter<count {
        index = *map.get(&index).unwrap();
        iter+=1;
        if iter%100000==0
        {
            println!("{}",iter as f64/count as f64);
        }
    }
    println!("Original index of {}: {}",org_index,index);
    println!();
}

pub fn shuffle_deck(filename: &str, len:usize, mut index: usize, print: bool)
{
    let ops = load_instructions(filename);
    let org_index=index;

    let mut deck: Vec<usize> = vec![0; len];
    for i in 0..deck.len() {
        deck[i]=i;
    }
    for card in &deck{
    print!("{} ",card);
    }
    println!("\nShuffle!");
    for op in &ops {
        deck = apply_op(*op,deck);
    }
    if print{
    for card in &deck{
        print!("{} ",card);
    }
    println!();
    }
    println!("It's rewind time");
    for op in ops.into_iter().rev() {
        index = undo_op_index(op,index,deck.len());
        deck = undo_op(op,deck);
        if print{
            for card in &deck{
                print!("{} ",card);
            }
            println!();
        }
    }
    println!("Original index of {}: {}",org_index,index);
    for card in &deck{
        print!("{} ",card);
    }
    println!();
}
pub fn load_instructions(filename: &str) -> Vec<Operation> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let mut ops: Vec<Operation> = vec![];
    for s in split {
        let space_delimited = s.split(" ").into_iter().collect::<Vec<&str>>();
        let second_last = space_delimited[space_delimited.len()-2];
        let mut op_type = OperationType::Reverse;
        let mut op_number = 0;
        match second_last {
            "cut" => {op_type=OperationType::Cut; op_number=space_delimited.last().unwrap().parse::<i64>().unwrap()},
            "increment" => {op_type=OperationType::Increment; op_number=space_delimited.last().unwrap().parse::<i64>().unwrap()},
            _ => ()
        }
        ops.push(Operation{op:op_type,number:op_number});
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
        shuffle_deck("data/card_instructions.txt",10007,5472,false);
    }
    #[test]
    fn shuffle()  {
        unshuffle_index("data/card_instructions.txt",119315717514047,2020,101741582076661,false);
    }
    #[test]
    fn small_shuffle()  {
        shuffle_deck("data/small_shuffle.txt",10,0,true);
        unshuffle_index("data/small_shuffle.txt",10,0,1,true);
    }
}
