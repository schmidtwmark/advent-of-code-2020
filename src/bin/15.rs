fn main() {
    println!("Run unit tests with cargo test");
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;
    use std::fs;
    use std::collections::HashMap;
    use std::collections::VecDeque;

    struct Tracker {
        turns: VecDeque<usize>,
        map: HashMap<usize, VecDeque<usize>>
    }

    impl Tracker {
        fn new() -> Tracker {
            Tracker {
                turns: VecDeque::new(),
                map: HashMap::new()
            }
        }

        fn push(&mut self, number: &usize, turn: &usize) {
            self.turns.push_back(*number);
            self.map.entry(*number).or_default().push_back(*turn);

        }

        fn top(&self) -> &usize {
            self.turns.back().unwrap()
        }

        fn get_previous_appearances(&self, number: &usize) -> Option<&VecDeque<usize>> {
            self.map.get(number)
        }
    }

    fn part_one(lines: Vec<String>, _param: usize) -> usize {
        game(lines, 2020)
    }
    fn game(lines: Vec<String>, target: usize) -> usize{
        let starting_numbers= lines[0].split(',').map(|c| c.parse::<usize>().unwrap()).collect_vec();

        let mut tracker = Tracker::new();

        for turn in 0..target{
            if let Some(start) = starting_numbers.get(turn) {
                tracker.push(start, &turn);
            } else {
                let top = tracker.top();
                if let Some(previous_appearances) = tracker.get_previous_appearances(top) {
                    if previous_appearances.len() == 1 {
                        tracker.push(&0, &turn);
                    } else {
                        let (a, b) = previous_appearances.iter().rev().take(2).copied().collect_tuple().unwrap();
                        tracker.push(&(a-b), &turn);
                    }
                }
            }
        }

        *tracker.turns.back().unwrap()
    }

    fn part_two(lines: Vec<String>, _param: usize) -> usize {
        game(lines, 30000000)
    }

    fn get_lines(sample: bool) -> Vec<String> {
        let filename = if sample {
            "samples/15.txt"
        } else {
            "inputs/15.txt"
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
        assert_eq!(result, 436);

    }

    #[test]
    fn test_part_one() {
        let result = part_one(get_lines(false), REAL_PARAM);
        println!("Part one real: {:?}", result);
        assert_eq!(result, 694);
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(get_lines(true), SAMPLE_PARAM);
        println!("Part two sample: {:?}", result);
        assert_eq!(result, 175594);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(get_lines(false), REAL_PARAM);
        println!("Part two real: {:?}", result);
        assert_eq!(result, 21768614);
    }
}
