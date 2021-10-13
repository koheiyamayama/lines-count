#[macro_use]
extern crate clap;
use clap::App;
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let input = matches.value_of("INPUT").unwrap();
    let path = Path::new(input);
    if path.is_file() {
        let count = count_lines_of_file(path);
        println!("{}", count);
    } else if path.is_dir() {
        let mut result = HashMap::new();

        let sum = count_lines_of_files(path, &mut result)
            .values()
            .sum::<usize>();
        let result_by_extension = count_lines_of_files_by_ext(&mut result);
        let result_by_extension = serde_json::to_string_pretty(&result_by_extension).unwrap();
        format_result(sum, result_by_extension)
    }
}

fn count_lines_of_file(path: &Path) -> usize {
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

fn count_lines_of_files<'a>(
    path: &Path,
    result: &'a mut HashMap<String, usize>,
) -> &'a HashMap<String, usize> {
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
            let count = count_lines_of_file(&path);
            let path = match path.as_path().to_str() {
                Some(path) => path,
                None => panic!("can't parse path."),
            };
            result.insert(path.to_string(), count);
        } else if path.is_dir() {
            count_lines_of_files(&path, result);
        }
    }

    return result;
}

// フラグ管理構造体を定義し、出力を管理する
fn format_result(sum: usize, result_by_extension: String) {
    println!("{{sum: {}}}", sum);
    println!("{}", result_by_extension);
}

fn count_lines_of_files_by_ext(result: &mut HashMap<String, usize>) -> HashMap<String, usize> {
    let mut result_by_ext: HashMap<String, usize> = HashMap::new();
    for (path, count) in result {
        let extension = Path::new(path)
            .extension()
            .unwrap_or(OsStr::new("no_extension"));
        *result_by_ext
            .entry(String::from(extension.to_str().unwrap()))
            .or_insert(*count) += *count;
    }
    return result_by_ext;
}
