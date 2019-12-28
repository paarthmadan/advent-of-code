mod helper;
mod intcode;

use helper::exit;
use intcode::{InputMethod, IntcodeMachine, MachineStatus};
use MachineStatus::Output;
use std::env;
use std::collections::HashMap;

struct Screen {
    tiles: HashMap<(u64, u64), Tile>
}

impl Screen {
    fn new() -> Self {
        Screen { 
            tiles: HashMap::new(),
        }
    }

    fn add_tile(&mut self, position: (u64, u64), tile: Tile) {
        self.tiles.insert(position, tile);
    }
}

enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn from_input(tile: u64) -> Self {
        match tile {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => unreachable! {},
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 13 <file>");
    };

    let file = &helper::load_file(&args[1])[0];

    let input: Vec<i64> = file.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    let ptr: usize = 0;

    let mut machine = IntcodeMachine::new(input, ptr, InputMethod::Computed);
    let mut screen = Screen::new();

    loop {
        let output = (machine.run(), machine.run(), machine.run());
        match output {
            (Output(x), Output(y), Output(z)) => screen.add_tile((x as u64, y as u64), Tile::from_input(z as u64)),
            _ => break,
        };
    }

    println!("{}", pt_1(&screen));
}

fn pt_1(screen: &Screen) -> u32 {
    let mut block_count = 0;
    for tile in screen.tiles.values() {
        match tile {
            Tile::Block => block_count += 1,
            _ => continue,
        }
    }

    block_count
}
