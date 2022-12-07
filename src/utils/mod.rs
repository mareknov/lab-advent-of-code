use std::{env, io};
use std::fs::File;
use std::io::BufRead;
use colour::blue_ln;

pub fn input_path(module: &str) -> String {
    format!("{}/src/{}", env::current_dir().unwrap().display(), module)
}

pub fn read_input(file_path: String) -> Vec<String> {
    let file = File::open(file_path).expect("Input file does not exists");

    io::BufReader::new(file)
        .lines()
        .map(|line| { line.expect("Cannot read line") })
        .collect()
}

pub fn read_as_str(file_path: String) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

pub fn to_int(line: &String) -> i32 {
    line.parse().expect(&*format!("Not a number {}", line))
}

pub fn log_setup(name: &str, in_file: &str) {
    blue_ln!("=======================");
    blue_ln!("{}", name);
    blue_ln!("=======================");
    println!("input file:\t{}", in_file);
}
