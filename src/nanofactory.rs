use std::collections::HashMap;
use std::fs;

use regex::Regex;

#[derive(Clone)]
pub struct Ore {
    name: String,
    amount: i64,
}

impl Ore {
    fn to_string(&self) -> String {
        return format!("{}({})", self.name, self.amount);
    }
}

pub struct Conversion {
    amount: i64,
    resources: Vec<Ore>,
}

pub fn compute_ore_for_fuel(conversions: &HashMap<String, Conversion>, fuel_amount: i64) -> i64 {
    let mut needed: Vec<Ore> = vec![Ore {
        name: "FUEL".parse().unwrap(),
        amount: fuel_amount,
    }];
    let mut left: HashMap<String, i64> = HashMap::new();
    let mut result: i64 = 0;
    while !needed.is_empty() {
        let ore = needed.pop().unwrap();
        let mut produced = 0;
        if conversions.contains_key(ore.name.as_str()) {
            let conv = conversions.get(ore.name.as_str()).unwrap();
            let mut in_store: i64 = 0;
            if left.contains_key(ore.name.as_str()) {
                in_store = *left.get(ore.name.as_str()).unwrap();
                if ore.amount < in_store {
                    left.insert(ore.name.clone(), in_store - ore.amount);
                } else {
                    left.remove(ore.name.as_str());
                }
            }
            let mut not_in_store = ore.amount - in_store;

            let times = (not_in_store as f32 / conv.amount as f32).ceil() as i64;
            for conv_or in &conv.resources {
                let mut xor = conv_or.clone();
                xor.amount *= times;
                needed.push(xor);
            }
            let produced = times * conv.amount;

            //println!("Making {} of {} ({} required and {} in store)", produced, ore.name, ore.amount, in_store);
            if produced > ore.amount - in_store && produced != 0 {
                let excess = produced - not_in_store;
                if left.contains_key(ore.name.as_str()) {
                    let cval = left.get(ore.name.as_str()).unwrap();
                    left.insert(ore.name, cval + excess);
                } else {
                    left.insert(ore.name, excess);
                }
            }
        } else {
            result += ore.amount;
        }
    }
    return result;
}

pub fn search(conversions: &HashMap<String, Conversion>, i: i64, j: i64) -> i64 {
    if i == j {
        return i;
    }
    let m = (i + j) / 2;
    let ore_needed = compute_ore_for_fuel(&conversions, m);
    if ore_needed <= 1000000000000 {
        return search(conversions, m + 1, j);
    } else {
        return search(conversions, i, m - 1);
    }
}

pub fn compute_max_fuel(filename: &String) -> i64 {
    let conversions = load_conversions(filename);
    return search(&conversions, 0, 10000000);
}

pub fn compute_fuel_ore(filename: &String) -> i64 {
    let conversions = load_conversions(filename);
    return compute_ore_for_fuel(&conversions, 1);
}

pub fn load_conversions(filename: &String) -> HashMap<String, Conversion> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let mut conversions = HashMap::new();
    let rg = Regex::new(r"(\d+ [[:alpha:]]+)").unwrap();
    for s in split {
        let mut ores: Vec<Ore> = vec![];
        for cap in rg.find_iter(s).map(|str| str.as_str()) {
            let ore_split: Vec<&str> = cap.split(" ").collect();
            ores.push(Ore {
                name: ore_split[1].to_string(),
                amount: ore_split[0].parse::<i64>().unwrap(),
            });
        }
        let product = ores.pop().unwrap();
        conversions.insert(
            product.name,
            Conversion {
                resources: ores,
                amount: product.amount,
            },
        );
    }
    return conversions;
}
