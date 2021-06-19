use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}: {}", path, e.to_string()),
    };
    println!("{:?}", file);
}
