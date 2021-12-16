mod helper;

use helper::exit;
use std::env;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Down(u32),
    Up(u32),
    Forward(u32),
}

use Command::*;

#[derive(Debug, Clone)]
struct CommandParserError;

impl From<ParseIntError> for CommandParserError {
    fn from(_err: ParseIntError) -> CommandParserError {
        CommandParserError
    }
}

impl FromStr for Command {
    type Err = CommandParserError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let command: Vec<&str> = str.split(" ").collect();

        let units = command[1].parse::<u32>()?;

        match command[0] {
            "down" => Ok(Down(units)),
            "up" => Ok(Up(units)),
            "forward" => Ok(Forward(units)),
            _ => Err(CommandParserError),
        }
    }
}

struct Submarine {
    x: u32,
    y: u32,
    aim: u32,
}

impl Submarine {
    pub fn new() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }

    fn perform(&mut self, command: Command) {
        match command {
            Down(units) => self.aim += units,
            Up(units) => self.aim -= units,
            Forward(units) => {
                self.x += units;
                self.y += units * self.aim;
            }
        }
    }

    fn score(&self) -> u32 {
        self.x * self.y
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 02 <file>");
    };

    let mut submarine = Submarine::new();

    helper::load_file(&args[1])
        .into_iter()
        .map(|line| match line.parse::<Command>() {
            Ok(command) => command,
            Err(_) => exit(&format!("Couldn't parse {} into Command", line)),
        })
        .for_each(|command| submarine.perform(command));

    println!("{:?}", submarine.score());
}
