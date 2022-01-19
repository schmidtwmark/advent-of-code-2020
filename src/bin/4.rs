#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use std::{env, fs};
use std::collections::HashSet;
use std::iter::FromIterator;
use regex::Regex;

fn main() {
    let (filename, _sample_param) = if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("samples/4.txt", 0)
    } else {
        ("inputs/4.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.lines().collect_vec();

    part_one(&input_lines);
    part_two(&input_lines);
}

fn to_passport<'a>(lines: &[&'a str]) -> Vec<Vec<(&'a str, &'a str)>> {
    let passports: Vec<Vec<(&str, &str)>> = lines
        .iter()
        .group_by(|line| !line.is_empty())
        .into_iter()
        .filter_map(|(key, group)| {
            if key {
                let m: Vec<(&str, &str)> = group
                    .flat_map(|s| {
                        // split into kvps
                        // recombine into Vec of kvps
                        s.split(' ') 
                            .map(|kvp| kvp.split(':').collect_tuple::<(&str, &str)>().unwrap())
                    })
                    .collect_vec();
                Some(m)
            } else {
                None
            }
        })
        .collect_vec();
    passports
}
lazy_static! {
    static ref HEIGHT_PARAMS: HashSet<&'static str> = {
        let mut h = HashSet::new();
        h.insert("cm");
        h.insert("in");
        h
    };
}
lazy_static! {
    static ref EYE_COLORS: HashSet<&'static str> = {
        let mut h = HashSet::new();
        h.insert("amb");
        h.insert("blu");
        h.insert("brn");
        h.insert("gry");
        h.insert("grn");
        h.insert("hzl");
        h.insert("oth");
        h
    };
}

lazy_static! {
    static ref REQUIRED: HashSet<&'static str> = {
        let mut h = HashSet::new();
        h.insert("byr");
        h.insert("eyr");
        h.insert("hgt");
        h.insert("hcl");
        h.insert("ecl");
        h.insert("pid");
        h.insert("iyr");
        h
    };
}

fn validate_passport_1(passport: &[(&str, &str)] ) -> bool {
    let key_set: HashSet<&str>  = HashSet::from_iter(passport.iter().map(|(key, _)| *key));
    key_set.is_superset(&REQUIRED)
}

fn part_one(lines: &[&str]) {
    let passports = to_passport(lines);
    let valid_count = passports.iter().filter(|p| validate_passport_1(p)).count();
    println!("{valid_count}");
}
fn validate_passport_2(passport: &[(&str, &str)] ) -> bool {
    let key_set: HashSet<&str>  = HashSet::from_iter(passport.iter().map(|(key, _)| *key));
    let valid =  key_set.is_superset(&REQUIRED);
    valid && passport.iter().all(|(key, val)| {
        let yr = val.parse::<usize>();
        match *key {
            "byr" => yr.map_or(false, |y| (1920..=2002).contains(&y)),
            "iyr" => yr.map_or(false, |y| (2010..=2020).contains(&y)),
            "eyr" => yr.map_or(false, |y| (2020..=2030).contains(&y)),
            "hgt" => { 
                let (num, ident) = val.split_at(val.len() - 2);

                let n = num.parse::<usize>().unwrap_or(0);
                if ident == "cm" { 
                    (150..=193).contains(&n)
                } else if ident == "in" {
                    (59..=76).contains(&n)
                } else {
                    false
                }
            },
            "hcl" => {
                let re = Regex::new(r"^#[\da-f]{6}$").unwrap();
                re.is_match(val)
            },
            "ecl" => EYE_COLORS.contains(val),
            "pid" => {
                let re = Regex::new(r"^\d{9}$").unwrap();
                re.is_match(val)
            } ,
            _ => true
        }
    })
}

fn part_two(lines: &[&str]) {
    let passports = to_passport(lines);
    let valid_count = passports.iter().filter(|p| validate_passport_2(p)).count();
    println!("{valid_count}");
}
