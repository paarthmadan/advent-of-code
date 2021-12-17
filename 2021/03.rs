mod helper;

use helper::exit;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 03 <file>");
    };

    let input: Vec<String> = helper::load_file(&args[1]);
    let n: u32 = input[0].chars().count() as u32;

    let input: Vec<u32> = input
        .into_iter()
        .map(|i| match u32::from_str_radix(&i, 2) {
            Ok(i) => i,
            Err(_) => exit(&format!("Couldn't parse: {}", i)),
        })
        .collect();

    let gamma = gamma(&input, n);
    let epsilon = epsilon(&input, n);
    println!("{}", gamma * epsilon);
}

fn epsilon(list: &Vec<u32>, n: u32) -> u32 {
    let mask: u32 = !(((0xffffffff) >> n) << n);
    (!gamma(&list, n)) & mask
}

fn gamma(list: &Vec<u32>, n: u32) -> u32 {
    let mut gamma = 0;
    for bit in 1..=n {
        gamma += most_common_bit(&list, bit);
    }
    gamma
}

fn most_common_bit(list: &Vec<u32>, bit: u32) -> u32 {
    let mask: u32 = 1 << bit;
    let mut sum: u32 = 0;

    for number in list {
        sum += (number & mask) >> bit;
    }

    let mcb = ((list.len() as u32) < (sum * 2)) as u32;

    mcb << bit
}
