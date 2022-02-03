use itertools::Itertools;
use std::{env, fs};

#[derive(Eq, PartialEq, Debug, Clone)]
struct Grid<T> {
    state: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn new(state: Vec<T>, width: usize, height: usize) -> Grid<T> {
        Grid {
            state,
            width,
            height,
        }
    }

    fn from_2d(initial: Vec<Vec<T>>) -> Grid<T> {
        let height = initial.len();
        let width = initial[0].len();
        Grid {
            state: initial.into_iter().flatten().collect_vec(),
            width,
            height,
        }
    }

    fn pos_to_index(&self, pos: (usize, usize)) -> usize {
        let (x, y) = pos;
        y * self.width + x
    }

    fn at(&self, pos: (usize, usize)) -> &T {
        &self.state[self.pos_to_index(pos)]
    }

    fn mut_at(&mut self, pos: (usize, usize)) -> &mut T {
        let index = self.pos_to_index(pos);
        &mut self.state[index]
    }

    fn neighbors(&self, pos: (usize, usize)) -> impl Iterator<Item = &T> {
        let (x, y) = pos;
        let delta = -1..=1;
        delta
            .clone()
            .cartesian_product(delta)
            .filter_map(move |(dx, dy)| {
                if dx == 0 && dy == 0 {
                    None
                } else {
                    let new_x = x as i64 + dx;
                    let new_y = y as i64 + dy;
                    if new_x >= 0
                        && new_x < self.width as i64
                        && new_y >= 0
                        && new_y < self.height as i64
                    {
                        Some(self.at((new_x as usize, new_y as usize)))
                    } else {
                        None
                    }
                }
            })
    }

    fn neighbors_along_directions(
        &self,
        pos: (usize, usize),
    ) -> Vec<impl Iterator<Item = (usize, usize)>> {
        let (x, y) = pos;
        let (width, height) = (self.width, self.height);
        let delta = -1..=1;
        let v = delta
            .clone()
            .cartesian_product(delta)
            .filter_map(move |(dx, dy)| {
                if dx == 0 && dy == 0 {
                    None
                } else {
                    let nums = 1..std::cmp::max(width, height);

                    // Have to make an in scope copy to appease borrow checker
                    let width = width;
                    let height = height;

                    Some(
                        nums.filter_map(move |d| {
                            let new_x = x as i64 + dx * d as i64;
                            let new_y = y as i64 + dy * d as i64;

                            if new_x >= 0 && new_y >= 0 {
                                Some((new_x as usize, new_y as usize))
                            } else {
                                None
                            }
                        })
                        .take_while(move |(new_x, new_y)| *new_x < width && *new_y < height),
                    )
                    //.map(|pos| self.at(pos)))
                }
            })
            .collect_vec();
        v
    }

    fn to_2d(&self) -> Vec<Vec<&T>> {
        self.state
            .chunks(self.width)
            .map(|chunk| chunk.iter().collect_vec())
            .collect_vec()
    }

    fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| (x, y))
    }
}

impl<T: std::fmt::Display> Grid<T> {
    fn display(&self) {
        self.state.chunks(self.width).for_each(|chunk| {
            chunk.iter().for_each(|t| print!("{t}"));
            println!();
        });
        println!();
    }
}

fn main() {
    let (filename, param) = if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("samples/11.txt", 0)
    } else {
        ("inputs/11.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.lines().collect_vec();

    part_one(&input_lines, param);
    part_two(&input_lines, param);
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Space {
    Floor,
    EmptyChair,
    OccupiedChair,
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let c = match self {
            Self::Floor => '.',
            Self::EmptyChair => 'L',
            Self::OccupiedChair => '#',
        };
        write!(f, "{c}")
    }
}

fn from_lines(lines: &[&str]) -> Grid<Space> {
    let initial = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Floor,
                    'L' => Space::EmptyChair,
                    '#' => Space::OccupiedChair,
                    _ => panic!(),
                })
                .collect_vec()
        })
        .collect_vec();
    Grid::from_2d(initial)
}

