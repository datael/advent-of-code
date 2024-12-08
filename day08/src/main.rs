use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let grid = Grid::from(input);

    grid.get_antinodes()
        .iter()
        .flat_map(|(_, positions)| positions)
        .collect::<HashSet<_>>()
        .len()
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len() as isize;
        let height = value.lines().count() as isize;

        let mut antennas = HashMap::<char, HashSet<Position>>::new();
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.char_indices() {
                match c {
                    '.' => {}
                    _ => {
                        antennas
                            .entry(c)
                            .or_default()
                            .insert(Position(x as isize, y as isize));
                    }
                }
            }
        }

        Grid {
            width,
            height,
            antennas,
        }
    }
}

#[derive(Clone)]
struct Grid {
    width: isize,
    height: isize,
    antennas: HashMap<char, HashSet<Position>>,
}

impl Grid {
    fn is_on_grid(&self, Position(x, y): Position) -> bool {
        0 <= x && x < self.width && 0 <= y && y < self.height
    }

    fn get_antinodes(&self) -> HashMap<char, HashSet<Position>> {
        let mut res = HashMap::<char, HashSet<Position>>::new();
        for (frequency, positions) in self.antennas.iter() {
            let pairs = positions
                .iter()
                .flat_map(|a| positions.iter().map(move |b| (a, b)))
                .filter(|(a, b)| a != b);

            for (a, b) in pairs {
                let next_pos = *a + (*a - *b);

                if self.is_on_grid(next_pos) {
                    res.entry(*frequency).or_default().insert(next_pos);
                }
            }
        }

        res
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position(isize, isize);

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn solve_part2(input: &str) -> usize {
    let grid = Grid::from(input);

    grid.get_antinodes_with_resonant_harmonics()
        .iter()
        .flat_map(|(_, positions)| positions)
        .collect::<HashSet<_>>()
        .len()
}

impl Grid {
    fn get_antinodes_with_resonant_harmonics(&self) -> HashMap<char, HashSet<Position>> {
        let mut res = HashMap::<char, HashSet<Position>>::new();
        for (frequency, positions) in self.antennas.iter() {
            let pairs = positions
                .iter()
                .flat_map(|a| positions.iter().map(move |b| (a, b)))
                .filter(|(a, b)| a != b);

            for (a, b) in pairs {
                let frequency_antinodes = res.entry(*frequency).or_default();

                let dpos = *a - *b;
                let mut next_pos = *a;
                loop {
                    if self.is_on_grid(next_pos) {
                        frequency_antinodes.insert(next_pos);
                    } else {
                        break;
                    }
                    next_pos = next_pos + dpos;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
            14,
        )] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [
            (
                r"T....#....
...T......
.T........
..........
..........
..........
..........
..........
..........
..........",
                9,
            ),
            (
                r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
                34,
            ),
        ] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
