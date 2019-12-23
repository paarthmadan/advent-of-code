mod helper;
mod intcode;

use helper::exit;
use intcode::{InputMethod, IntcodeMachine, MachineStatus};
use std::collections::HashMap;
use std::env;
use Direction::*;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn origin() -> Self {
        Position::new(0, 0)
    }

    fn increment(old: &Self, direction: &Direction) -> Self {
        match direction {
            Up => Position::new(old.x, old.y + 1),
            Down => Position::new(old.x, old.y - 1),
            Left => Position::new(old.x - 1, old.y),
            Right => Position::new(old.x + 1, old.y),
        }
    }
}

struct Robot {
    position: Position,
    direction: Direction,
    ship: HashMap<Position, Colour>,
}

impl Robot {
    fn new() -> Self {
        let mut robot = Robot {
            position: Position::origin(),
            direction: Direction::Up,
            ship: HashMap::new(),
        };

        robot.paint(Colour::White);

        robot
    }

    fn read(&self) -> Colour {
        match self.ship.get(&self.position) {
            Some(colour) => *colour,
            None => Colour::Black,
        }
    }

    fn paint(&mut self, paint_colour: Colour) {
        self.ship.insert(self.position, paint_colour);
    }

    fn run(&mut self, paint_colour: Colour, new_direction: Direction) {
        self.paint(paint_colour);
        self.direction = self.direction.resolve(new_direction);
        self.position = Position::increment(&self.position, &self.direction);
    }

    fn print_ship(&self) {
        let mut min = (std::i32::MAX, std::i32::MAX);
        let mut max = (std::i32::MIN, std::i32::MIN);

        for position in self.ship.keys() {
            let (x, y) = (position.x, position.y);
            if x > max.0 { max.0 = x };
            if x < min.0 { min.0 = x };
            if y > max.1 { max.1 = y };
            if y < min.1 { min.1 = y };
        }

        for y in (min.1..=max.1).rev() {
            for x in min.0..=max.0 {
                let colour = match self.ship.get(&Position::new(x, y)) {
                    Some(colour) => *colour,
                    None => Colour::Black,
                };

                let c = match colour {
                    Colour::Black => '.',
                    Colour::White => '#',
                };

                print!("{}", c);
            }
            println!("");
        }
    }
}

#[derive(Copy, Clone)]
enum Colour {
    Black,
    White,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn resolve(&self, new_direction: Direction) -> Direction {
        match self {
            Up => new_direction,
            Down => if new_direction == Left { Right } else { Left },
            Left => if new_direction == Left { Down } else { Up },
            Right => if new_direction == Left { Up } else { Down },
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 11 <file>");
    };

    let file = &helper::load_file(&args[1])[0];

    let input: Vec<i64> = file.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    let ptr: usize = 0;

    let mut robot = Robot::new();

    let mut machine = IntcodeMachine::new(input, ptr, InputMethod::Computed);

    loop {
        let ship_colour = match robot.read() {
            Colour::Black => 0,
            Colour::White => 1,
        };

        machine.append_input(ship_colour);

        let mut desired_colour: Option<Colour> = None;
        let mut direction_to_move: Option<Direction> = None;

        if let MachineStatus::Output(colour) = machine.run() {
            desired_colour = Some(match colour {
                0 => Colour::Black,
                1 => Colour::White,
                _ => unreachable! {},
            });
        };

        if let MachineStatus::Output(direction) = machine.run() {
            direction_to_move = Some(match direction {
                0 => Direction::Left,
                1 => Direction::Right,
                _ => unreachable! {},
            });
        };
        match (desired_colour, direction_to_move) {
            (Some(colour), Some(direction)) => robot.run(colour, direction),
            _ => break,
        }
    }

    println!("{}", robot.ship.len());
    robot.print_ship();
}
