fn main() {
    println!("Run unit tests with cargo test");
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;
    use std::fs;

    fn part_one(sample: bool, _param: usize) -> usize {
        let _lines = get_lines(sample);
        0
    }

    fn part_two(sample: bool, _param: usize) -> usize {
        let _lines = get_lines(sample);
        0
    }

    fn get_lines(sample: bool) -> Vec<String> {
        let filename = if sample {
            "samples/aaaaa.txt"
        } else {
            "inputs/aaaaa.txt"
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
        assert_eq!(result, 0);

    }

    #[test]
    fn test_part_one() {
        let result = part_one(false, REAL_PARAM);
        println!("{:?}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_one(true, SAMPLE_PARAM);
        println!("{:?}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(false, REAL_PARAM);
        println!("{:?}", result);
        assert_eq!(result, 0);
    }
}
