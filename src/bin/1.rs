
use std::{env, fs};
use itertools::Itertools;

fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("samples/1.txt", 0)
    } else {
        ("inputs/1.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.split('\n').map(|s| s.parse::<i32>().unwrap());
    let (a, b) = input_lines.clone().cartesian_product(input_lines.clone()).find(|(a, b)|
        a+b == 2020).unwrap();

    println!("{a}, {b}");
    println!("{}", a * b);

    let (a, b, c) = input_lines.clone().cartesian_product(input_lines.clone()).cartesian_product(input_lines).filter_map(|((a, b), c)|
        if a+b+c == 2020 { Some((a, b, c))} else { None }).next().unwrap();

    println!("{a}, {b}, {c}");
    println!("{}", a * b * c);
}
