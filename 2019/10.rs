mod helper;

use helper::exit;
use std::collections::{HashMap, HashSet};
use std::{env, fmt};

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Copy, Clone)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Asteroid {
    fn new(x: i32, y: i32) -> Self {
        Asteroid { x, y }
    }

    fn magnitude_between(a: &Self, b: &Self) -> i32 {
        (a.x - b.x).abs() + (a.y - b.y).abs()
    }
}

impl fmt::Display for Asteroid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
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
        let dx = b.x as i32 - a.x as i32; 
        let dy = a.y as i32 - b.y as i32; 
        let px = dx < 0;
        let py = dy < 0;
        if dx == 0 {
            Slope::new(dx, dy, py)
        } else {
            Slope::new(dx, dy, px)
        }
    }

    fn polarity_as_f32(&self) -> f32 {
        match self.pole {
            true => -1.0,
            false => 1.0,
        }
    }

    fn normalized_slope(&self) -> f32 {
       self.delta_y as f32 / self.delta_x as f32
    }
}

impl fmt::Display for Slope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Slope: {} / {}, Normalized: {}, Pole: {}", self.delta_y, self.delta_x, self.normalized_slope(), self.pole)
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

struct MonitoringStation {
    origin_asteroid: Asteroid,
    map: HashMap<Slope, Vec<Asteroid>>,
}

impl MonitoringStation {
    fn new(origin_asteroid: Asteroid, map: HashMap<Slope, Vec<Asteroid>>) -> Self {
        MonitoringStation { origin_asteroid, map }
    }
}

impl fmt::Display for MonitoringStation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Origin Asteroid: {} | Count: {}", self.origin_asteroid, self.map.len())
    }
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
                asteroids.insert(Asteroid::new(x as i32, y as i32));
            }
        }
    }

    let order = pt_2(pt_1(&asteroids));
    let answer = order[199];
    println!("{}", answer);
    println!("{}", answer.x * 100 + answer.y);
}

fn pt_1(asteroids: &HashSet<Asteroid>) -> MonitoringStation {
    let mut max_count = 0;
    let mut monitoring_station = None;

    for asteroid in asteroids {
        let map = find_detectable_asteroids(&asteroid, &asteroids);
        let count = map.len();
        if count > max_count {
            max_count = count;
            monitoring_station = Some(MonitoringStation::new(*asteroid, map));
        }
    }

    let monitoring_station = monitoring_station.unwrap();

    println!("{:#?}", monitoring_station.map);

    println!("{}", monitoring_station);
    monitoring_station
}

fn pt_2(mut monitoring_station: MonitoringStation) -> Vec<Asteroid> {
    let mut positive_keys: Vec<_> = monitoring_station.map.keys().cloned().filter(|s| !s.pole).collect();
    let mut negative_keys: Vec<_> = monitoring_station.map.keys().cloned().filter(|s| s.pole).collect();

    positive_keys.sort_by(|a, b| b.normalized_slope().partial_cmp(&a.normalized_slope()).unwrap());
    negative_keys.sort_by(|a, b| b.normalized_slope().partial_cmp(&a.normalized_slope()).unwrap());

    let mut keys = Vec::new();

    keys.append(&mut positive_keys);
    keys.append(&mut negative_keys);

    for key in &keys {
        let origin = &monitoring_station.origin_asteroid;
        let vals = monitoring_station.map.get_mut(&key).unwrap();
        vals.sort_by(
            |a, b| Asteroid::magnitude_between(&origin, &a)
            .partial_cmp(
                &Asteroid::magnitude_between(&origin, &b)
            ) .unwrap()
        );
    }

    let mut asteroids_ordered: Vec<Asteroid> = Vec::new();

    loop {
        let mut changed = false;
        for key in &keys {
            let vals = monitoring_station.map.get_mut(&key).unwrap();
            if vals.len() > 0 {
                asteroids_ordered.push(vals.remove(0));
                changed = true;
            }
        }
        if !changed { break; }
    }

    asteroids_ordered
}

fn find_detectable_asteroids(origin: &Asteroid, asteroids: &HashSet<Asteroid>) -> HashMap<Slope, Vec<Asteroid>> {
    let mut slopes_encountered: HashMap<Slope, Vec<Asteroid>> = HashMap::new();

    for asteroid in asteroids {
        if *asteroid == *origin {
            continue;
        }
        let current_slope = Slope::between(origin, asteroid);
        let current_slope_list = slopes_encountered.entry(current_slope).or_insert(Vec::new());
        current_slope_list.push(*asteroid);
    }

    slopes_encountered
}
