use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::time::Duration;
use std::{fs, thread};

use petgraph::Graph;
use queues::*;
use std::cmp::{min, Ordering};
use std::fmt::Binary;
use std::ops::Mul;

pub struct Maze {
    width: usize,
    height: usize,
    map: Vec<Vec<char>>,
    start: (usize, usize),
    doors: HashMap<char, (usize, usize)>,
    keys: HashMap<char, (usize, usize)>,
}

impl Maze {
    fn coords(&mut self, id: usize) -> (usize, usize) {
        (id % self.width, id / self.width)
    }
    fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];
        if self.valid(x - 1, y) {
            neighbours.push((x - 1, y));
        }
        if self.valid(x + 1, y) {
            neighbours.push((x + 1, y));
        }
        if self.valid(x, y - 1) {
            neighbours.push((x, y - 1));
        }
        if self.valid(x, y + 1) {
            neighbours.push((x, y + 1));
        }
        neighbours
    }
    fn has_key(&self, x: usize, y: usize) -> bool {
        self.get(x, y) >= 'a' && self.get(x, y) <= 'z'
    }
    fn has_door(&self, x: usize, y: usize) -> bool {
        self.get(x, y) >= 'A' && self.get(x, y) <= 'Z'
    }
    fn show(&self) {
        for y in 0..self.height {
            println!("{}", self.map[y].iter().collect::<String>());
        }
        println!("doors: {}, keys: {}", &self.doors.len(), &self.keys.len());
        for (k, v) in &self.doors {
            //println!("Door {}, is at ({}, {})",k,v.0,v.1);
        }
        for (k, v) in &self.keys {
            //println!("Key {}, is at ({}, {})",k,v.0,v.1);
        }
    }

    fn valid(&self, x: usize, y: usize) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height && self.get(x, y) != '#'
    }
    fn get(&self, x: usize, y: usize) -> char {
        self.map[y][x]
    }
    fn set(&mut self, x: usize, y: usize, c: char) {
        self.map[y][x] = c;
    }
}

/*
This is terrible, will give the correct answer with a minor modification but is O(n!)
while key_q.size() > 0 {
    let (key, cost, key_str) = key_q.remove().unwrap();
    let map = key_map.get(&key).unwrap();
    println!("Key: {}", key_str);
    for (next_key, (next_cost, lock_str)) in map {
        if !in_string(key_str, *next_key) && locks_keys(*lock_str, key_str) {
            //Prints non-trivial paths
            if *lock_str > 0 {
                println!("We can go from {} to {}", key, next_key);
            }
            let new_key_str = add_key(key_str, *next_key);
            if new_key_str == ultimate_str {
                println!("Cost: {}", cost);
                break;
            }
            key_q.add((*next_key, next_cost + cost, new_key_str));
        }
    }
}
*/
pub fn load_maze(filename: &String) -> Maze {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let mut maze: Vec<Vec<char>> = vec![];
    let mut y = 0;
    let mut x = 0;
    for s in split {
        maze.push(vec![]);
        for c in s.chars() {
            maze[y].push(c);
        }
        y += 1;
    }
    Maze {
        width: maze[0].len(),
        height: y,
        map: maze,
        keys: HashMap::new(),
        doors: HashMap::new(),
        start: (0, 0),
    }
}

pub fn show_maze(filename: &String) {
    let maze: Maze = load_maze(filename);
    explore_maze(maze);
}

fn resolve_dist(
    lineage: &HashMap<(usize, usize), (usize, usize)>,
    mut node: (usize, usize),
    start: (usize, usize),
) -> usize {
    let mut dist = 0;
    while lineage.contains_key(&node) && node != start {
        node = *lineage.get(&node).unwrap();
        dist += 1;
    }
    dist
}

fn resolve_dist_keys(
    lineage: &HashMap<(usize, usize), (usize, usize)>,
    maze: &Maze,
    mut node: (usize, usize),
    start: (usize, usize),
) -> (usize, usize) {
    let mut dist = 0;
    let mut string = 0;
    while lineage.contains_key(&node) && node != start {
        node = *lineage.get(&node).unwrap();
        if maze.has_door(node.0, node.1) {
            string = add_key(string, maze.get(node.0, node.1).to_ascii_lowercase());
        }
        dist += 1;
    }
    //println!("bitstring:  {}",string);
    (dist, string)
}

pub fn in_string(str: usize, key: char) -> bool {
    str & (1 << (key as usize - 'a' as usize)) != 0
}

