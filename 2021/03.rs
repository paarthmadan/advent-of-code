mod helper;

use helper::exit;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 03 <file>");
    };

    let input: Vec<String> = helper::load_file(&args[1]);
    let n: i32 = input[0].chars().count() as i32;

    let input: Vec<u32> = input
        .into_iter()
        .map(|i| match u32::from_str_radix(&i, 2) {
            Ok(i) => i,
            Err(_) => exit(&format!("Couldn't parse: {}", i)),
        })
        .collect();

    let gamma = gamma(&input, n);
    let epsilon = epsilon(gamma, n);
    println!("{}", gamma * epsilon);

    let oxygen = oxygen(&input, n - 1);
    let carbon = carbon(&input, n - 1);
    println!("{}", oxygen * carbon);
}

fn epsilon(gamma: u32, n: i32) -> u32 {
    let mask: u32 = !((0xffffffff >> n) << n);
    !gamma & mask
}

fn gamma(list: &Vec<u32>, n: i32) -> u32 {
    let mut gamma = 0;
    for bit in 0..=(n - 1) {
        gamma += most_common_bit(&list, bit);
    }
    gamma
}

fn most_common_bit(list: &Vec<u32>, bit: i32) -> u32 {
    let mut sum: u32 = 0;

    for number in list {
        sum += signal(*number, bit);
    }

    let mcb = ((list.len() as u32) < (sum * 2)) as u32;

    mcb << bit
}

fn signal(num: u32, bit: i32) -> u32 {
    let mask: u32 = 1 << bit;
    (num & mask) >> bit
}

fn oxygen(list: &Vec<u32>, bit: i32) -> u32 {
    if list.len() == 1 {
        return list[0];
    }

    let (zeros, ones): (Vec<u32>, Vec<u32>) = list.iter().partition(|n| signal(**n, bit) == 0);

    if ones.len() >= zeros.len() {
        return oxygen(&ones, bit - 1);
    } else {
        return oxygen(&zeros, bit - 1);
    }
}

fn carbon(list: &Vec<u32>, bit: i32) -> u32 {
    if list.len() == 1 {
        return list[0];
    }

    let (zeros, ones): (Vec<u32>, Vec<u32>) = list.iter().partition(|n| signal(**n, bit) == 0);

    if zeros.len() <= ones.len() {
        return carbon(&zeros, bit - 1);
    } else {
        return carbon(&ones, bit - 1);
    }
}
