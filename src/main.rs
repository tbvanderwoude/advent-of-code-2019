use std::{env, io};
mod space_image;
mod misc;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()!=2
    {
        return;
    }
    let filename=&args[1];
    let weights: Vec<i32> = misc::load_weights(filename);
    println!("{}",misc::total_fuel(&weights,true));
    /*
    let mut layers: Vec<String> = space_image::load_image_layers(filename);
    println!("{}",space_image::min_zero_code(&layers));
    space_image::render(&layers);
    */
}
