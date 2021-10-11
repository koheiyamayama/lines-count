use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    if path.is_file() {
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
    } else if path.is_dir() {
        let entries = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(e) => panic!("error: {:?}", e),
        };
        for entry in entries {
            println!("{:?}", entry);
        }
    }
}
