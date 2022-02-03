
use std::{env, fs};
use itertools::Itertools;

fn main() {

    part_one(&input_lines, param);
    part_two(&input_lines, param);
}

fn part_one(sample: bool, _param: usize) -> usize{
    let _lines = get_lines(sample);
    0
}

fn part_two(sample: bool, _param: usize) -> usize{
    let _lines = get_lines(sample);
    0
}

fn get_lines(sample: bool) {
    let (filename, param)= if sample {
        ("samples/aaaaa.txt", 0)
    } else {
        ("inputs/aaaaa.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.lines().collect_vec();

}

const SAMPLE_PARAM: usize = 0;
const REAL_PARAM: usize = 0;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one_sample() {
        let result = part_one(true, SAMPLE_PARAM);
        println!("{:?}", result)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(false, REAL_PARAM);
        println!("{:?}", result)
    }

    #[test]
    fn test_part_two_sample() {
        let result= part_one(true, SAMPLE_PARAM);
        println!("{:?}", result)
    }

    #[test]
    fn test_part_two() {
        let result= part_two(false, REAL_PARAM);
        println!("{:?}", result)
    }

}