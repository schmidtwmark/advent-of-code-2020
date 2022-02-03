fn main() {
    println!("Run unit tests with cargo test");
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;
    use std::fs;
    use std::collections::HashMap;

    fn part_one(sample: bool, _param: usize) -> usize {
        let mut lines = get_lines(sample);
        lines.insert(0, "0".to_owned()); // gross
        let adapters = lines.iter().map(|line| line.parse::<usize>().unwrap()).sorted();
        let jumps : HashMap<usize, usize> = adapters.tuple_windows().map(|(a, b)| b - a).counts();
        jumps[&1] * (jumps[&3] + 1)

    }

    fn part_two(sample: bool, _param: usize) -> i64{
        let lines = get_lines(sample);
        let mut adapters = lines.iter().map(|line| line.parse::<i64>().unwrap()).sorted().collect_vec();
        adapters.insert(0, 0);
        adapters.push(adapters.last().unwrap() + 3);

        adapters.reverse();
        println!("Adapters:\n{adapters:?}");

        let mut counts = vec![0; adapters.len()];
        counts[0] = 1;
        for (idx, adapter) in adapters.iter().enumerate() {
            let neighbors = adapters.iter().enumerate().skip(idx + 1).take_while(|(_idx, candidate)| *candidate >= &(adapter - 3));
            println!("Neighbors for adapter {adapter} at idx {idx} are {:?}", neighbors.clone().collect_vec());
            for (neighbor_idx, _neighbor) in neighbors {
                counts[neighbor_idx] += counts[idx]
            }


        }

        counts[counts.len() - 1]
    }

    fn get_lines(sample: bool) -> Vec<String> {
        let filename = if sample {
            "samples/10.txt"
        } else {
            "inputs/10.txt"
        };

        let contents = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
        contents.lines().map(|s| s.to_owned()).collect_vec()
    }

    const SAMPLE_PARAM: usize = 0;
    const REAL_PARAM: usize = 0;

    #[test]
    fn test_part_one_sample() {
        let result = part_one(true, SAMPLE_PARAM);
        println!("{:?}", result);
        assert_eq!(result, 35);

    }

    #[test]
    fn test_part_one() {
        let result = part_one(false, REAL_PARAM);
        println!("{:?}", result);
        assert_eq!(result, 2272);
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(true, SAMPLE_PARAM);
        println!("{:?}", result);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(false, REAL_PARAM);
        println!("{:?}", result);
        assert_eq!(result, 0);
    }
}
