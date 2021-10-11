use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    if path.is_file() {
        let count = lines_count_file(path);
        println!("{}", count);
    } else if path.is_dir() {
        find_entries(path);
    }
}

fn lines_count_file(path: &Path) -> usize {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            panic!("couldn't open {:?}: {}", path, e.to_string());
        }
    };
    let file = BufReader::new(file);
    let count = file.lines().count();
    count
}

fn find_entries(path: &Path) {
    let entries = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(error) => panic!("error: {:?}", error),
    };
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(error) => panic!("error: {:?}", error),
        };
        let path = entry.path();
        if path.is_file() {
            let count = lines_count_file(&path);
            let path = match path.as_path().to_str() {
                Some(path) => path,
                None => panic!("can't parse path."),
            };
            println!("{}: {}", path, count);
        } else if path.is_dir() {
            find_entries(&path);
        }
    }
}
