use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("e: {}", e);
            panic!("couldn't open {:?}: {}", path, e.to_string());
        }
    };
    println!("aaaa");

    println!("{:?}", file);
    let file = BufReader::new(file);
    println!("{:?}", file);

    let count = file.lines().count();
    println!("{}", count);
}
