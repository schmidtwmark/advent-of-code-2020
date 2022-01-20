use itertools::Itertools;
use std::{env, fs, collections::HashMap};


fn main() {
    let (filename, _sample_param) = if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("samples/5.txt", 0)
    } else {
        ("inputs/5.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.lines().collect_vec();

    part_one(&input_lines);
    part_two(&input_lines);
}

type Seat = (usize, usize);

fn to_seats(lines: &[&str]) -> Vec<Seat> {
    lines.iter().map(|line| {
        let binary : usize = line.chars().map(|c| match c {
            'B' => 1,
            'F' => 0,
            'R' => 1,
            'L' => 0,
            _ => panic!()
        }).fold(0, |acc, i| { (acc << 1) | i });
        let col = binary & 0b111;
        let row = binary >> 3;
        (row, col)
    }).collect()
}

fn part_one(lines: &[&str]) {
    let seats = to_seats(lines);
    let max = seats.iter().map(|(row, col)| row * 8 + col).max();
    println!("{max:?}");
}

fn part_two(lines: &[&str]) {
    let seats = to_seats(lines);

    let row_map : HashMap<usize, Vec<usize>> = seats.iter().fold(HashMap::new(), |mut m, (row, col)| {
        m.entry(*row).or_default().push(*col);
        m
    });
    let row_missing_seat= row_map.iter().filter_map(|(row, taken)| {
        if taken.len() == 7 {
            let mut out = taken.clone();
            out.sort_unstable();
            Some((row, out))
        } else {
            None
        }
    }).next().unwrap();
    println!("{row_missing_seat:?}");
    let (row, seats_taken) = row_missing_seat;
    let col = seats_taken.iter().enumerate().find(|(i, seat)| i != *seat).unwrap().1 - 1;

    // seats.sort_by(|(row_a, col_a), (row_b, col_b)| row_a.cmp(row_b));
    println!("Your seat is: {row} {col}");
    println!("{}", row * 8 + col);
}
