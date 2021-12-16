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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 02 <file>");
    };

    let input: (u32, u32) = helper::load_file(&args[1])
        .into_iter()
        .map(|line| match line.parse::<Command>() {
            Ok(command) => command,
            Err(_) => exit(&format!("Couldn't parse {} into Command", line)),
        })
        .fold((0, 0), |acc, command| match command {
            Down(units) => (acc.0, acc.1 + units),
            Up(units) => (acc.0, acc.1 - units),
            Forward(units) => (acc.0 + units, acc.1),
        });

    println!("{:?}", input.0 * input.1)
}
