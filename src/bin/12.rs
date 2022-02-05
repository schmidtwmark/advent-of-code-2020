
extern crate num;
#[macro_use]
extern crate num_derive;

fn main() {
    println!("Run unit tests with cargo test");
}


#[derive(FromPrimitive, Copy, Clone, Debug)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}
#[cfg(test)]
mod tests {

    use super::*;
    use itertools::Itertools;
    use std::fs;
    use num::complex;

    #[derive(Debug, Clone)]
    enum Command {
        Forward(i32),
        Turn(i32),
        Cardinal(Direction, i32),
    }

    impl Command {
        fn from_line(line: &String ) -> Command {
            let (ident, value) = line.split_at(1);
            let value = value.parse::<i32>().unwrap();
            match ident {
                "N" => Command::Cardinal(Direction::North, value),
                "S" => Command::Cardinal(Direction::South, value),
                "E" => Command::Cardinal(Direction::East, value),
                "W" => Command::Cardinal(Direction::West, value),
                "L" => Command::Turn(-value),
                "R" => Command::Turn(value),
                "F" => Command::Forward(value),
                _ => panic!()
            }
        }
    }

    impl Direction {
        fn offset(&self, value: i32) -> (i32, i32) {
            match self {
                Direction::North => (0, value),
                Direction::South=> (0, -value),
                Direction::East=> (value, 0),
                Direction::West=> (-value, 0),

            }
        }

        fn turn(&self, value: i32) -> Direction {
            let turns = value / 90;    
            let val = (*self as i32 + turns).rem_euclid(4);
            num::FromPrimitive::from_i32(val).unwrap()
        }
    }

    fn part_one(sample: bool, _param: usize) -> i32 {
        let lines = get_lines(sample);
        let commands = lines.iter().map(Command::from_line).collect_vec();
        let final_state = commands.iter().fold((Direction::East, (0, 0)), |state, command| {
            println!("New state: {:?}", state);
            let (direction, (x, y)) = state;
            match command {
                Command::Forward(v) => { let (dx, dy) = direction.offset(*v); (direction, (x + dx, y + dy)) }
                Command::Turn(v) =>  (direction.turn(*v), (x, y)),
                Command::Cardinal(d, v) => { let (dx, dy) = d.offset(*v); (direction, (x + dx, y + dy)) }
            }
        });

        println!("Final state: {:?}", final_state);
        let (_dir, (x, y)) = final_state;
        x.abs() + y.abs()
    }

    fn part_two(sample: bool, _param: usize) -> i32 {
        let _lines = get_lines(sample);
        let lines = get_lines(sample);
        let commands = lines.iter().map(Command::from_line).collect_vec();
        let final_state = commands.iter().fold(((0,0), (10, 1)), |state, command| {
            println!("Applying command: {command:?} state: {:?}", state);
            let ((ship_x, ship_y), (waypoint_x, waypoint_y)) = state;
            match command {
                Command::Forward(v) => ((ship_x + v * waypoint_x, ship_y + v * waypoint_y), (waypoint_x, waypoint_y)),
                Command::Turn(v) => {
                    let turns = v / 90;    
                    let mut c = complex::Complex::new(waypoint_x, waypoint_y);
                    for _i in 0..(turns.abs()) {
                        c *= complex::Complex::new(0, -turns.signum());
                    }
                    

                    ((ship_x, ship_y), (c.re, c.im))
                },
                Command::Cardinal(d, v) => { let (dx, dy) = d.offset(*v); ((ship_x, ship_y), (waypoint_x + dx, waypoint_y + dy)) }
            }
        });

        println!("Final state: {:?}", final_state);
        let ((ship_x, ship_y), (_waypoint_x, _waypoint_y)) = final_state;
        ship_x.abs() + ship_y.abs()
    }

    fn get_lines(sample: bool) -> Vec<String> {
        let filename = if sample {
            "samples/12.txt"
        } else {
            "inputs/12.txt"
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
        assert_eq!(result, 25);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(false, REAL_PARAM);
        println!("{:?}", result);
        assert_eq!(result, 2270);
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(true, SAMPLE_PARAM);
        println!("{:?}", result);
        assert_eq!(result, 286);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(false, REAL_PARAM);
        println!("{:?}", result);
        assert_eq!(result, 138669);
    }
}
