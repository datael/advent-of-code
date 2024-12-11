use std::{
    collections::{HashSet, VecDeque},
    ops::Add,
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

    grid.iter_positions()
        .filter(|position| grid.is_trailhead(*position))
        .map(|trailhead| grid.calculate_trailhead_score(trailhead))
        .sum()
}

#[derive(Clone)]
struct Grid {
    width: isize,
    height: isize,
    square_heights: Vec<u8>,
}

impl Grid {
    fn is_on_grid(&self, Position(x, y): Position) -> bool {
        0 <= x && x < self.width && 0 <= y && y < self.height
    }

    fn get_height(&self, position: Position) -> Option<u8> {
        self.to_index(position)
            .map(|valid_position| self.square_heights[valid_position])
    }

    fn to_index(&self, Position(x, y): Position) -> Option<usize> {
        if !self.is_on_grid(Position(x, y)) {
            return None;
        }

        Some((y * self.width + x) as usize)
    }

    fn iter_positions(&self) -> impl Iterator<Item = Position> {
        (0..self.width).flat_map(|x| (0..self.height).map(move |y| Position(x, y)))
    }

    fn is_trailhead(&self, start: Position) -> bool {
        self.get_height(start) == Some(0)
    }

    fn calculate_trailhead_score(&self, start: Position) -> usize {
        let mut ends = HashSet::<Position>::new();
        let mut nexts = VecDeque::<(Position, u8)>::new();
        nexts.push_back((start, 0));

        while let Some((curr, curr_height)) = nexts.pop_front() {
            for offset in Direction::iter() {
                let test_position = curr + offset.into_offset();

                if let Some(height) = self.get_height(test_position) {
                    if height == curr_height + 1 {
                        if height == 9 {
                            ends.insert(test_position);
                        } else {
                            nexts.push_back((test_position, height));
                        }
                    }
                }
            }
        }

        ends.len()
    }
}

impl<S> From<S> for Grid
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let width = value.as_ref().lines().next().unwrap().len();
        let height = value.as_ref().lines().count();

        let mut grid = Grid {
            width: width as isize,
            height: height as isize,
            square_heights: vec![0u8; width * height],
        };

        for (y, line) in value.as_ref().lines().enumerate() {
            for (x, c) in line.char_indices() {
                match c {
                    '0'..='9' => {
                        let index = grid
                            .to_index(Position(x as isize, y as isize))
                            .expect("If this is off the grid something is terribly wrong");
                        grid.square_heights[index] = c as u8 - b'0';
                    }
                    _ => unreachable!("Unexpected input"),
                }
            }
        }

        grid
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position(isize, isize);

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .iter()
        .copied()
    }

    fn into_offset(self) -> Position {
        match self {
            Direction::Up => Position(0, -1),
            Direction::Right => Position(1, 0),
            Direction::Down => Position(0, 1),
            Direction::Left => Position(-1, 0),
        }
    }
}

fn solve_part2(input: &str) -> usize {
    let grid = Grid::from(input);

    grid.iter_positions()
        .filter(|position| grid.is_trailhead(*position))
        .map(|trailhead| grid.calculate_trailhead_rating(trailhead))
        .sum()
}

impl Grid {
    fn calculate_trailhead_rating(&self, start: Position) -> usize {
        let mut ends = Vec::<Position>::new();
        let mut nexts = VecDeque::<(Position, u8)>::new();
        nexts.push_back((start, 0));

        while let Some((curr, curr_height)) = nexts.pop_front() {
            for offset in Direction::iter() {
                let test_position = curr + offset.into_offset();

                if let Some(height) = self.get_height(test_position) {
                    if height == curr_height + 1 {
                        if height == 9 {
                            ends.push(test_position);
                        } else {
                            nexts.push_back((test_position, height));
                        }
                    }
                }
            }
        }

        ends.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [
            (
                r"0123
1234
8765
9876",
                1,
            ),
            (
                r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
                36,
            ),
        ] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [(
            r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
            81,
        )] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
