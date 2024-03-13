use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use rdiff::diff::diff;

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buff = BufReader::new(file);
    buff.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

#[test]
fn test_diff_filles() {
    let file_1 = read_lines("./tests/resources/large.txt");
    let file_2 = read_lines("./tests/resources/large_ch.txt");
    let _diff = diff(&file_1, &file_2);
}