mod helper;

use helper::exit;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 05 <file>");
    };

    let file = &helper::load_file(&args[1])[0];

    let mut input: Vec<i32> = file.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    perform(&mut input);
}

fn perform(mut input: &mut Vec<i32>) {
    const TERMINATE: i32 = 99;

    let mut pc: usize = 0;
    let mut opcode;

    loop {
        opcode = parse_opcode(&input[pc]);
        match opcode {
            1 | 2 => instr4(opcode, &mut input, &mut pc),
            3 | 4 => instr2(opcode, &mut input, &mut pc),
            99 => break,
            _ => unreachable!{},
        }
    }
}

fn instr4(opcode: i32, instr: &mut Vec<i32>, ptr: &mut usize) {
    let modes = parse_parameter_modes(&instr[*ptr as usize], 2);

    let s1 = resolve(instr, &instr[*ptr + 1], modes[0]);
    let s2 = resolve(instr, &instr[*ptr + 2], modes[1]);
    let dst = instr[*ptr + 3];

    match opcode {
        1 =>  {
            instr[dst as usize] = s1 + s2;
        },
        2 => {
            instr[dst as usize] = s1 * s2;
        },
        _ => unreachable!{},
    }

    *ptr += 4;
}

fn instr2(opcode: i32, instr: &mut Vec<i32>, ptr: &mut usize) {
    let p1 = instr[*ptr + 1];
    match opcode {
        3 => {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read from STDIN");
            let num = input.trim().parse::<i32>().unwrap();
            instr[p1 as usize] = num;
        },
        4 => {
            println!("{}", instr[p1 as usize])
        },
        _ => unreachable!{},
    }
    *ptr += 2;
}

fn parse_opcode(instr1: &i32) -> i32 {
    *instr1 % 100
}

fn parse_parameter_modes(instr1: &i32, min: u8) -> Vec<u8> {
    // Remove last 2 digits (opcode)
    let mut v = *instr1 / 100;
    let mut modes: Vec<u8> = Vec::new();

    for _ in 1..=min {
        modes.push((v % 10) as u8);
        v = v / 10;
    }

    modes
}

fn resolve(instr: &Vec<i32>, p: &i32, mode: u8) -> i32 {
    match mode {
        0 => instr[*p as usize],
        1 => *p,
        _ => unreachable!{},
    }
}
