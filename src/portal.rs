use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::time::Duration;
use std::{fs, thread};

use petgraph::Graph;
use queues::*;
use std::cmp::{min, Ordering};
use std::fmt::Binary;
use std::ops::Mul;

#[derive(Clone)]
pub struct Maze {
    width: usize,
    height: usize,
    map: Vec<Vec<char>>,
    start: (usize, usize),
    tunnels: HashMap<(usize,usize),(usize,usize)>
}

impl Maze {
    fn neigh_nodes(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
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
        if self.get(x,y)=='p'
        {
            neighbours.push(*self.tunnels.get(&(x,y)).unwrap())
        }
        neighbours
    }
    fn show(&self) {
        for y in 0..self.height {
            println!("{}", self.map[y].iter().collect::<String>());
        }
    }
    fn has_letter(&self, x:usize, y: usize) ->bool
    {
        self.get(x,y) >= 'A'&&self.get(x,y) <= 'Z'
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

pub fn load_maze(filename: &String) -> Maze {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let mut maze: Vec<Vec<char>> = vec![];
    let mut y = 0;
    for s in split {
        maze.push(vec![]);
        for c in s.chars() {
            maze[y].push(c);
        }
        y += 1;
    }
    let mut x = maze.iter().map(|vec|vec.len()).max().unwrap();
    for i in 0..maze.len(){
        while maze[i].len()<x {
            maze[i].push(' ');
        }
    }
    Maze {
        width: x,
        height: y,
        map: maze,
        start: (0, 0),
        tunnels: HashMap::new()
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
pub fn explore_maze(mut maze: Maze) {
    let mut temp_map: HashMap<String,(usize,usize)> = HashMap::new();
    let mut start = (0,0);
    let mut end = (0,0);
    for y in 1..maze.height{
        for x in 1..maze.width {
            if maze.get(x,y)=='.'
            {
                for neigh in maze.neighbours(x,y){
                    if maze.has_letter(neigh.0,neigh.1)
                    {
                        let mut portal_name: String = String::from("");
                        for letter_neigh in maze.neighbours(neigh.0,neigh.1){
                            if maze.has_letter(letter_neigh.0,letter_neigh.1)
                            {
                                if letter_neigh.0<neigh.0||letter_neigh.1<neigh.1
                                {
                                    portal_name.push(maze.get(letter_neigh.0,letter_neigh.1));
                                    portal_name.push(maze.get(neigh.0,neigh.1));
                                }
                                else {
                                    portal_name.push(maze.get(neigh.0,neigh.1));
                                    portal_name.push(maze.get(letter_neigh.0,letter_neigh.1));
                                }
                                maze.set(letter_neigh.0,letter_neigh.1,'#');
                            }
                        }
//                        println!("({}, {}) has a portal: {}",x,y,portal_name);
                        maze.set(neigh.0,neigh.1,'#');
                        if portal_name=="AA"
                        {
                            start = (x,y);
                        }
                        else if portal_name=="ZZ" {
                            end = (x,y);
                        }
                        else if temp_map.contains_key(&portal_name)
                        {
                            let other_pos=*temp_map.get(&portal_name).unwrap();
                            maze.tunnels.insert(other_pos, (x, y));
                            maze.tunnels.insert( (x, y),other_pos);
                            maze.set(other_pos.0,other_pos.1,'p');
                            maze.set(x,y,'p');
                        }
                        else
                        {
                            temp_map.insert(portal_name,(x,y));
                        }
                    }
                }
            }
        }
    }
    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut q: Queue<(usize, usize)> = Queue::new();
    q.add(start);
    while q.size() > 0 {
        let node = q.remove().unwrap();
        let neighbours = maze.neighbours(node.0, node.1);
        for (x, y) in neighbours {
            if !parent.contains_key(&(x, y)) {
                parent.insert((x, y), node);
                q.add((x, y));
                if x==end.0&&y==end.1
                {
                    println!("Dist to end: {}",resolve_dist(&parent,end,start))
                }
//                if bx==x&&by==y
//                {
//                }
            }
        }
    }
//    maze.show();
//    for ((ax,ay),(bx,by)) in &maze.tunnels {
//        println!("({}, {}) <-> ({}, {})",*ax,*ay,*bx,*by);
//    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        show_maze(&"data/portalmaze.txt".to_string());
        Ok(())
    }
}
