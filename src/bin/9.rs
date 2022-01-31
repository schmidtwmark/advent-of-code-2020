use itertools::Itertools;
use std::collections::VecDeque;
use std::{env, fs};

fn main() {
    let (filename, param) = if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("samples/9.txt", 5)
    } else {
        ("inputs/9.txt", 25)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.lines().collect_vec();

    part_one(&input_lines, param);
    part_two(&input_lines, param);
}

fn find_invalid(lines: &[&str], param: usize) -> Option<usize> {
    let nums = lines
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect_vec();
    let mut valid: VecDeque<_> = nums.iter().take(param).collect();

    for num in nums.iter().skip(param) {
        if valid
            .iter()
            .cartesian_product(valid.iter())
            .filter_map(|(a, b)| {
                let sum = *a + *b;
                if &sum == num {
                    Some(())
                } else {
                    None
                }
            })
            .next()
            .is_some()
        {
            valid.pop_front();
            valid.push_back(num);
        } else {
            return Some(*num);
        }
    }

    None
}

fn part_one(lines: &[&str], param: usize) {
    println!("Part 1: {:?}", find_invalid(lines, param));
}

fn part_two(lines: &[&str], param: usize) {
    let invalid = find_invalid(lines, param).unwrap();
    let nums = lines
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect_vec();
    
    for (idx, _) in nums.iter().enumerate() {
        let mut sum = 0;
        let range = nums.iter().skip(idx).take_while(|num| { sum += *num; sum <= invalid}).collect_vec();
        let sum : usize = range.iter().copied().sum();
        println!("Range for index {idx} is {:?} with sum {sum}", range);
        if sum == invalid { 
            if let Some(min) = range.iter().min() {
                if let Some(max) = range.iter().max() {
                    println!("Part 2: {}, min: {}, max: {}", *min + *max, min, max);
                    break;
                }
            }

        }
    }
}
