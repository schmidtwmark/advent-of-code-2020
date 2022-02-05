fn main() {
    println!("Run unit tests with cargo test");
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;
    use std::fs;
    use num::integer::lcm;
    use std::collections::HashSet;

    fn part_one(sample: bool, _param: usize) -> usize {
        let lines = get_lines(sample);
        let start: usize = lines[0].parse().unwrap();
        let buses = lines[1].split(',').filter_map(|s| s.parse::<usize>().ok()).collect_vec();

        let mut i = start;
        loop {
            if let Some(bus) = buses.iter().find(|bus| i % *bus == 0){
                println!("Bus {bus} works at time {i}");
                return (i - start) * bus
            }
            i += 1;
        }

    }

    fn part_two(sample: bool, _param: usize) -> usize {
        let lines = get_lines(sample);
        let buses = lines[1].split(',').enumerate().filter_map(|(idx, s)| s.parse::<usize>().ok().map(|i| (idx, i))).collect_vec();
        // let (max_idx, max) = buses.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let sorted = buses.iter().sorted_by(|a, b| a.1.cmp(&b.1)).collect_vec();
        let (max_idx, max) = sorted[0];

        let mut timestamp = *max;

        let mut known_idx :HashSet<usize> = HashSet::new();
        known_idx.insert(*max_idx);
        let mut least_common_multiple= *max;
        loop {
            // Check if all buses match
            let correct = sorted.iter().filter(|(idx, bus)| {
                let diff = *idx as i64  - *max_idx as i64;
                let target = timestamp as i64 + diff;
                target as usize % bus == 0
            });

            if correct.clone().count() == sorted.len() {
                return timestamp - max_idx;
            }

            // if any are correct, increment lcm of all known
            least_common_multiple = correct.filter(|(idx, _)| {
                !known_idx.contains(idx)
            }).fold(least_common_multiple, |acc, (_, bus)| lcm(acc, *bus));


            timestamp += least_common_multiple;

        }

    }

    fn get_lines(sample: bool) -> Vec<String> {
        let filename = if sample {
            "samples/13.txt"
        } else {
            "inputs/13.txt"
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
        assert_eq!(result, 295);

    }

    #[test]
    fn test_part_one() {
        let result = part_one(false, REAL_PARAM);
        println!("{:?}", result);
        assert_eq!(result, 4782);
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(true, SAMPLE_PARAM);
        println!("{:?}", result);
        assert_eq!(result, 1068781);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(false, REAL_PARAM);
        println!("{:?}", result);
        assert_eq!(result, 1118684865113056);
    }
}
