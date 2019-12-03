mod helper;

use helper::exit;
use std::{collections::HashSet, env};

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Point(i32, i32);

impl Point {
    fn origin() -> Point {
        Point(0, 0) 
    }
    fn manhattan_distance(&self) -> u32 {
        self.0.abs() as u32 + self.1.abs() as u32
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit("Usage: 03 <file>");
    };

    let file = &helper::load_file(&args[1]);
    let wire_points: Vec<Vec<Point>> = file.into_iter().map(|line| extract_points(line)).collect();

    let mut set_a: HashSet<Point> = HashSet::new();
    let mut set_b: HashSet<Point> = HashSet::new();

    for point in &wire_points[0] { set_a.insert(*point); }
    for point in &wire_points[1] { set_b.insert(*point); }

    let min = set_a.intersection(&set_b).into_iter().min_by(|x, y| *(&x.manhattan_distance().cmp(&y.manhattan_distance()))).unwrap();

    println!("{:#?}", min);
    println!("{}", min.manhattan_distance());
}

fn extract_points(line: &str) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let mut last_point = Point::origin();
    line.split(",").for_each(|code| create_points_from_code(code, &mut last_point, &mut points));
    points
}

fn create_points_from_code(code: &str, last_point: &mut Point, points: &mut Vec<Point>) {
    let direction = code.chars().nth(0).unwrap();
    let displacement = code[1..].parse::<i32>().unwrap();
    let (x, y) = (last_point.0, last_point.1);
    match direction {
        'U' => for py in (y + 1)..=(y + displacement) { points.push(Point(x, py)); }
        'D' => for py in ((y - displacement)..=(y - 1)).rev() { points.push(Point(x, py)); },
        'R' => for px in (x + 1)..=(x + displacement) { points.push(Point(px, y)); },
        'L' => for px in ((x - displacement)..=(x - 1)).rev() { points.push(Point(px, y)); },
        _ => exit("Parsed unknown code"),
    }
    let new_last_point = points.last().unwrap();
    *last_point = Point(new_last_point.0, new_last_point.1);
}
