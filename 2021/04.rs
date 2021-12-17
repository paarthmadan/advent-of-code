mod helper;

use helper::exit;
use std::collections::HashMap;
use std::env;

struct Board {
    board_state_encoded: u32,
    board_map: HashMap<u32, (usize, usize)>,
    length: usize,
    winning_draw: Option<u32>,
}

impl Board {
    pub fn from_matrix(matrix: Vec<Vec<u32>>) -> Self {
        let mut board_map: HashMap<u32, (usize, usize)> = HashMap::new();
        let board_state_encoded: u32 = 0;
        let winning_draw = None;
        let length = matrix.len();

        for (y, row) in matrix.into_iter().enumerate() {
            for (x, el) in row.into_iter().enumerate() {
                board_map.insert(el, (x, y));
            }
        }

        Board {
            board_map,
            board_state_encoded,
            length,
            winning_draw,
        }
    }

    fn update(&mut self, number: u32) {
        match self.board_map.get(&number).copied() {
            Some(pos) => self.encode(pos),
            None => {}
        }
    }

    fn set_winning_draw(&mut self, draw: u32) {
        self.winning_draw = Some(draw);
    }

    fn encode(&mut self, pos: (usize, usize)) {
        let bit = pos.1 * self.length + pos.0;
        self.board_state_encoded += 1 << bit;
    }

    fn signal(&self, pos: (usize, usize)) -> u32 {
        let bit = (pos.1 * self.length) + pos.0;
        let mask: u32 = 1 << bit;
        (self.board_state_encoded & mask) >> bit
    }

    fn score(&self, recent_num: u32) -> u32 {
        let sum: u32 = (&self.board_map)
            .into_iter()
            .map(|(k, v)| (!self.signal(*v) & 1) * k)
            .sum();
        recent_num * sum
    }

    fn check_board(&self) -> bool {
        self.check_rows() || self.check_columns()
    }

    fn check_rows(&self) -> bool {
        let s: u32 = 0xffffffff;
        let l = self.length;
        let window = !((s >> l) << l);

        for i in 1..=l {
            let mask = window << (l * (i - 1));

            if self.board_state_encoded & mask == mask {
                return true;
            }
        }

        false
    }

    fn check_columns(&self) -> bool {
        let l = self.length;
        for i in 1..=l {
            let position_bit = 1 << (i - 1);
            let mut mask = position_bit;
            for j in 1..=(l - 1) {
                let bit_multiplier = self.length * j;
                mask += position_bit << bit_multiplier;
            }

            if self.board_state_encoded & mask == mask {
                return true;
            }
        }

        false
    }
}

fn parse_board(input: &[String]) -> Board {
    let board_num: Vec<Vec<u32>> = input
        .into_iter()
        .map(|row| {
            row.trim()
                .split_whitespace()
                .into_iter()
                .map(|c| match c.parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => exit(&format!("Couldn't parse {} into number", c)),
                })
                .collect()
        })
        .collect();

    Board::from_matrix(board_num)
}

fn parse_draws(str: &str) -> Vec<u32> {
    str.split(",")
        .into_iter()
        .map(|c| match c.parse::<u32>() {
            Ok(num) => num,
            Err(_) => exit(&format!("Couldn't parse {} into number", c)),
        })
        .collect()
}

fn parse_boards(input: Vec<String>) -> Vec<Board> {
    let mut index = 1;
    let mut boards: Vec<Board> = vec![];

    while index < input.len() {
        boards.push(parse_board(&input[(index + 1)..(index + 6)]));
        index += 6;
    }

    boards
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 04 <file>");
    };

    let input: Vec<String> = helper::load_file(&args[1]);
    let draws = parse_draws(&input[0]);
    let mut boards = parse_boards(input);
    let mut completed_boards: Vec<Board> = Vec::with_capacity(boards.len());

    'draw: for draw in draws {
        for board in &mut boards {
            board.update(draw);
        }
        let (mut completed, incompleted): (Vec<Board>, Vec<Board>) =
            boards.into_iter().partition(|b| b.check_board());

        boards = incompleted;
        completed.iter_mut().for_each(|b| b.set_winning_draw(draw));
        completed_boards.append(&mut completed);

        if boards.len() == 0 {
            break 'draw;
        }
    }

    let score = match completed_boards.pop() {
        Some(board) => board.score(board.winning_draw.unwrap()),
        None => 0,
    };

    println!("{}", score);
}
