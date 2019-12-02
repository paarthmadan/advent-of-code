use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

pub fn load_file(file_path: &str) -> Vec<String> {
    let mut input = Vec::new();

    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input.push(line.unwrap());
    }

    input
}

pub fn exit(message: &str) -> ! {
    println!("{}", message);
    process::exit(1);
}
