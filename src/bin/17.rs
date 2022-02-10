fn main() {
    println!("Run unit tests with cargo test");
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;
    use std::collections::HashSet;
    use std::fs;

    type Point3D = (i32, i32, i32);
    type Point4D = (i32, i32, i32, i32);

    fn get_neighbors(point: &Point3D) -> HashSet<Point3D> {
        let (x, y, z) = point;
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .filter_map(|((dx, dy), dz)| {
                if dx == 0 && dy == 0 && dz == 0 {
                    None
                } else {
                    Some((x + dx, y + dy, z + dz))
                }
            })
            .collect()
    }

    fn get_neighbors4(point: &Point4D) -> HashSet<Point4D> {
        let (x, y, z, w) = point;
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .filter_map(|(((dx, dy), dz), dw)| {
                if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                    None
                } else {
                    Some((x + dx, y + dy, z + dz, w+ dw))
                }
            })
            .collect()
    }

    fn part_one(lines: Vec<String>, _param: usize) -> usize {
        let mut activated_points: HashSet<Point3D> = lines
            .iter()
            .enumerate()
            .map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .filter_map(|(x, c)| match c {
                        '#' => Some((x as i32, y as i32, 0)),
                        _ => None,
                    })
                    .collect_vec()
            })
            .flatten()
            .collect();

        for step in 1..=6 {
            println!("Step {step}, activated: {}", activated_points.len());
            let to_consider: HashSet<_> =
                activated_points
                    .iter()
                    .fold(activated_points.clone(), |mut acc, p| {
                        acc.extend(get_neighbors(p));
                        acc
                    });
            activated_points = to_consider
                .into_iter()
                .filter(|p| {
                    let neighbors = get_neighbors(p);

                    let activated_neighbors: HashSet<_> = neighbors
                        .iter()
                        .filter(|n| activated_points.contains(n))
                        .collect();
                    if activated_points.contains(p) {
                        activated_neighbors.len() >= 2 && activated_neighbors.len() <= 3
                    } else {
                        activated_neighbors.len() == 3
                    }
                })
                .collect();
        }

        activated_points.len()
    }

    fn part_two(lines: Vec<String>, _param: usize) -> usize {
        let mut activated_points: HashSet<Point4D> = lines
            .iter()
            .enumerate()
            .map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .filter_map(|(x, c)| match c {
                        '#' => Some((x as i32, y as i32, 0, 0)),
                        _ => None,
                    })
                    .collect_vec()
            })
            .flatten()
            .collect();

        for step in 1..=6 {
            println!("Step {step}, activated: {}", activated_points.len());
            let to_consider: HashSet<_> =
                activated_points
                    .iter()
                    .fold(activated_points.clone(), |mut acc, p| {
                        acc.extend(get_neighbors4(p));
                        acc
                    });
            activated_points = to_consider
                .into_iter()
                .filter(|p| {
                    let neighbors = get_neighbors4(p);

                    let activated_neighbors: HashSet<_> = neighbors
                        .iter()
                        .filter(|n| activated_points.contains(n))
                        .collect();
                    if activated_points.contains(p) {
                        activated_neighbors.len() >= 2 && activated_neighbors.len() <= 3
                    } else {
                        activated_neighbors.len() == 3
                    }
                })
                .collect();
        }

        activated_points.len()
    }

    fn get_filename(sample: bool) -> &'static str {
        if sample {
            "samples/17.txt"
        } else {
            "inputs/17.txt"
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
        assert_eq!(result, 112);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(get_lines(get_filename(false)), REAL_PARAM);
        println!("Part one real: {:?}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(get_lines(get_filename(true)), SAMPLE_PARAM);
        println!("Part two sample: {:?}", result);
        assert_eq!(result, 848);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(get_lines(get_filename(false)), REAL_PARAM);
        println!("Part two real: {:?}", result);
        assert_eq!(result, 2296);
    }
}