pub fn add_key(str: usize, key: char) -> usize {
    str | (1 << (key as usize - 'a' as usize))
}
pub fn locks_keys(locks: usize, keys: usize) -> bool {
    locks & keys == locks
}
pub fn bfs(key_string: usize, maze: &Maze, key: char) -> HashMap<char, (usize, usize)> {
    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut costs: HashMap<char, (usize, usize)> = HashMap::new();
    let mut q: Queue<(usize, usize)> = Queue::new();
    let start = *maze.keys.get(&key).unwrap();
    q.add(start);
    while q.size() > 0 {
        let node = q.remove().unwrap();
        let neighbours = maze.neighbours(node.0, node.1);
        for (x, y) in neighbours {
            if !parent.contains_key(&(x, y)) {
                parent.insert((x, y), node);
                q.add((x, y));
                if maze.has_key(x, y) && !in_string(key_string, maze.get(x, y)) {
                    costs.insert(
                        maze.get(x, y),
                        resolve_dist_keys(&parent, &maze, (x, y), start),
                    );
                }
            }
        }
    }
    costs
}

pub fn distToGetKeys(
    key: char,
    keys: usize,
    key_map: &HashMap<char, HashMap<char, (usize, usize)>>,
    cache: &mut HashMap<(char, usize), usize>,
) -> usize {
    //println!("Key: {:b}",keys);
    if cache.contains_key(&(key, keys)) {
        //println!("Used cache for ({}, {})",key,keys);
        *cache.get(&(key, keys)).unwrap()
    } else {
        let mut min_dist = std::usize::MAX;
        for (next_key, (cost, locks)) in key_map.get(&key).unwrap() {
            if !in_string(keys, *next_key) && locks_keys(*locks, keys) {
                let mut dist =
                    cost + distToGetKeys(*next_key, add_key(keys, *next_key), key_map, cache);
                min_dist = min_dist.min(dist);
            }
        }
        if min_dist == std::usize::MAX {
            min_dist = 0;
        }
        cache.insert((key, keys), min_dist);
        if cache.contains_key(&(key, keys)) {
            //println!("Now has cache for ({}, {})", key, keys);
        }
        return min_dist;
    }
}

pub fn explore_maze(mut maze: Maze) {
    let i: i32 = 0;
    for y in 0..maze.height {
        for x in 0..maze.width {
            if maze.get(x, y).is_lowercase() {
                maze.keys.insert(maze.get(x, y), (x, y));
            }
            if maze.get(x, y).is_uppercase() {
                maze.doors.insert(maze.get(x, y), (x, y));
            }
            if maze.get(x, y) == '@' {
                maze.set(x, y, '.');
                maze.start = (x, y);
            }
        }
    }
    let mut costs: HashMap<char, usize> = HashMap::new();
    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut q: Queue<(usize, usize)> = Queue::new();
    q.add(maze.start);

    while q.size() > 0 {
        let node = q.remove().unwrap();
        let neighbours = maze.neighbours(node.0, node.1);
        for (x, y) in neighbours {
            if !parent.contains_key(&(x, y)) {
                parent.insert((x, y), node);
                if maze.has_key(x, y) {
                    costs.insert(maze.get(x, y), resolve_dist(&parent, (x, y), maze.start));
                    q.add((x, y));
                } else if maze.has_door(x, y) {
                } else {
                    q.add((x, y));
                }
            }
        }
    }
    //maze.show();
    let mut key_map: HashMap<char, HashMap<char, (usize, usize)>> = HashMap::new();
    let mut ultimate_str = 0;
    for (c, p) in &maze.keys {
        key_map.insert(*c, bfs(add_key(0, *c), &maze, *c));
        ultimate_str = add_key(ultimate_str, *c);
    }
    //println!("Ultimate string: {:b}", ultimate_str);

    let mut key_keys_map: HashMap<(char, usize), usize> = HashMap::new();
    let mut min_cost: usize = 10000000;

    let mut dist: HashMap<(char, usize), usize> = HashMap::new();
    for (c, map) in &costs {
        let c_dist =
            costs.get(c).unwrap() + distToGetKeys(*c, add_key(0, *c), &key_map, &mut key_keys_map);
        min_cost = min_cost.min(c_dist);
    }
    println!("Answer: {}", min_cost);
    /*
    while key_q.size() > 0 {
        let c = key_q.remove().unwrap();
        for (c, d) in key_map.get(c).unwrap() {}
    }*/
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        show_maze(&"data/maze.txt".to_string());
        Ok(())
    }
    #[test]
    fn more_keys_than_doors() {
        let keys = add_key(add_key(add_key(0, 'd'), 'b'), 'c');
        let doors = add_key(add_key(0, 'b'), 'c');
        assert!(locks_keys(doors, keys));
    }
    #[test]
    fn enough_keys() {
        let keys = add_key(add_key(0, 'b'), 'c');
        let doors = add_key(add_key(0, 'b'), 'c');
        assert!(locks_keys(doors, keys));
    }
    #[test]
    fn too_few_keys() {
        let keys = add_key(add_key(0, 'b'), 'c');
        let doors = add_key(add_key(add_key(0, 'd'), 'b'), 'c');
        assert!(!locks_keys(doors, keys));
    }
}