use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::diff::{diff, print_diff};

mod diff;

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buff = BufReader::new(file);
    buff.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Error: expected 2 arguments, but fount {}", args.len());
        std::process::exit(1);
    }
    let file_1 = read_lines(args[1].clone());
    let file_2 = read_lines(args[2].clone());
    let _diff = diff(&file_1, &file_2);
    
    print_diff(&_diff);
}
