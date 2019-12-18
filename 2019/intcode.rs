use self::InputMethod::*;
use self::Instr2::*;
use self::Instr3::*;
use self::Instr4::*;
use self::InstructionType::*;
use self::Status::*;
use std::io;

pub struct IntcodeMachine {
    memory: Vec<i64>,
    ptr: usize,
    input_method: InputMethod,
    input_stack: Vec<i64>,
    relative_base: i64,
}

pub enum MachineStatus {
    Output(i64),
    Halt,
}

impl IntcodeMachine {
    pub fn new(input: Vec<i64>, ptr: usize, input_method: InputMethod) -> IntcodeMachine {
        IntcodeMachine {
            memory: input,
            ptr,
            input_method,
            input_stack: Vec::new(),
            relative_base: 0,
        }
    }

    fn read(&mut self, index: usize) -> i64 {
        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
        }
        self.memory[index]
    }

    fn write(&mut self, index: usize, value: i64) {
        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
        }
        self.memory[index] = value;
    }

    fn push_input_to_stack(&mut self) {
        match self.input_method {
            User => {
                let mut inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Couldn't read from STDIN");
                let num = inp.trim().parse::<i64>().unwrap();
                self.input_stack.push(num);
            }
            Computed => {}
        }
    }

    pub fn run(&mut self) -> MachineStatus {
        loop {
            let opcode = parse_opcode(self.read(self.ptr));
            let instr = match opcode {
                1 => Instruction::new(Instr4(Addition)),
                2 => Instruction::new(Instr4(Multiplication)),
                3 => Instruction::new(Instr2(InputAndSave(self.input_method))),
                4 => Instruction::new(Instr2(Output(self.input_method))),
                5 => Instruction::new(Instr3(JumpIfTrue)),
                6 => Instruction::new(Instr3(JumpIfFalse)),
                7 => Instruction::new(Instr4(LessThan)),
                8 => Instruction::new(Instr4(Equal)),
                9 => Instruction::new(Instr2(RelativeBase)),
                99 => Instruction::new(Terminate),
                _ => {
//                    println!("{}", opcode);
                    unreachable! {}
                },
            };

            match instr.call(self) {
                Continue => continue,
                Halt(out) => match out {
                    Some(o) => return MachineStatus::Output(o),
                    None => return MachineStatus::Halt,
                },
            }
        }
    }

    pub fn append_input(&mut self, input: i64) {
        self.input_stack.push(input)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum InputMethod {
    User,
    Computed,
}

struct Instruction {
    instr_type: InstructionType,
}

#[derive(Debug)]
enum InstructionType {
    Instr2(Instr2),
    Instr3(Instr3),
    Instr4(Instr4),
    Terminate,
}

#[derive(Debug)]
enum Instr2 {
    InputAndSave(InputMethod),
    Output(InputMethod),
    RelativeBase,
}

#[derive(Debug)]
enum Instr3 {
    JumpIfTrue,
    JumpIfFalse,
}

#[derive(Debug)]
enum Instr4 {
    Addition,
    Multiplication,
    LessThan,
    Equal,
}

enum Status {
    Continue,
    Halt(Option<i64>),
}

impl Instruction {
    fn new(instr_type: InstructionType) -> Instruction {
        Instruction { instr_type }
    }

    fn call(&self, machine: &mut IntcodeMachine) -> Status {
        //println!("Head: {}", machine.ptr);
        //println!("Instruction: {:#?}", self.instr_type);
        //println!("Memory: {:?}", machine.memory);
        match &self.instr_type {
            Instr2(op) => {
                let modes = parse_parameter_modes(&machine.read(machine.ptr), 1);
                let p1 = resolve(machine, machine.ptr + 1, modes[0]);
                match op {
                    InputAndSave(method) => {
                        machine.push_input_to_stack();

                        let val = machine.read(machine.ptr + 1);
                        let pop = machine.input_stack.remove(0);

                        if modes[0] != 2 {
                            machine.write(val as usize, pop);
                        } else {
                            machine.write((machine.relative_base + val) as usize, pop);
                        };
                    }
                    Output(method) => match method {
                        Computed => {
                            machine.ptr += 2;
                            return Status::Halt(Some(p1));
                        }
                        User => println!("{}", p1),
                    },
                    RelativeBase => {
                        machine.relative_base += p1;
                    }
                }
                machine.ptr += 2;
            }
            Instr3(op) => {
                let modes = parse_parameter_modes(&machine.read(machine.ptr), 2);
                let s1 = resolve(machine, machine.ptr + 1, modes[0]);
                let s2 = resolve(machine, machine.ptr + 2, modes[1]);
                match op {
                    JumpIfTrue => {
                        machine.ptr = if s1 != 0 {
                            s2 as usize
                        } else {
                            machine.ptr + 3
                        }
                    }
                    JumpIfFalse => {
                        //println!("s1: {}, s2: {}", s1, s2);
                        machine.ptr = if s1 == 0 {
                            s2 as usize
                        } else {
                            machine.ptr + 3
                        }
                    }
                }
            }
            Instr4(op) => {
                let modes = parse_parameter_modes(&machine.read(machine.ptr), 3);
                let s1 = resolve(machine, machine.ptr + 1, modes[0]);
                let s2 = resolve(machine, machine.ptr + 2, modes[1]);
                let dst = write_resolve(machine, machine.ptr + 3, modes[2]) as usize;

                //println!("Saving at DST: {}", dst);

                match op {
                    Addition => machine.write(dst, s1 + s2),
                    Multiplication => machine.write(dst, s1 * s2),
                    LessThan => machine.write(dst, (s1 < s2) as i64),
                    Equal => machine.write(dst, (s1 == s2) as i64),
                }
                machine.ptr += 4;
            }
            Terminate => return Status::Halt(None),
        }
        Status::Continue
    }
}

fn parse_parameter_modes(instr1: &i64, min: u8) -> Vec<u8> {
    let mut v = *instr1 / 100;
    let mut modes: Vec<u8> = Vec::new();

    for _ in 1..=min {
        modes.push((v % 10) as u8);
        v = v / 10;
    }

    modes
}

fn resolve(machine: &mut IntcodeMachine, ptr: usize, mode: u8) -> i64 {
    let p = machine.read(ptr);
    match mode {
        0 => machine.read(p as usize),
        1 => p,
        2 => machine.read((machine.relative_base + p) as usize),
        _ => unreachable! {},
    }
}

fn write_resolve(machine: &mut IntcodeMachine, ptr: usize, mode: u8) -> i64 {
    let p = machine.read(ptr);
    let f = match mode {
        0 | 1 => p,
        2 => machine.relative_base + p,
        _ => unreachable! {},
    };
    f
}

fn parse_opcode(instr1: i64) -> i64 {
    instr1 % 100
}
