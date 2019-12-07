mod helper;

use helper::exit;
use std::cell::RefCell;
use std::{env, io};
use Instr2::*;
use Instr3::*;
use Instr4::*;
use InstructionType::*;
use Status::*;

struct Instruction<'a> {
    instr_type: InstructionType,
    input: &'a RefCell<Vec<i32>>,
    pc: &'a RefCell<usize>,
}

enum InstructionType {
    Instr2(Instr2),
    Instr3(Instr3),
    Instr4(Instr4),
    Terminate,
}

enum Instr2 {
    InputAndSave,
    Output,
}

enum Instr3 {
    JumpIfTrue,
    JumpIfFalse,
}

enum Instr4 {
    Addition,
    Multiplication,
    LessThan,
    Equal,
}

enum Status {
    Continue,
    Halt,
}

impl Instruction<'_> {
    fn new<'a>(
        instr: InstructionType,
        input: &'a RefCell<Vec<i32>>,
        pc: &'a RefCell<usize>,
    ) -> Instruction<'a> {
        Instruction {
            instr_type: instr,
            input: input,
            pc: pc,
        }
    }

    fn call(&self) -> Status {
        let mut input = self.input.borrow_mut();
        let mut ptr = self.pc.borrow_mut();
        match &self.instr_type {
            Instr2(op) => {
                let modes = parse_parameter_modes(&input[*ptr as usize], 1);
                let p1 = input[*ptr + 1];
                match op {
                    InputAndSave => {
                        let mut num = String::new();
                        io::stdin()
                            .read_line(&mut num)
                            .expect("Failed to read from STDIN");
                        let num = num.trim().parse::<i32>().unwrap();
                        input[p1 as usize] = num;
                    }
                    Output => println!("{}", resolve(&input, &p1, modes[0])),
                }
                *ptr += 2;
            }
            Instr3(op) => {
                let modes = parse_parameter_modes(&input[*ptr as usize], 2);
                let s1 = resolve(&input, &input[*ptr + 1], modes[0]);
                let s2 = resolve(&input, &input[*ptr + 2], modes[1]);
                match op {
                    JumpIfTrue => *ptr = if s1 != 0 { s2 as usize } else { *ptr + 3 },
                    JumpIfFalse => *ptr = if s1 == 0 { s2 as usize } else { *ptr + 3 },
                }
            }
            Instr4(op) => {
                let modes = parse_parameter_modes(&input[*ptr as usize], 2);
                let s1 = resolve(&input, &input[*ptr + 1], modes[0]);
                let s2 = resolve(&input, &input[*ptr + 2], modes[1]);
                let dst = input[*ptr + 3] as usize;

                match op {
                    Addition => input[dst] = s1 + s2,
                    Multiplication => input[dst] = s1 * s2,
                    LessThan => input[dst] = (s1 < s2) as i32,
                    Equal => input[dst] = (s1 == s2) as i32,
                }
                *ptr += 4;
            }
            Terminate => return Status::Halt,
        }
        Status::Continue
    }
}

fn parse_parameter_modes(instr1: &i32, min: u8) -> Vec<u8> {
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
        _ => unreachable! {},
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 05 <file>");
    };

    let file = &helper::load_file(&args[1])[0];

    let input: Vec<i32> = file.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    perform(input);
}

fn perform(input: Vec<i32>) {
    let input = RefCell::new(input);
    let pc: RefCell<usize> = RefCell::new(0);
    let mut opcode;

    loop {
        opcode = parse_opcode(&input.borrow()[*pc.borrow()]);
        let instr = match opcode {
            1 => Instruction::new(Instr4(Addition), &input, &pc),
            2 => Instruction::new(Instr4(Multiplication), &input, &pc),
            3 => Instruction::new(Instr2(InputAndSave), &input, &pc),
            4 => Instruction::new(Instr2(Output), &input, &pc),
            5 => Instruction::new(Instr3(JumpIfTrue), &input, &pc),
            6 => Instruction::new(Instr3(JumpIfFalse), &input, &pc),
            7 => Instruction::new(Instr4(LessThan), &input, &pc),
            8 => Instruction::new(Instr4(Equal), &input, &pc),
            99 => Instruction::new(Terminate, &input, &pc),
            _ => unreachable! {},
        };

        match instr.call() {
            Continue => continue,
            Halt => break,
        }
    }
}

fn parse_opcode(instr1: &i32) -> i32 {
    *instr1 % 100
}
