
use std::{env, fs};
use itertools::Itertools;

fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("samples/3.txt", 0)
    } else {
        ("inputs/3.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.lines().collect_vec();

    part_one(&input_lines);
    part_two(&input_lines);
}

fn tree_count(lines: &[&str], dx: usize, dy: usize) -> usize {
    let trees : Vec<Vec<bool>>= lines.iter()
        .map(|line| 
            line.chars().map(|c| c == '#').collect_vec()).collect_vec();
    let (mut x, mut y) = (0, 0);
    let width = trees[0].len();
    let mut tree_count = 0;
    while y < trees.len() {
        if trees[y][x] { tree_count += 1; }
        y += dy;
        x = (x + dx) % width;
    }
    tree_count


}

fn part_one(lines: &[&str]) {
    println!("{}", tree_count(lines, 3, 1))
}

fn part_two(lines: &[&str]) {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let prod : usize = slopes.iter().map(|(dx, dy)| tree_count(lines, *dx, *dy)).product();
    println!("prod: {prod}" );
}