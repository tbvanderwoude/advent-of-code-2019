use aoc::common::parse_numbers;
use std::io;
use std::io::Read;
use std::collections::HashMap;
use std::hash::Hash;
const TWO_PI: f32 = 2.0f32 * std::f32::consts::PI;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Asteroid {
    x: i32,
    y: i32,
}

pub fn load_asteroids(contents: String) -> Vec<Asteroid> {
    let split = contents.split("\n");
    let mut asteroids: Vec<Asteroid> = vec![];
    for (y, s) in split.enumerate() {
        for (x, c) in s.chars().enumerate() {
            if c == '#' {
                asteroids.push(Asteroid {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    return asteroids;
}

pub fn compute_two_hundreth_coord(center: Asteroid,map: HashMap<(i32, i32), Asteroid>) -> i32 {
    let mut dist_asteroids: Vec<(f32,Asteroid)> = vec![];
    for (_, asteroid) in map {
        let mut angle: f32 = -(((asteroid.y - center.y) as f32)
            .atan2(-(asteroid.x - center.x) as f32))
            - 2f32 * TWO_PI / 8f32;
        if angle < 0f32 {
            angle += TWO_PI;
        }
        dist_asteroids.push((angle, asteroid));
    }

    dist_asteroids.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut ordered_asteroids: Vec<Asteroid> = dist_asteroids.iter().map(|a| a.1).collect();
    ordered_asteroids.reverse();
    let mut removed_index = 1;
    while removed_index < 200 {
        ordered_asteroids.pop().unwrap();
        removed_index += 1;
    }
    let two_hundreth_asteroid: Asteroid = ordered_asteroids.pop().unwrap();
    two_hundreth_asteroid.x * 100 + two_hundreth_asteroid.y
}

fn point_norm(x: i32, y: i32) -> f32 {
    ((x.pow(2) + y.pow(2)) as f32).sqrt()
}

fn compute_visible_asteroids(
    center: Asteroid,
    org_asteroids: &Vec<Asteroid>,
) -> HashMap<(i32, i32), Asteroid> {
    let asteroids = org_asteroids.clone();
    let mut visible_map: HashMap<(i32, i32), Asteroid> = HashMap::new();
    for asteroid in asteroids {
        if asteroid.x != center.x || asteroid.y != center.y {
            let rel_x: f32 = (asteroid.x - center.x) as f32;
            let rel_y: f32 = (asteroid.y - center.y) as f32;
            let l: f32 = (rel_x.powf(2f32) + rel_y.powf(2f32)).sqrt();
            let quant_x: i32 = (rel_x * 360f32 / l) as i32;
            let quant_y: i32 = (rel_y * 360f32 / l) as i32;
            if !visible_map.contains_key(&(quant_x, quant_y)) {
                visible_map.insert((quant_x, quant_y), asteroid);
            } else {
                let other = visible_map.get(&(quant_x, quant_y)).unwrap();
                let other_rel_x: f32 = (other.x - center.x) as f32;
                let other_rel_y: f32 = (other.y - center.y) as f32;
                let other_l: f32 = (other_rel_x.powf(2f32) + other_rel_y.powf(2f32)).sqrt();
                if l < other_l {
                    visible_map.insert((quant_x, quant_y), asteroid);
                }
            }
        }
    }
    visible_map
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut asteroids = load_asteroids(input);
    let coord = asteroids.iter().enumerate().map(|(i,a)|
        (i,*a,compute_visible_asteroids(*a,&asteroids))).max_by_key(|t|(*t).2.len()).unwrap();
    let part1 = coord.2.len();
    asteroids.remove(coord.0);
    let part2 = compute_two_hundreth_coord(coord.1,coord.2);
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
