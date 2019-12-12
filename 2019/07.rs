mod helper;
mod intcode;

use helper::exit;
use intcode::{InputMethod, IntcodeMachine, MachineStatus};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 07 <file>");
    };

    let file = &helper::load_file(&args[1])[0];

    let input: Vec<i32> = file.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let ptr: usize = 0;

    println!("{}", pt_1(&input, &ptr));
    println!("{}", pt_2(&input, &ptr));
}

fn pt_1(input: &Vec<i32>, ptr: &usize) -> i32 {
    let mut max = 0;
    for setting in permutations(&mut [0, 1, 2, 3, 4]) {
        let mut machines: Vec<IntcodeMachine> = Vec::with_capacity(5);
        for i in 0..=4 {
            let mut m =
                IntcodeMachine::new(input.to_owned(), ptr.to_owned(), InputMethod::Computed);
            m.append_input(setting[i]);
            machines.push(m);
        }

        let mut signal = 0;

        for machine in machines.iter_mut() {
            machine.append_input(signal);
            if let MachineStatus::Output(o) = machine.run() {
                signal = o;
            }
        }

        if signal > max {
            max = signal;
        }
    }
    max
}

fn pt_2(input: &Vec<i32>, ptr: &usize) -> i32 {
    let mut max = 0;
    for setting in permutations(&mut [5, 6, 7, 8, 9]) {
        let mut machines: Vec<IntcodeMachine> = Vec::with_capacity(5);
        for i in 0..=4 {
            let mut m =
                IntcodeMachine::new(input.to_owned(), ptr.to_owned(), InputMethod::Computed);
            m.append_input(setting[i]);
            machines.push(m);
        }

        let mut signal = 0;

        loop {
            let mut halt_count = 0;

            for machine in machines.iter_mut() {
                machine.append_input(signal);

                match machine.run() {
                    MachineStatus::Halt => halt_count += 1,
                    MachineStatus::Output(o) => signal = o,
                }
            }

            if signal > max {
                max = signal;
            }

            if halt_count == 5 {
                break;
            }
        }
    }
    max
}

fn permutations(mut elements: &mut [i32; 5]) -> Vec<[i32; 5]> {
    let mut permutations: Vec<[i32; 5]> = Vec::new();
    permutations_helper(5, &mut elements, &mut permutations);
    return permutations;
}

fn permutations_helper(n: usize, elements: &mut [i32; 5], permutations: &mut Vec<[i32; 5]>) {
    if n == 1 {
        permutations.push(*elements);
        return;
    }
    for i in 0..n {
        permutations_helper(n - 1, elements, permutations);
        if n & 1 == 0 {
            let temp = elements[0];
            elements[0] = elements[n - 1];
            elements[n - 1] = temp;
        } else {
            let temp = elements[i];
            elements[i] = elements[n - 1];
            elements[n - 1] = temp;
        }
    }
}
