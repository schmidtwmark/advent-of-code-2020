use itertools::Itertools;
use std::{collections::{HashMap, HashSet}, env, fs};

fn main() {
    let (filename, _sample_param) = if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("samples/7.txt", 0)
    } else {
        ("inputs/7.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.lines().collect_vec();

    part_one(&input_lines);
    part_two(&input_lines);
}

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    contains: HashMap<String, usize>,
}

fn line_to_rule(line: &&str) -> Rule {
    //light red bags contain 1 bright white bag, 2 muted yellow bags.
    // 0    1   2    3       4 5      6          7
    let splat = line.split(' ');
    let name: String = itertools::Itertools::intersperse(splat.clone().take(2), " ").collect();
    let groups = splat
        .skip(4)
        .group_by(|element| !element.starts_with("bag"))
        .into_iter()
        .filter_map(|(key, mut value)| {
            if key {
                if let Ok(count) = value.next().unwrap().parse::<usize>() {
                    let ident: String = itertools::Itertools::intersperse(value.take(2), " ").collect();
                    return Some((ident, count))
                }
            }
            None
        })
        .collect();
    Rule {
        name,
        contains: groups,
    }
}

// fn flatten_rules<'a>(rules: &'a [Rule]) -> HashMap<&'a str, Vec<&'a str>>{
//     let mut map = HashMap::new();
//     for rule in rules.iter().sorted_by(|a, b| a.contains.len().cmp(&b.contains.len())){
//         // Start with empty ones first
//         map.entry(key)
//     }
//     map
// }


fn part_one(lines: &[&str]) {
    let rules : HashMap<_, _>= lines.iter().map(line_to_rule).map(|r| (r.name, r.contains)).collect();

    let rules_containing = |target: &str| -> HashSet<_>{
        rules.iter().filter_map(|(name, inner)| if inner.contains_key(target) { Some(name) } else { None }).collect()
    };

    let mut new_elements = rules_containing("shiny gold");
    let mut elements : HashSet<&String> = HashSet::new();

    while {
        elements.extend(new_elements.iter());
        new_elements = new_elements.iter().fold(HashSet::new(), |mut acc, ele| {
            acc.extend(rules_containing(ele));
            acc
        });
        
        !new_elements.is_empty()
    } {}


    println!("Part 1: {}", elements.len());
}

fn part_two(lines: &[&str]) {
    let rules : HashMap<_, _>= lines.iter().map(line_to_rule).map(|r| (r.name, r.contains)).collect();


    let mut new_elements = rules["shiny gold"].clone();
    let mut elements : HashMap<String, usize> = HashMap::new();

    while {
        new_elements.iter().for_each(|(n, c)| {
            *elements.entry(n.to_string()).or_default() += c;
        });
        // elements.extend(new_elements.clone());
        new_elements = new_elements.iter().fold(HashMap::new(), |mut acc, (bag, count)| {
            let contains: HashMap<String, usize> = rules[bag].iter().map(|(k, v)| (k.to_owned(), v * count)).collect();
            contains.iter().for_each(|(n, c)| {
                *acc.entry(n.to_string()).or_default() += c;
            });
            acc
        });
        
        !new_elements.is_empty()
    } {}

    let count : usize = elements.iter().map(|(_k, v)| v).sum();

    println!("Part 2: {}", count)
}
