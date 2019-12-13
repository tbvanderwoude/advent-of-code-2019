use std::env;

mod intcode;
mod arcade;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
        return;
    }
    let filename = &args[1];

    env::set_var("RUST_BACKTRACE", "1");
    let mut program: Vec<i64> = intcode::load_program(filename);
    arcade::render_screen(program);
}
