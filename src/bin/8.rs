
use std::{env, fs};
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("samples/8.txt", 0)
    } else {
        ("inputs/8.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.lines().collect_vec();

    part_one(&input_lines);
    part_two(&input_lines);
}

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64)
}

impl Instruction {
    fn from_line(line: &&str) -> Instruction {
        let (ident, param) = line.split(' ').collect_tuple().unwrap();
        let param = param.parse().unwrap();
        match ident {
            "nop" => Instruction::Nop(param),
            "acc" => Instruction::Acc(param),
            "jmp" => Instruction::Jmp(param),
            _ => panic!()
        }
    }
}

fn process(instructions: &[Instruction]) -> (bool, i64) {
    let mut pc = 0;
    let mut acc = 0;
    let mut visited : HashSet<i64>  = HashSet::new();
    while (pc as usize) < instructions.len() {
        visited.insert(pc);
        pc = pc + match instructions[pc as usize] {
            Instruction::Nop(_) => {1},
            Instruction::Acc(v) => {acc += v; 1} ,
            Instruction::Jmp(v) => {v}
        };
        if visited.contains(&pc) {
            return (false, acc);
        }
    }
    (true, acc)
}

fn part_one(lines: &[&str]) {
    let instructions = lines.iter().map(Instruction::from_line).collect_vec();
    let (_finished, acc) = process(&instructions);
    println!("Part 1: {}", acc);
}

fn part_two(lines: &[&str]) {
    let instructions = lines.iter().map(Instruction::from_line).collect_vec();

    for (i, instruction ) in instructions.iter().enumerate(){
        if let Some(new_instruction) = match instruction {
            Instruction::Nop(v) => Some(Instruction::Jmp(*v)),
            Instruction::Acc(_) => None,
            Instruction::Jmp(v) => Some(Instruction::Nop(*v))
        } {
            let mut new_instructions = instructions.clone();
            new_instructions[i] = new_instruction;
            let (succeeded, acc) = process(&new_instructions);
            if succeeded {
                println!("Changed instruction at index {i}");
                println!("Part 2: {}", acc);
                break;
            }
        }
    }

}