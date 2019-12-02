mod helper;

use helper::exit;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 01 <file>");
    };

    let fuel: i32 = helper::load_file(&args[1])
        .into_iter()
        .map(|x| match x.parse::<i32>() {
            Ok(v) => calculate_fuel(v),
            Err(_) => exit(&format!("Couldn't parse: {}", x)),
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
