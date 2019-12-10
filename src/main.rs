use std::{env, io};
use std::cmp::max;
use std::fs;
use std::collections::{HashSet, HashMap};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Asteroid
{
    x:i32,
    y:i32
}
pub fn load_asteroids(filename: &String) -> Vec<Asteroid>
{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let mut asteroids: Vec<Asteroid> = vec![];
    for (y,s) in split.enumerate() {
        for (x,c) in s.chars().enumerate() {
            if(c=='#')
            {
                asteroids.push(Asteroid{x:x as i32,y:y as i32});
            }
        }
    }
    for aster in &asteroids {
        //println!("Asteroid at ({0}, {1})",aster.x,aster.y);
    }
    return asteroids;
}
fn point_norm(x:i32,y:i32) ->f32
{
    return ((x.pow(2)+y.pow(2)) as f32).sqrt();
}
fn compute_visible_asteroids(center:Asteroid, orgAsteroids:&Vec<Asteroid>) -> HashMap<(i32,i32),Asteroid>
{
    let asteroids = orgAsteroids.clone();
    let mut visibleMap:HashMap<(i32,i32),Asteroid>=HashMap::new();
    for asteroid in asteroids{
        if asteroid.x!=center.x||asteroid.y!=center.y
        {
            let relX:f32=(asteroid.x-center.x) as f32;
            let relY:f32=(asteroid.y-center.y) as f32;
            let l:f32 = (relX.powf(2f32)+relY.powf(2f32)).sqrt();
            let quantX:i32=(relX*360f32/l) as i32;
            let quantY:i32=(relY*360f32/l) as i32;
            if !visibleMap.contains_key(&(quantX,quantY))
            {
                visibleMap.insert((quantX,quantY),asteroid);
            }
            else {
                let other=visibleMap.get(&(quantX,quantY)).unwrap();
                let otherRelX:f32=(other.x-center.x) as f32;
                let otherRelY:f32=(other.y-center.y) as f32;
                let otherL:f32 = (otherRelX.powf(2f32)+otherRelY.powf(2f32)).sqrt();
                if(l<otherL)
                {
                    visibleMap.insert((quantX,quantY),asteroid);
                }
            }
        }
    }
    return visibleMap;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()!=2
    {
        return;
    }
    let filename=&args[1];
    let mut asteroids=load_asteroids(filename);
    let mut maxVisible: usize=0;
    let mut centerId:usize=0;
    let mut map=HashMap::new();
    for (i,center) in (&asteroids).iter().enumerate(){
        let mut visibleMap=compute_visible_asteroids(*center,&asteroids);
        if visibleMap.len()>maxVisible
        {
            maxVisible=visibleMap.len();
            centerId=i;
            map=visibleMap;
        }
    }
    print!("There are {0} asteroids visible from Asteroid #{1}",maxVisible,centerId);
    asteroids.remove(centerId);
}
