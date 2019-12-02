mod helper;

use helper::exit;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 02 <file>");
    };

    let file = &helper::load_file(&args[1])[0];
    let mut input: Vec<u32> = file
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    eval(&mut input, 0);
    println!("{}", input[0]);
}

fn eval(input: &mut Vec<u32>, pointer: usize) {
    if (pointer >= input.len() - 1) { return; }
    let op = input[pointer];
    match op {
        1 => compute(input, pointer, true),
        2 => compute(input, pointer, false),
        99 => return,
        _ => exit("Parsed unknown opcode"),
    };
    eval(input, pointer + 4);
}

fn compute(input: &mut Vec<u32>, pointer: usize, is_addition: bool) {
    let a = input[pointer + 1] as usize;
    let b = input[pointer + 2] as usize;
    let c = input[pointer + 3] as usize;

    if is_addition {
        input[c] = input[a] + input[b];
    } else {
        input[c] = input[a] * input[b];
    }
}
