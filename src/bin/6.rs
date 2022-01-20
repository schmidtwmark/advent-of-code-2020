use std::{env, fs, collections::HashSet};
use std::iter::FromIterator;
use itertools::Itertools;
use std::ops::{BitOr, BitAnd};

fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("samples/6.txt", 0)
    } else {
        ("inputs/6.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.lines().collect_vec();

    part_one(&input_lines);
    part_two(&input_lines);
}

fn to_groups<'a>(lines: &[&'a str]) -> Vec<Vec<HashSet<char>>>{
    lines.iter().group_by(|line| !line.is_empty()).into_iter().filter_map(|(key, val)| {
        if key { Some(val.map(|s| HashSet::from_iter(s.chars())).collect_vec()) } else { None }
    }).collect_vec()
}

fn flatten_groups(groups: &[Vec<HashSet<char>>]) -> Vec<HashSet<char>>{
    groups.iter().map(|people| people.iter().fold(HashSet::new(), |h, person| {
        h.bitor(person)
    })).collect_vec()
}

fn flatten2_groups(groups: &[Vec<HashSet<char>>]) -> Vec<HashSet<char>>{
    groups.iter().map(|people| people.iter().fold(None, |h, person| {
        match h {
            None => Some(person.clone()),
            Some(hi) => Some(hi.bitand(person))
        }
    }).unwrap()).collect_vec()
}
fn part_one(lines: &[&str]) {
    let groups = to_groups(lines);
    let flattened = flatten_groups(&groups);
    let count : usize = flattened.iter().map(|s| s.len()).sum();
    println!("P1: {count:?}");
}

fn part_two(lines: &[&str]) {
    let groups = to_groups(lines);
    let flattened = flatten2_groups(&groups);
    let count : usize = flattened.iter().map(|s| s.len()).sum();
    println!("P2: {count:?}");

}