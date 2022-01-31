
use std::{env, fs};
use itertools::Itertools;

fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("samples/aaaaa.txt", 0)
    } else {
        ("inputs/aaaaa.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.lines().collect_vec();

    part_one(&input_lines);
    part_two(&input_lines);
}

fn part_one(lines: &[&str]) {
    println!("Part 1: {}", 0)
}

fn part_two(lines: &[&str]) {
    println!("Part 2: {}", 0)
}