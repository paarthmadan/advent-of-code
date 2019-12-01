mod helper;

use helper::load_file;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./01 <file>");
        process::exit(1)
    };

    let fuel: i32 = load_file(&args[1])
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .map(|v| calculate_fuel(v))
        .sum();

    println!("{}", fuel);
}

fn calculate_fuel(starting: i32) -> i32 {
    let new = starting / 3 - 2;
    if new <= 0 {
        return 0;
    }
    new + calculate_fuel(new)
}
