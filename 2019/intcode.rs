use std::io;
use self::Instr2::*;
use self::Instr3::*;
use self::Instr4::*;
use self::InstructionType::*;
use self::Status::*;

pub struct IntcodeMachine {
    memory: Vec<i32>,
    ptr: usize,
}

impl IntcodeMachine {
    pub fn new(input: Vec<i32>, ptr: usize) -> IntcodeMachine {
        IntcodeMachine {
            memory: input,
            ptr,
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = parse_opcode(self.memory[self.ptr]);
            let instr = match opcode {
                1 => Instruction::new(Instr4(Addition)),
                2 => Instruction::new(Instr4(Multiplication)),
                3 => Instruction::new(Instr2(InputAndSave)),
                4 => Instruction::new(Instr2(Output)),
                5 => Instruction::new(Instr3(JumpIfTrue)),
                6 => Instruction::new(Instr3(JumpIfFalse)),
                7 => Instruction::new(Instr4(LessThan)),
                8 => Instruction::new(Instr4(Equal)),
                99 => Instruction::new(Terminate),
                _ => unreachable! {},
            };

            match instr.call(&mut self.memory, &mut self.ptr) {
                Continue => continue,
                Halt => break,
            }
        }
    }
}

struct Instruction {
    instr_type: InstructionType,
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

impl Instruction {
    fn new(instr_type: InstructionType) -> Instruction {
        Instruction { instr_type }
    } 

    fn call(&self, input: &mut Vec<i32>, ptr: &mut usize) -> Status {
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

fn parse_opcode(instr1: i32) -> i32 {
    instr1 % 100
}
