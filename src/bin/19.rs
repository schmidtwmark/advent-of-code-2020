fn main() {
    println!("Run unit tests with cargo test");
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;
    use std::fs;

    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    enum Rule {
        Terminal(char),
        NonTerminal(Vec<Vec<usize>>),
    }

    impl Rule {
        fn from_str(line: &str) -> (usize, Rule) {
            let (key, rest) = line.split_once(": ").unwrap();

            let rule = if rest.starts_with('"') {
                Rule::Terminal(rest.chars().nth(1).unwrap())
            } else {
                let groups = rest.split(" | ");
                Rule::NonTerminal(
                    groups
                        .map(|g| {
                            g.split(' ')
                                .map(|n| n.parse::<usize>().unwrap())
                                .collect_vec()
                        })
                        .collect_vec(),
                )
            };

            (key.parse().unwrap(), rule)
        }
    }
    fn part_one(lines: Vec<String>, _param: usize) -> usize {
        let (rules, targets) = lines.split(|s| s == "").collect_tuple().unwrap();
        let rules: HashMap<usize, Rule> = rules.iter().map(|s| Rule::from_str(s)).collect();

        targets
            .iter()
            .filter(|target| {
                let solutions = process_rule(&rules, 0, target, 0);
                solutions.contains(&"")
            })
            .count()
    }
    pub fn simple_rule(rule_char: char, input: &str) -> Option<&str> {
        input
            .chars()
            .next()
            .filter(|c2| *c2 == rule_char)
            .map(|_| &input[1..])
    }

    // Takes a bunch of alternate rule index chains, We must try each possibility
    fn chains<'a>(
        chains: &[Vec<usize>],
        rules: &HashMap<usize, Rule>,
        input: &'a str,
        indent: usize,
    ) -> Vec<&'a str> {
        chains
            .iter()
            .flat_map(|this_chain| chain(this_chain, rules, input, indent + 1))
            .collect()
    }

    fn chain<'a>(
        chain: &[usize],
        rules: &HashMap<usize, Rule>,
        input: &'a str,
        indent: usize,
    ) -> Vec<&'a str> {
        chain
            .iter()
            // Try to go through all the links in the chain
            .try_fold(vec![input], |solutions, index| {
                // For each previous output, reuse it as an input to process this link in the chain
                let new_solutions: Vec<&str> = solutions
                    .iter()
                    // Find all the possibilites that match using each of the
                    // previous inputs, and the next rule index in the chain,
                    // then flatten them into the possible output solutions (which
                    // will be used for input to the next link in the chain, or for
                    // the last link, returned)
                    .flat_map(|input| process_rule(rules, *index, input, indent))
                    .collect();
                if new_solutions.is_empty() {
                    // If the next link, using the previous output as input, found no solutions
                    // The chain is broken
                    None
                } else {
                    // Continue to the next link, providing all the
                    // solutions/remainders we've found so far as its input
                    Some(new_solutions)
                }
            })
            .unwrap_or_else(Vec::new)
    }

    fn process_rule<'a>(
        rules: &HashMap<usize, Rule>,
        index: usize,
        input: &'a str,
        indent: usize,
    ) -> Vec<&'a str> {
        let rule = &rules[&index];
        log::debug!(
            "{:-indent$}Checking: {:2}: {:?} Input: {}",
            "",
            index,
            rule,
            input,
            indent = indent,
        );
        let result: Vec<&str> = match rule {
            // Match a single char
            Rule::Terminal(c) => simple_rule(*c, input).into_iter().collect(),
            // Or match a chain
            Rule::NonTerminal(indexes) => chains(indexes, rules, input, indent),
        };
        log::debug!(
            "{:-indent$}{} {:?}",
            "",
            !result.is_empty(),
            result,
            indent = indent,
        );
        // Return whatever's left over from the input
        result
    }

    fn part_two(lines: Vec<String>, _param: usize) -> usize {
        let (rules, targets) = lines.split(|s| s == "").collect_tuple().unwrap();
        let mut rules: HashMap<usize, Rule> = rules.iter().map(|s| Rule::from_str(s)).collect();

        // 8: 42 | 42 8
        // 11: 42 31 | 42 11 31

        *rules.get_mut(&8).unwrap() = Rule::NonTerminal(vec![vec![42], vec![42, 8]]);
        *rules.get_mut(&11).unwrap() = Rule::NonTerminal(vec![vec![42, 31], vec![42, 11, 31]]);

        targets
            .iter()
            .filter(|target| {
                let solutions = process_rule(&rules, 0, target, 0);
                solutions.contains(&"")
            })
            .count()
    }

    fn get_filename(sample: bool) -> &'static str {
        if sample {
            "samples/19.txt"
        } else {
            "inputs/19.txt"
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
    fn test_part_one_sample1() {
        let result = part_one(get_lines(get_filename(true)), SAMPLE_PARAM);
        println!("Part one sample: {:?}", result);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_part_one_sample2() {
        let result = part_one(get_lines("samples/19.1.txt"), SAMPLE_PARAM);
        println!("Part one sample2: {:?}", result);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(get_lines(get_filename(false)), REAL_PARAM);
        println!("Part one real: {:?}", result);
        assert_eq!(result, 222);
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(get_lines("samples/19.1.txt"), SAMPLE_PARAM);
        println!("Part two sample: {:?}", result);
        // bbabbbbaabaabba
        // babbbbaabbbbbabbbbbbaabaaabaaa
        // aaabbbbbbaaaabaababaabababbabaaabbababababaaa
        // bbbbbbbaaaabbbbaaabbabaaa
        // bbbababbbbaaaaaaaabbababaaababaabab
        // ababaaaaaabaaab
        // ababaaaaabbbaba
        // baabbaaaabbaaaababbaababb
        // abbbbabbbbaaaababbbbbbaaaababb
        // aaaaabbaabaaaaababaa
        // aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
        // aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(get_lines(get_filename(false)), REAL_PARAM);
        println!("Part two real: {:?}", result);
        assert_eq!(result, 0);
    }
}
