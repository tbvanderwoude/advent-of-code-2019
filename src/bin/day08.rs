use std::io::Read;

use std::{env, io, thread};

pub fn min_zero_code(layers: &Vec<String>) -> i32 {
    let mut min_layer: usize = 0;
    let mut min_zero: i32 = -1;
    for (i, layer) in layers.iter().enumerate() {
        let zero_count: i32 = layer.matches("0").count() as i32;
        if zero_count < min_zero || min_zero == -1 {
            min_layer = i;
            min_zero = zero_count;
        }
    }
    let one_count = layers[min_layer].matches("1").count();
    let two_count = layers[min_layer].matches("2").count();
    return (one_count * two_count) as i32;
}

pub fn load_image_layers(mut contents: String) -> Vec<String> {
    let layer_size = 25 * 6;
    let mut layers: Vec<String> = vec![];
    while !contents.is_empty() {
        let (chunk, rest) = contents.split_at(std::cmp::min(layer_size, contents.len()));
        layers.push(chunk.to_string());
        contents = rest.to_string();
    }
    return layers;
}

pub fn render(layers: &Vec<String>) {
    for y in 0..6 {
        for x in 0..25 {
            for layer in layers {
                let c = layer.chars().nth(y * 25 + x).unwrap();
                if c != '2' {
                    if c == '0' {
                        print!(".");
                    } else {
                        print!("{}", c);
                    }
                    break;
                }
            }
        }
        print!("\n");
    }
}
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let layers = load_image_layers(input);
    let part1 = min_zero_code(&layers);
    println!("Part 1: {}\nPart2:", part1);
    render(&layers);
}
