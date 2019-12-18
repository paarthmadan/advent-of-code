mod helper;

use helper::exit;
use std::collections::HashSet;
use std::env;

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd)]
struct Asteroid {
    x: u32,
    y: u32,
}

impl Asteroid {
    fn new(x: u32, y: u32) -> Self {
        Asteroid { x, y }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Slope {
    delta_x: i32,
    delta_y: i32,
    pole: bool,
}

impl Slope {
    fn new(delta_x: i32, delta_y: i32, pole: bool) -> Self {
        let gcd = gcd(delta_y, delta_x);
        Slope {
            delta_x: delta_x / gcd,
            delta_y: delta_y / gcd,
            pole,
        }
    }

    fn between(a: &Asteroid, b: &Asteroid) -> Self {
        Slope::new(a.x as i32 - b.x as i32, a.y as i32 - b.y as i32, a > b)
    }
}

fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 10 <file>");
    };

    let mut asteroids: HashSet<Asteroid> = HashSet::new();

    for (y, line) in helper::load_file(&args[1]).into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.insert(Asteroid::new(x as u32, y as u32));
            }
        }
    }
    pt_1(&asteroids);
}

fn pt_1(asteroids: &HashSet<Asteroid>) {
    let mut max_count = 0;
    let mut max_asteroid: Option<&Asteroid> = None;

    for asteroid in asteroids {
        let count = find_detectable_asteroids(&asteroid, &asteroids);
        if count > max_count {
            max_count = count;
            max_asteroid = Some(asteroid);
        }
    }

    println!("{}", max_count);
    println!("{:?}", *max_asteroid.unwrap());
}

fn find_detectable_asteroids(current_asteroid: &Asteroid, asteroids: &HashSet<Asteroid>) -> u32 {
    let mut slopes_encountered: HashSet<Slope> = HashSet::new();

    for asteroid in asteroids {
        if *asteroid == *current_asteroid { continue; }
        slopes_encountered.insert(Slope::between(current_asteroid, asteroid));
    }

    slopes_encountered.len() as u32
}