fn part_one(lines: &[&str], _param: usize) {
    let mut grid = from_lines(lines);
    grid.display();

    while {
        let new_grid = grid
            .positions()
            .map(|pos| {
                // println!("Examining position: {:?}", pos);
                let v = grid.at(pos);
                let occupied_neighbors = grid
                    .neighbors(pos)
                    .filter(|neighbor| matches!(neighbor, Space::OccupiedChair));
                match v {
                    Space::Floor => Space::Floor,
                    Space::EmptyChair => {
                        if occupied_neighbors.count() == 0 {
                            Space::OccupiedChair
                        } else {
                            Space::EmptyChair
                        }
                    }
                    Space::OccupiedChair => {
                        if occupied_neighbors.count() >= 4 {
                            Space::EmptyChair
                        } else {
                            Space::OccupiedChair
                        }
                    }
                }
            })
            .collect_vec();

        let old_grid = grid;
        grid = Grid::new(new_grid, old_grid.width, old_grid.height);
        grid.display();
        // panic!();
        old_grid != grid
    } {}

    println!(
        "Part 1: {}",
        grid.positions()
            .filter_map(|pos| {
                let v = grid.at(pos);
                if matches!(v, Space::OccupiedChair) {
                    Some(v)
                } else {
                    None
                }
            })
            .count()
    );
}

fn part_two(lines: &[&str], _param: usize) {
    let mut grid = from_lines(lines);
    grid.display();

    let mut count = 1;
    while {
        let new_grid = grid
            .positions()
            .map(|pos| {
                // println!("Examining position: {:?}", pos);
                let v = grid.at(pos);
                let along = grid.neighbors_along_directions(pos);
                let occupied_neighbors = along
                    .into_iter()
                    .map(|dir| {
                        // println!("Examining direction");
                        dir.filter_map(|pos| {
                            // println!("Examining neighbor {pos:?}");
                            let v = grid.at(pos);
                            if matches!(v, Space::EmptyChair | Space::OccupiedChair) {
                                Some(v)
                            } else {
                                None
                            }
                        })
                        .next()
                    })
                    .filter_map(|first| {
                        if let Some(f) = first {
                            if matches!(f, Space::OccupiedChair) {
                                return Some(f);
                            }
                        }
                        None
                    });
                match v {
                    Space::Floor => Space::Floor,
                    Space::EmptyChair => {
                        if occupied_neighbors.count() == 0 {
                            Space::OccupiedChair
                        } else {
                            Space::EmptyChair
                        }
                    }
                    Space::OccupiedChair => {
                        if occupied_neighbors.count() >= 5 {
                            Space::EmptyChair
                        } else {
                            Space::OccupiedChair
                        }
                    }
                }
            })
            .collect_vec();

        let old_grid = grid;
        grid = Grid::new(new_grid, old_grid.width, old_grid.height);
        println!("Application {count}");
        count += 1;
        grid.display();
        // count < 5
        old_grid != grid
    } {}

    println!(
        "Part 2: {}",
        grid.positions()
            .filter_map(|pos| {
                let v = grid.at(pos);
                if matches!(v, Space::OccupiedChair) {
                    Some(v)
                } else {
                    None
                }
            })
            .count()
    );
}

// L.LL.LL.LL
// LLLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLLL
// L.LLLLLL.L
// L.LLLLL.LL

// #.##.##.##
// #######.##
// #.#.#..#..
// ####.##.##
// #.##.##.##
// #.#####.##
// ..#.#.....
// ##########
// #.######.#
// #.#####.##

// #.LL.L#.##
// #LLLLLL.L#
// L.L.L..L..
// #LLL.LL.L#
// #.LL.LL.LL
// #.LLLL#.##
// ..L.L.....
// #LLLLLLLL#
// #.LLLLLL.L
// #.#LLLL.##
