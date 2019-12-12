use std::{env, io};
use std::cmp::max;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::hash::Hash;

const TWO_PI: f32 = 2.0f32 * std::f32::consts::PI;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Asteroid
{
    x: i32,
    y: i32,
}

pub fn load_asteroids(filename: &String) -> Vec<Asteroid>
{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let mut asteroids: Vec<Asteroid> = vec![];
    for (y, s) in split.enumerate() {
        for (x, c) in s.chars().enumerate() {
            if (c == '#')
            {
                asteroids.push(Asteroid { x: x as i32, y: y as i32 });
            }
        }
    }
    for aster in &asteroids {
        //println!("Asteroid at ({0}, {1})",aster.x,aster.y);
    }
    return asteroids;
}

pub fn compute_two_hundreth_coord(mut asteroids: Vec<Asteroid>) -> i32
{
    let mut maxVisible: usize = 0;
    let mut centerId: usize = 0;
    let mut map = HashMap::new();
    for (i, center) in (&asteroids).iter().enumerate() {
        let mut visibleMap = compute_visible_asteroids(*center, &asteroids);
        if visibleMap.len() > maxVisible
        {
            maxVisible = visibleMap.len();
            centerId = i;
            map = visibleMap;
        }
    }
    let center: Asteroid = asteroids[centerId];
    let mut distAsteroids = vec![];
    asteroids.remove(centerId);
    for (key, asteroid) in map {
        let mut angle: f32 = -(((asteroid.y - center.y) as f32).atan2(-(asteroid.x - center.x) as f32)) - 2f32 * TWO_PI / 8f32;
        if angle < 0f32
        {
            angle += TWO_PI;
        }
        distAsteroids.push((angle, asteroid));
    }

    distAsteroids.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    //println!("The center is at ({0}, {1})",center.x,center.y);
    for (angle, asteroid) in &distAsteroids {
        //println!("Asteroid at ({0}, {1}) (rel ({3}, {4}) with angle of {2})", asteroid.x, asteroid.y,angle,asteroid.x-center.x,asteroid.y-center.y);
    }
    let mut orderedAsteroids: Vec<Asteroid> = distAsteroids.iter().map(|a| a.1).collect();
    orderedAsteroids.reverse();

    //print!("There are {0} asteroids visible from Asteroid #{1}",maxVisible,centerId);

    let mut removedIndex = 1;
    let mut coordCode = 0;
    while (removedIndex < 200) {
        let aster = orderedAsteroids.pop().unwrap();
        removedIndex += 1;
    }
    let two_hundreth_asteroid: Asteroid = orderedAsteroids.pop().unwrap();
    return two_hundreth_asteroid.x * 100 + two_hundreth_asteroid.y;
}

fn point_norm(x: i32, y: i32) -> f32
{
    return ((x.pow(2) + y.pow(2)) as f32).sqrt();
}

fn compute_visible_asteroids(center: Asteroid, orgAsteroids: &Vec<Asteroid>) -> HashMap<(i32, i32), Asteroid>
{
    let asteroids = orgAsteroids.clone();
    let mut visibleMap: HashMap<(i32, i32), Asteroid> = HashMap::new();
    for asteroid in asteroids {
        if asteroid.x != center.x || asteroid.y != center.y
        {
            let relX: f32 = (asteroid.x - center.x) as f32;
            let relY: f32 = (asteroid.y - center.y) as f32;
            let l: f32 = (relX.powf(2f32) + relY.powf(2f32)).sqrt();
            let quantX: i32 = (relX * 360f32 / l) as i32;
            let quantY: i32 = (relY * 360f32 / l) as i32;
            if !visibleMap.contains_key(&(quantX, quantY))
            {
                visibleMap.insert((quantX, quantY), asteroid);
            } else {
                let other = visibleMap.get(&(quantX, quantY)).unwrap();
                let otherRelX: f32 = (other.x - center.x) as f32;
                let otherRelY: f32 = (other.y - center.y) as f32;
                let otherL: f32 = (otherRelX.powf(2f32) + otherRelY.powf(2f32)).sqrt();
                if (l < otherL)
                {
                    visibleMap.insert((quantX, quantY), asteroid);
                }
            }
        }
    }
    return visibleMap;
}