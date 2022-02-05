fn main() {
    println!("Run unit tests with cargo test");
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;
    use regex::Regex;
    use std::collections::HashMap;
    use std::fs;

    fn parse_mask(line: &str) -> (usize, usize) {
        let mask = &line[7..];

        let masker = |mut acc, c, target| {
            acc <<= 1;
            if c == target {
                acc |= 1;
            }
            acc
        };

        let up_mask = mask.chars().fold(0usize, |acc, c| masker(acc, c, '1'));
        let down_mask = mask.chars().fold(0usize, |acc, c| masker(acc, c, '0'));

        (up_mask, down_mask)
    }

    #[derive(Debug, Clone)]
    enum Command {
        Mask(usize, usize),
        Set(usize, usize),
        Mask2(String),
    }

    fn part_one(lines: Vec<String>, _param: usize) -> usize {
        let (mut up_mask, mut down_mask) = parse_mask(&lines[0]);

        let re = Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();

        let commands = lines[1..]
            .iter()
            .map(|s| {
                println!("command: {s}");
                if let Some(matches) = re.captures_iter(s).next() {
                    let (address, value) = (1..=2)
                        .map(|i| matches[i].parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    Command::Set(address, value)
                } else {
                    let (up_mask, down_mask) = parse_mask(s);
                    Command::Mask(up_mask, down_mask)
                }
            })
            .collect_vec();

        println!("Commands: {commands:?}");

        let addr_space: HashMap<usize, usize> =
            commands
                .iter()
                .fold(HashMap::new(), |mut address_space, command| {
                    match command {
                        Command::Set(address, v) => {
                            let v_up = v | up_mask;
                            let v_down = v_up & !down_mask;
                            println!("V:\t{v:#038b}\nV':\t{v_up:#038b}\nV'':\t{v_down:#038b}\n\n");

                            address_space.insert(*address, v_down);
                        }
                        Command::Mask(u, d) => {
                            up_mask = *u;
                            down_mask = *d;
                        }
                        _ => panic!(),
                    }
                    address_space
                });

        addr_space.values().sum()
    }

    fn part_two(lines: Vec<String>, _param: usize) -> usize {
        let re = Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();
        let mut mask = lines[0][7..].to_owned();

        let commands = lines[1..]
            .iter()
            .map(|s| {
                println!("command: {s}");
                if let Some(matches) = re.captures_iter(s).next() {
                    let (address, value) = (1..=2)
                        .map(|i| matches[i].parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    Command::Set(address, value)
                } else {
                    Command::Mask2(s[7..].to_owned())
                }
            })
            .collect_vec();

        let addr_space: HashMap<usize, usize> =
            commands
                .iter()
                .fold(HashMap::new(), |mut address_space, command| {
                    match command {
                        Command::Set(address, new_value) => {
                            let mut value = *address;
                            // println!("Original value:\t{value:#038b}");
                            // println!("mask: \t\t0b{mask}");
                            let mut mutated :String = mask
                                .chars()
                                .rev()
                                .map(|m| {
                                    let out = match m {
                                        'X' => 'X',
                                        '1' => '1',
                                        '0' => char::from_digit((value & 1) as u32, 2).unwrap(),
                                        _ => panic!(),
                                    };
                                    value >>= 1;
                                    out
                                })
                                .collect();

                            mutated = mutated.chars().rev().collect();

                            let floating_count = mutated.chars().filter(|c| *c == 'X').count() as u32;
                            // println!("mutated:\t0b{mutated}");
                            let addresses = (0..(2usize.pow(floating_count))).map(|mut num| {
                                // println!("num:\t{num:#038b}");
                                let out : String= mutated
                                    .chars()
                                    .map(|c| {
                                        if c == 'X' {
                                            let v = num & 1;
                                            num >>= 1;
                                            char::from_digit(v as u32, 2).unwrap()
                                        } else {
                                            c
                                        }
                                    }).collect();
                                // println!("out:\t0b{out}");
                                usize::from_str_radix(&out, 2).unwrap()
                            }).collect_vec();

                            for address in addresses {
                                // println!("address: {address} value: {new_value}");
                                address_space.insert(address, *new_value);
                            }
                        }
                        Command::Mask2(s) => {
                            mask = s.clone();
                        }
                        _ => panic!(),
                    }
                    address_space
                });

        // println!("Addr space: {addr_space:?}");

        addr_space.values().sum()
    }

    fn get_filename(sample: bool) -> &'static str {
        if sample {
            "samples/14.txt"
        } else {
            "inputs/14.txt"
        }
    }

    fn get_lines(filename: &str) -> Vec<String> {
        let contents = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
        contents.lines().map(|s| s.to_owned()).collect_vec()
    }

    const SAMPLE_PARAM: usize = 0;
    const REAL_PARAM: usize = 0;

    #[test]
    fn test_part_one_sample() {
        let result = part_one(get_lines(get_filename(true)), SAMPLE_PARAM);
        println!("Part one sample: {:?}", result);
        assert_eq!(result, 165);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(get_lines(get_filename(false)), REAL_PARAM);
        println!("Part one real: {:?}", result);
        assert_eq!(result, 12512013221615);
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(get_lines("samples/14.1.txt"), SAMPLE_PARAM);
        println!("Part two sample: {:?}", result);
        assert_eq!(result, 208);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(get_lines(get_filename(false)), REAL_PARAM);
        println!("Part two real: {:?}", result);
        assert_eq!(result, 3905642473893);
    }
}
