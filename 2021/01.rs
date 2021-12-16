mod helper;

use helper::exit;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 01 <file>");
    };

    let input = helper::load_file(&args[1])
        .into_iter()
        .map(|i| match i.parse::<u32>() {
            Ok(i) => i,
            Err(_) => exit(&format!("Couldn't parse: {}", i)),
        })
        .collect::<Vec<u32>>();

    let num_increases: u32 = input
        .windows(3)
        .map(|window| window.into_iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .map(|window| (window[1] > window[0]) as u32)
        .sum();

    println!("{:?}", num_increases);
}
