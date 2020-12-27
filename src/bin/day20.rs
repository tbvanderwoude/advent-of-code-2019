use std::io;
use std::io::Read;

use std::collections::HashMap;

use queues::*;

#[derive(Clone)]
pub struct Maze {
    width: usize,
    height: usize,
    map: Vec<Vec<char>>,
    start: (usize, usize),
    tunnels: HashMap<(usize, usize), (usize, usize)>,
}

impl Maze {
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
    fn show(&self) {
        for y in 0..self.height {
            println!("{}", self.map[y].iter().collect::<String>());
        }
    }
    fn has_letter(&self, x: usize, y: usize) -> bool {
        self.get(x, y) >= 'A' && self.get(x, y) <= 'Z'
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

pub fn load_maze(contents: String) -> Maze {
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
    let x = maze.iter().map(|vec| vec.len()).max().unwrap();
    for i in 0..maze.len() {
        while maze[i].len() < x {
            maze[i].push(' ');
        }
    }
    Maze {
        width: x,
        height: y,
        map: maze,
        start: (0, 0),
        tunnels: HashMap::new(),
    }
}

pub fn run_maze(contents: String, recursive: bool) -> usize {
    let maze: Maze = load_maze(contents);
    explore_maze(maze, recursive)
}
fn resolve_dist(
    lineage: &HashMap<(usize, usize, usize), (usize, usize, usize)>,
    mut node: (usize, usize, usize),
    start: (usize, usize, usize),
) -> usize {
    let mut dist = 0;
    while lineage.contains_key(&node) && node != start {
        node = *lineage.get(&node).unwrap();
        dist += 1;
    }
    dist
}
pub fn explore_maze(mut maze: Maze, recursive: bool) -> usize {
    let mut temp_map: HashMap<String, (usize, usize)> = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 1..maze.height {
        for x in 1..maze.width {
            if maze.get(x, y) == '.' {
                for neigh in maze.neighbours(x, y) {
                    if maze.has_letter(neigh.0, neigh.1) {
                        let mut portal_name: String = String::from("");
                        for letter_neigh in maze.neighbours(neigh.0, neigh.1) {
                            if maze.has_letter(letter_neigh.0, letter_neigh.1) {
                                if letter_neigh.0 < neigh.0 || letter_neigh.1 < neigh.1 {
                                    portal_name.push(maze.get(letter_neigh.0, letter_neigh.1));
                                    portal_name.push(maze.get(neigh.0, neigh.1));
                                } else {
                                    portal_name.push(maze.get(neigh.0, neigh.1));
                                    portal_name.push(maze.get(letter_neigh.0, letter_neigh.1));
                                }
                                maze.set(letter_neigh.0, letter_neigh.1, '#');
                            }
                        }
                        maze.set(neigh.0, neigh.1, '#');
                        if portal_name == "AA" {
                            start = (x, y);
                        } else if portal_name == "ZZ" {
                            end = (x, y);
                        } else if temp_map.contains_key(&portal_name) {
                            let other_pos = *temp_map.get(&portal_name).unwrap();
                            maze.tunnels.insert(other_pos, (x, y));
                            maze.tunnels.insert((x, y), other_pos);
                            let this_norm = (x as i32 - (maze.width / 2) as i32).abs()
                                + (y as i32 - (maze.height / 2) as i32).abs();
                            let other_norm = (other_pos.0 as i32 - (maze.width / 2) as i32).abs()
                                + (other_pos.1 as i32 - (maze.height / 2) as i32).abs();
                            if this_norm > other_norm {
                                maze.set(other_pos.0, other_pos.1, 'i');
                                maze.set(x, y, 'o');
                            } else {
                                maze.set(other_pos.0, other_pos.1, 'o');
                                maze.set(x, y, 'i');
                            }
                        } else {
                            temp_map.insert(portal_name, (x, y));
                        }
                    }
                }
            }
        }
    }
    let mut parent: HashMap<(usize, usize, usize), (usize, usize, usize)> = HashMap::new();
    let mut q: Queue<(usize, usize, usize)> = Queue::new();
    q.add((start.0, start.1, 0));
    while q.size() > 0 {
        let node = q.remove().unwrap();
        if node.2 < 30 {
            let neighbours = maze.neighbours(node.0, node.1);
            for (x, y) in neighbours {
                if !parent.contains_key(&(x, y, node.2)) {
                    parent.insert((x, y, node.2), node);
                    q.add((x, y, node.2));
                    if (!recursive || node.2 == 0) && x == end.0 && y == end.1 {
                        return resolve_dist(
                            &parent,
                            (end.0, end.1, node.2),
                            (start.0, start.1, 0),
                        );
                    }
                }
            }
            if maze.get(node.0, node.1) == 'i' {
                let (px, py) = *maze.tunnels.get(&(node.0, node.1)).unwrap();
                if !parent.contains_key(&(px, py, node.2 + 1)) {
                    parent.insert((px, py, node.2 + 1), node);
                    q.add((px, py, node.2 + 1));
                }
            } else if node.2 > 0 && maze.get(node.0, node.1) == 'o' {
                let (px, py) = *maze.tunnels.get(&(node.0, node.1)).unwrap();
                if !parent.contains_key(&(px, py, node.2 - 1)) {
                    parent.insert((px, py, node.2 - 1), node);
                    q.add((px, py, node.2 - 1));
                }
            }
        }
    }
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let part1 = run_maze(input.clone(), false);
    let part2 = run_maze(input.clone(), true);
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
