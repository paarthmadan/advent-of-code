use self::InputMethod::*;
use self::Instr2::*;
use self::Instr3::*;
use self::Instr4::*;
use self::InstructionType::*;
use self::Status::*;
use std::io;

pub struct IntcodeMachine {
    memory: Vec<i32>,
    ptr: usize,
    input_method: InputMethod,
    input_stack: Vec<i32>,
}

pub enum MachineStatus {
    Output(i32),
    Halt,
}

impl IntcodeMachine {
    pub fn new(input: Vec<i32>, ptr: usize, input_method: InputMethod) -> IntcodeMachine {
        IntcodeMachine {
            memory: input,
            ptr,
            input_method,
            input_stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> MachineStatus {
        loop {
            let opcode = parse_opcode(self.memory[self.ptr]);
            let instr = match opcode {
                1 => Instruction::new(Instr4(Addition)),
                2 => Instruction::new(Instr4(Multiplication)),
                3 => Instruction::new(Instr2(InputAndSave(self.input_method))),
                4 => Instruction::new(Instr2(Output)),
                5 => Instruction::new(Instr3(JumpIfTrue)),
                6 => Instruction::new(Instr3(JumpIfFalse)),
                7 => Instruction::new(Instr4(LessThan)),
                8 => Instruction::new(Instr4(Equal)),
                99 => Instruction::new(Terminate),
                _ => unreachable! {},
            };

            match instr.call(&mut self.memory, &mut self.ptr, &mut self.input_stack) {
                Continue => continue,
                Halt(out) => match out {
                    Some(o) => return MachineStatus::Output(o),
                    None => return MachineStatus::Halt,
                },
            }
        }
    }

    pub fn append_input(&mut self, input: i32) {
        self.input_stack.push(input)
    }
}

#[derive(Clone, Copy)]
pub enum InputMethod {
    User,
    Computed,
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
    InputAndSave(InputMethod),
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
    Halt(Option<i32>),
}

impl Instruction {
    fn new(instr_type: InstructionType) -> Instruction {
        Instruction { instr_type }
    }

    fn call(
        &self,
        input: &mut Vec<i32>,
        ptr: &mut usize,
        mut input_stack: &mut Vec<i32>,
    ) -> Status {
        match &self.instr_type {
            Instr2(op) => {
                let modes = parse_parameter_modes(&input[*ptr as usize], 1);
                let p1 = input[*ptr + 1];
                match op {
                    InputAndSave(method) => {
                        push_input_to_stack(&mut input_stack, *method);
                        input[p1 as usize] = input_stack.remove(0);
                    }
                    Output => {
                        let o = resolve(&input, &p1, modes[0]);
                        *ptr += 2;
                        return Status::Halt(Some(o));
                    }
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
            Terminate => return Status::Halt(None),
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

fn push_input_to_stack(input_stack: &mut Vec<i32>, im: InputMethod) {
    match im {
        User => {
            let mut inp = String::new();
            io::stdin()
                .read_line(&mut inp)
                .expect("Couldn't read from STDIN");
            let num = inp.trim().parse::<i32>().unwrap();
            input_stack.push(num);
        }
        Computed => {}
    }
}
