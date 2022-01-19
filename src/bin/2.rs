
use std::{env, fs, ops::RangeInclusive};
use regex::Regex;
use itertools::Itertools;


type Policy = (RangeInclusive<usize>, char);
fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("samples/2.txt", 0)
    } else {
        ("inputs/2.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));

    let re = Regex::new(r"^(\d*)-(\d*) (.): (.*)").unwrap();

    let passwords: Vec<(Policy, String)>= contents.split('\n').map(|line| {
        let matches = re.captures_iter(line).next().unwrap();
        println!("matches: {:?}", matches);
        let (a, b) = (1..=2).map(|i| matches[i].parse::<usize>().unwrap()).collect_tuple().unwrap();
        ((a..=b, matches[3].chars().next().unwrap()), matches[4].to_owned())
    }).collect_vec();


    let valid = passwords.iter().filter(|((range, c), s)| {
        range.contains(&s.chars().filter(|candidate| candidate == c).count())
    }).count();
    println!("{:?}", valid);

    let valid2 = passwords.iter().filter(|((range, c), s)| {
        let (a,b) = (range.start() - 1, range.end() - 1);
        let check = |i| s.chars().nth(i).map_or(false, |candidate| candidate == *c);
        check(a) ^ check(b)
    });
    valid2.clone().for_each(|pass| {
        println!("valid: {pass:?}");
    });
    println!("{:?}", valid2.count());




}