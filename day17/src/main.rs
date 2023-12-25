use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    ops::{Add, Mul},
};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 102);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 94);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> u32 {
    solve::<1, 3>(input)
}

fn solve_part2(input: &str) -> u32 {
    solve::<4, 10>(input)
}

fn solve<const MIN: usize, const MAX: usize>(input: &str) -> u32 {
    let grid = Grid::from(input);

    grid.distance_of_shortest_path_between::<MIN, MAX>(
        &Position { x: 0, y: 0 },
        &Position {
            x: grid.width as i32 - 1,
            y: grid.height as i32 - 1,
        },
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Position {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

struct Grid {
    grid: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let grid: Vec<Vec<_>> = value
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let height = grid.len();
        let width = grid[0].len();

        Self {
            grid,
            height,
            width,
        }
    }
}

impl Grid {
    fn get_weight(&self, position: &Position) -> u32 {
        self.grid[position.y as usize][position.x as usize]
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        position.x >= 0
            && position.x < self.width as i32
            && position.y >= 0
            && position.y < self.height as i32
    }

    fn get_nexts<const MIN: usize, const MAX: usize>(
        &self,
        position: &Position,
        incoming: Option<(Direction, usize)>,
    ) -> Vec<(Position, (Direction, usize), u32)> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        // May not turn back on ourselves
        .filter(|direction| incoming.map_or(true, |incoming| **direction != incoming.0.opposite()))
        .filter_map(|direction| {
            incoming.map_or(Some((direction, MIN - 1, MIN)), |incoming| {
                if *direction != incoming.0 {
                    // If we're going in a different direction, return default
                    Some((direction, MIN - 1, MIN))
                } else if incoming.1 + 1 < MAX {
                    // If we're going in the same direction, may not be further than MAX
                    // and also increment by one
                    Some((direction, incoming.1 + 1, 1))
                } else {
                    // If we're going in the same direction and we're already at MAX,
                    // we cannot go any further so discard
                    None
                }
            })
        })
        .filter_map(|(direction, distance, steps)| {
            let offset = direction.as_position_offset();

            // Sum the cost of all the steps
            let mut next_position = *position;
            let mut cost_to_add = 0;
            for n in 0..steps {
                next_position = *position + offset * (n + 1) as i32;

                // Ensure we're not going out of bounds
                if !self.is_within_bounds(&next_position) {
                    return None;
                }

                cost_to_add += self.get_weight(&next_position);
            }
            Some((next_position, (*direction, distance), cost_to_add))
        })
        .collect()
    }

    fn distance_of_shortest_path_between<const MIN: usize, const MAX: usize>(
        &self,
        start: &Position,
        goal: &Position,
    ) -> u32 {
        type Task = (Position, Option<(Direction, usize)>);

        // Reverse because we want heap to act as a min-heap
        let mut heap = BinaryHeap::<Reverse<(u32, Task)>>::new();
        let mut seen = HashSet::<Task>::new();

        seen.insert((*start, None));

        heap.push(Reverse((0, (*start, None))));

        while let Some(Reverse((cost, (position, incoming)))) = heap.pop() {
            // If we are at the goal, we're done by virtue of min-heap
            // and djiikstra's algorithm
            if position == *goal {
                return cost;
            }

            for (next_position, next_incoming, cost_to_add) in
                self.get_nexts::<MIN, MAX>(&position, incoming)
            {
                // Only traverse if we've not already been here in the same way previously
                if seen.insert((next_position, Some(next_incoming))) {
                    heap.push(Reverse((
                        cost + cost_to_add,
                        (next_position, Some(next_incoming)),
                    )));
                }
            }
        }

        unreachable!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn as_position_offset(&self) -> Position {
        match self {
            Self::Up => Position { x: 0, y: -1 },
            Self::Down => Position { x: 0, y: 1 },
            Self::Left => Position { x: -1, y: 0 },
            Self::Right => Position { x: 1, y: 0 },
        }
    }
}
