mod helper;
mod intcode;

use helper::exit;
use intcode::{InputMethod, IntcodeMachine};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 09 <file>");
    };

    let file = &helper::load_file(&args[1])[0];

    let input: Vec<i64> = file.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    let ptr: usize = 0;

    let mut machine = IntcodeMachine::new(input, ptr, InputMethod::User);
    machine.run();
}
