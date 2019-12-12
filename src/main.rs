use std::env;
mod moon;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()!=2
    {
        return;
    }
    let filename=&args[1];

    env::set_var("RUST_BACKTRACE","1");
    let mut moons: Vec<moon::Moon> = moon::load_moons(filename);
    println!("Alignment at iteration #{}",moon::full_alignment(moons));
}
