mod helper;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./01 <file>");
        process::exit(1)
    };

    let fuel: i32 = helper::load_file(&args[1])
        .into_iter()
        .map(|x| {
            let v = x.parse::<i32>().unwrap();
            calculate_fuel(v)
        })
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
