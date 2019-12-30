use std::collections::{HashMap, HashSet};
use std::fs;

fn read_orbits(filename: &str) ->(HashMap<String,String>, HashSet<String>)
{
    let mut orbit_map: HashMap<String,String> = HashMap::new();
    let mut planet_set: HashSet<String> = HashSet::new();
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = contents.split("\n");

    for s in lines {
        let line_parts: Vec<&str> = s.split(")").collect();
        orbit_map.insert(line_parts[1].to_string(),line_parts[0].to_string());
        planet_set.insert(line_parts[0].to_string());
        planet_set.insert(line_parts[1].to_string());
    }
    (orbit_map, planet_set)
}
fn count_orbits(filename: &str) -> i32{
    let (map, planets) = read_orbits(filename);
    let mut orbits=0;
    for mut planet in planets{
        while map.contains_key(&planet){
            orbits+=1;
            planet = map.get(&planet).unwrap().to_string();
        }
    }
    orbits
}
fn build_stack(mut planet: String, map: &HashMap<String,String>)->Vec<String>{
    let mut stack=vec![];
    while map.contains_key(&planet){
        planet=map.get(&planet).unwrap().to_string();
        stack.push(planet.clone());
    }
    stack
}
fn dist_to_santa(filename: &str) -> i32{
    let (map, planets) = read_orbits(filename);
    let mut orbits=0;
    let mut santa_stack = build_stack(String::from("SAN"),&map);
    let mut my_stack = build_stack(String::from("YOU"),&map);
    while santa_stack.last().unwrap()==my_stack.last().unwrap(){
        santa_stack.pop();
        my_stack.pop();
    }
    (my_stack.len() + santa_stack.len()) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_orbits() {
        assert_eq!(158090, count_orbits("data/greater_universe.txt"));
    }
    #[test]
    fn dist() {
        assert_eq!(241, dist_to_santa("data/greater_universe.txt"));
    }
}