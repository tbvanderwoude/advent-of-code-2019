use petgraph::Graph;
use std::collections::HashMap;
use std::fs;

pub struct Maze {
    width: usize,
    height: usize,
    map: Vec<char>,
}

impl Maze {
    fn id(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
    fn coords(&self, id: usize) -> (usize, usize) {
        (id % self.width, id / self.width)
    }
    fn get(&self, x: usize, y: usize) -> char {
        self.map[self.id(x, y)]
    }
}

pub fn load_maze(filename: &String) -> Maze
{
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let mut maze: Vec<char> = vec![];
    let mut y = 0;
    let mut x = 0;
    for s in split {
        for c in s.chars() {
            maze.push(c);
        }
        y += 1;
    }
    Maze { width: maze.len() / y, height: y, map: maze }
}

pub fn show_maze(filename: &String)
{
    let maze: Maze = load_maze(filename);
    explore_maze(maze);
}

pub fn explore_maze(maze: Maze)
{
    let mut i: i32 = 0;
    let mut oxygen = (0, 0);
    let mut keys: HashMap<char, (usize, usize)> = HashMap::new();
    let mut doors: HashMap<char, (usize, usize)> = HashMap::new();
    for y in 0..maze.height {
        for x in 0..maze.width {
            if maze.get(x, y).is_lowercase()
            {
                keys.insert(maze.get(x, y), (x, y));
            }
            if maze.get(x, y).is_uppercase()
            {
                doors.insert(maze.get(x, y), (x, y));
            }
        }
    }
    for (k, v) in doors {
        println!("Door {}, is at ({}, {})", k, v.0, v.1);
    }
    for (k, v) in keys {
        println!("Key {}, is at ({}, {})", k, v.0, v.1);
    }
}