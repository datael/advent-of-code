use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::{self, Display, Formatter},
};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 136);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 64);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let mut platform = parse_platform(input);

    platform.tilt_north();

    platform.calculate_weight()
}

fn parse_platform(input: &str) -> Platform {
    Platform {
        cells: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => CellContents::Empty,
                        'O' => CellContents::RoundedRock,
                        '#' => CellContents::CubeShapedRock,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Platform {
    cells: Vec<Vec<CellContents>>,
}

impl Platform {
    fn tilt_north(&mut self) {
        for x in 0..self.cells[0].len() {
            for y in 0..self.cells.len() {
                for yy in 0..self.cells.len() - y - 1 {
                    if self.cells[yy][x] == CellContents::Empty
                        && self.cells[yy + 1][x] == CellContents::RoundedRock
                    {
                        self.cells[yy][x] = self.cells[yy + 1][x];
                        self.cells[yy + 1][x] = CellContents::Empty;
                    }
                }
            }
        }
    }

    fn calculate_weight(&self) -> usize {
        let mut weight = 0;

        for y in 0..self.cells.len() {
            let mul = self.cells.len() - y;

            for x in 0..self.cells[0].len() {
                if self.cells[y][x] == CellContents::RoundedRock {
                    weight += mul;
                }
            }
        }

        weight
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                match cell {
                    CellContents::Empty => write!(f, ".")?,
                    CellContents::RoundedRock => write!(f, "O")?,
                    CellContents::CubeShapedRock => write!(f, "#")?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CellContents {
    Empty,
    RoundedRock,
    CubeShapedRock,
}

enum Direction {
    North,
    West,
    South,
    East,
}

fn solve_part2(input: &str) -> usize {
    let mut platform = parse_platform(input);

    let mut memo = HashMap::new();

    // Detect loops
    let mut cycles = 0_usize;
    let cycles_returned_to = loop {
        match memo.entry(platform.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(cycles);
            }
            Entry::Occupied(entry) => {
                break *entry.get();
            }
        };

        platform.cycle();
        cycles += 1;
    };

    // Skip calculating the loops
    let loop_size = cycles - cycles_returned_to;
    let target = 1_000_000_000;
    let rem = (target - cycles_returned_to) % loop_size;

    // Then finally cycle the remainder
    for _ in 0..rem {
        platform.cycle();
    }

    platform.calculate_weight()
}

impl Platform {
    fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.tilt_north(),
            Direction::West => self.tilt_west(),
            Direction::South => self.tilt_south(),
            Direction::East => self.tilt_east(),
        }
    }

    fn tilt_south(&mut self) {
        for x in 0..self.cells[0].len() {
            for y in (0..self.cells.len()).rev() {
                for yy in (1..self.cells.len() - y).rev() {
                    if self.cells[yy][x] == CellContents::Empty
                        && self.cells[yy - 1][x] == CellContents::RoundedRock
                    {
                        self.cells[yy][x] = self.cells[yy - 1][x];
                        self.cells[yy - 1][x] = CellContents::Empty;
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.cells.len() {
            for x in 0..self.cells[0].len() {
                for xx in 0..self.cells[0].len() - x - 1 {
                    if self.cells[y][xx] == CellContents::Empty
                        && self.cells[y][xx + 1] == CellContents::RoundedRock
                    {
                        self.cells[y][xx] = self.cells[y][xx + 1];
                        self.cells[y][xx + 1] = CellContents::Empty;
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.cells.len() {
            for x in (0..self.cells[0].len()).rev() {
                for xx in (1..self.cells[0].len() - x).rev() {
                    if self.cells[y][xx] == CellContents::Empty
                        && self.cells[y][xx - 1] == CellContents::RoundedRock
                    {
                        self.cells[y][xx] = self.cells[y][xx - 1];
                        self.cells[y][xx - 1] = CellContents::Empty;
                    }
                }
            }
        }
    }
}
