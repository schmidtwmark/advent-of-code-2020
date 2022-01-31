
use std::{env, fs};
use itertools::Itertools;

fn main() {
    let (filename, param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("samples/aaaaa.txt", 0)
    } else {
        ("inputs/aaaaa.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.lines().collect_vec();

    part_one(&input_lines, param);
    part_two(&input_lines, param);
}

fn part_one(lines: &[&str], _param: usize) {
    println!("Part 1: {}", 0)
}

fn part_two(lines: &[&str], _param: usize) {
    println!("Part 2: {}", 0)
}