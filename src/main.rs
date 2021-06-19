use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}: {}", path, e.to_string()),
    };

    let file = BufReader::new(file);

    let count = file.lines().count();
    println!("{}", count);
}
