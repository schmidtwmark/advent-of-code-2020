fn main() {
    println!("Run unit tests with cargo test");
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;
    use std::{fs, ops::RangeInclusive};
    use regex::Regex;
    use std::collections::HashMap;

    type Rule = (String, Vec<RangeInclusive<usize>>);
    type Ticket = Vec<usize>;

    fn parse_lines(lines: Vec<String>) -> (Vec<Rule>, Ticket, Vec<Ticket>) {

        let groups = lines.into_iter().group_by(|line| line != "");
        let (rules, mut my_ticket, other_tickets) = groups.into_iter().filter_map(|(key, g)| if key { Some(g.filter(|line| !line.contains("ticket"))) } else { None }).collect_tuple().unwrap();

        let convert = |line: String| {
            line.split(',').map(|v| v.parse::<usize>().unwrap()).collect()
        };

        let my_ticket = convert(my_ticket.next().unwrap());
        let other_tickets = other_tickets.map(convert).collect_vec();
        let re = Regex::new(r"(.*): (\d*)-(\d*) or (\d*)-(\d*)").unwrap();
        let rules = rules.map(|rule| {
            let matches = re.captures_iter(&rule).next().unwrap();
            let name = &matches[1];
            let (a, b, c, d) = matches.iter().skip(2).take(4).map(|digit| digit.unwrap().as_str().parse::<usize>().unwrap()).collect_tuple().unwrap();
            (name.to_owned(), vec![(a..=b), (c..=d)])
        }).collect_vec();

        (rules, my_ticket, other_tickets)

    }
    fn part_one(lines: Vec<String>, _param: usize) -> usize {
        let (rules, _my_ticket, other_tickets) = parse_lines(lines);

        let ranges = rules.iter().map(|(_, ranges)| ranges).flatten().collect_vec();

        // other_tickets.iter().filter_map(|v| v.iter().filter(predicate))
        let invalids = other_tickets.iter().flatten().filter(|v| !ranges.iter().any(|r| r.contains(v) )).collect_vec();
        println!("{invalids:?}");
        invalids.into_iter().sum()
    }

    fn part_two(lines: Vec<String>, _param: usize) -> usize {
        let (rules, my_ticket, other_tickets) = parse_lines(lines);

        let ranges = rules.iter().map(|(_, ranges)| ranges).flatten().collect_vec();
        let valids = other_tickets.iter().filter(|vals| vals.iter().all(|v| ranges.iter().any(|r| r.contains(v)))).collect_vec();
        println!("Valids: {valids:?}");

        let mut mappings: HashMap<String, usize> = HashMap::new();
        while mappings.len() < rules.len() {
            println!("{mappings:?}");
            let unknown_rules = rules.iter().filter(|(s, _)| !mappings.keys().contains(s));
            println!("{:?}", unknown_rules.clone().collect_vec());

            for (rule_name, reqs) in unknown_rules {
                let unknown_indices = (0..rules.len()).filter(|i| !mappings.values().contains(i));
                println!("{:?}", unknown_indices.clone().collect_vec());
                let works_for_indices= unknown_indices.filter(|i| valids.iter().all(|v| reqs.iter().any(|r| r.contains(&v[*i])))).collect_vec();
                if works_for_indices.len() == 1 {
                    mappings.insert(rule_name.clone(), works_for_indices[0]);
                    break;
                } 
            }
        } 

        println!("Final mappings: {mappings:?}");

        let ticket_vals : HashMap<_,_> = mappings.iter().map(|(k, v)| (k, my_ticket[*v])).collect();
        println!("Ticket vals: {ticket_vals:?}");
        ticket_vals.iter().filter_map(|(k, v)| if k.starts_with("departure") { Some(v) } else { None}).product()

    }

    fn get_lines(sample: bool) -> Vec<String> {
        let filename = if sample {
            "samples/16.txt"
        } else {
            "inputs/16.txt"
        };

        let contents = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
        contents.lines().map(|s| s.to_owned()).collect_vec()
    }

    const SAMPLE_PARAM: usize = 0;
    const REAL_PARAM: usize = 0;

    #[test]
    fn test_part_one_sample() {
        let result = part_one(get_lines(true), SAMPLE_PARAM);
        println!("Part one sample: {:?}", result);
        assert_eq!(result, 71);

    }

    #[test]
    fn test_part_one() {
        let result = part_one(get_lines(false), REAL_PARAM);
        println!("Part one real: {:?}", result);
        assert_eq!(result, 24980);
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(get_lines(true), SAMPLE_PARAM);
        println!("Part two sample: {:?}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(get_lines(false), REAL_PARAM);
        println!("Part two real: {:?}", result);
        assert_eq!(result, 809376774329);
    }
}
