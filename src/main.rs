use std::env;

mod nanofactory;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
        return;
    }
    let filename = &args[1];

    env::set_var("RUST_BACKTRACE", "1");
    let ore: i64 = nanofactory::compute_max_fuel(filename);
    println!("Ore needed: {}",ore);
    //arcade::render_screen(program);
}
