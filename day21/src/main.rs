use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    ops::Add,
};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(6, INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 16);

    let part_1_result = solve_part1(64, INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(26_501_365, INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(target_steps: usize, input: &str) -> usize {
    let grid = FiniteGrid::from(input);
    let start = find_start(input);

    grid.count_possible_ending_tiles(target_steps, &[&start])
}

fn find_start(input: &str) -> Position {
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if c == 'S' {
                return Position {
                    x: x as i64,
                    y: y as i64,
                };
            }
        }
    }

    unreachable!()
}
trait Grid {
    fn get_tile(&self, position: &Position) -> Tile;
    fn is_within_bounds(&self, position: &Position) -> bool;

    fn count_possible_ending_tiles(&self, target_steps: usize, start: &[&Position]) -> usize {
        let mut visited = HashMap::<Position, usize>::new();

        let mut queue = BinaryHeap::from_iter(start.iter().map(|start| Reverse((0, **start))));

        while let Some(Reverse((steps, position))) = queue.pop() {
            let next_steps = steps + 1;
            if next_steps > target_steps {
                continue;
            }

            for direction in &[
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let offset = direction.as_offset();
                let new_position = position + offset;

                if self.is_within_bounds(&new_position)
                    && self.get_tile(&new_position) != Tile::Rock
                {
                    if let Some(existing_steps) = visited.get_mut(&new_position) {
                        // If we've been here before, but had made more steps, we can update the number of steps:
                        // This route was more efficient and we can continue to traverse from it
                        if *existing_steps > next_steps {
                            *existing_steps = next_steps;
                            queue.push(Reverse((next_steps, new_position)));
                        }
                    } else {
                        // We've never been here before: unconditionally add to the traversial queue
                        visited.insert(new_position, next_steps);
                        queue.push(Reverse((next_steps, new_position)));
                    }
                }
            }
        }

        visited
            .iter()
            .filter(|(_, steps)| *steps % 2 == target_steps % 2)
            .count() as usize
    }
}

struct FiniteGrid {
    grid: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
}

impl FiniteGrid {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get_tile(&self, position: &Position) -> Tile {
        self.grid[position.y as usize][position.x as usize]
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        position.x >= 0
            && position.x < self.width as i64
            && position.y >= 0
            && position.y < self.height as i64
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    GardenPlot,
    Rock,
}

impl From<&str> for FiniteGrid {
    fn from(value: &str) -> Self {
        let grid: Vec<Vec<_>> = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Tile::Rock,
                        '.' | 'S' => Tile::GardenPlot,
                        _ => unreachable!(),
                    })
                    .collect()
            })
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Offset {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_offset(&self) -> Offset {
        match self {
            Self::Up => Offset { x: 0, y: -1 },
            Self::Down => Offset { x: 0, y: 1 },
            Self::Left => Offset { x: -1, y: 0 },
            Self::Right => Offset { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Position {
    x: i64,
    y: i64,
}

impl Add<Offset> for Position {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn solve_part2(target_steps: usize, input: &str) -> usize {
    // So it turns out that despite noticing a bunch of stuff with the grid,
    // it's actually possible to extrapolate the answer by calculating the
    // first few groups' worth of values with the slower algorithm,
    // then it's go-go-day09!

    let grid = FiniteGrid::from(input);
    let infinite_grid = InfiniteGrid::from(&grid);
    let start = find_start(input);

    // Key to this working is that the grid is square, the start is centered,
    // and it has a clear run to the edge in all 4 directions
    assert!(grid.width() == grid.height());
    assert!((target_steps as i64 - start.x) % grid.height() as i64 == 0);
    assert!((target_steps as i64 - start.y) % grid.height() as i64 == 0);

    let target_grid_reach = (target_steps - start.y as usize) / grid.height();

    let mut sequence = Vec::new();

    // Basically, we are just going to calculate enoguh values the slow way
    // to be able to extrapolate the final value
    for grid_reach in 0.. {
        let this_target_steps = grid.height() / 2 + grid_reach * grid.height();

        if let Ok(extrapolated_value) =
            extrapolate_forwards(&sequence, target_grid_reach - grid_reach + 1)
        {
            return extrapolated_value as usize;
        } else {
            sequence.push(
                infinite_grid.count_possible_ending_tiles(this_target_steps, &[&start]) as i64,
            );
        }
    }

    unreachable!()
}

fn extrapolate_forwards(nums: &Vec<i64>, extrapolations: usize) -> Result<i64, ()> {
    if nums.len() < 2 {
        return Err(());
    }

    let mut nums = nums.clone();
    let mut rows = Vec::new();

    loop {
        rows.push(nums);
        nums = get_diffs(&rows.last().unwrap());

        if nums.len() == 1 {
            return Err(());
        }

        if nums.iter().all(|n| *n == 0) {
            rows.push(nums);
            break;
        }
    }

    for _ in 0..extrapolations {
        rows.last_mut().unwrap().push(0);

        for n in (0..rows.len() - 1).rev() {
            let last_row = rows.iter().nth(n + 1).unwrap();
            let diff = *last_row.last().unwrap();

            let next_row = rows.iter_mut().nth(n).unwrap();
            next_row.push(diff + next_row.last().unwrap());
        }
    }

    return Ok(*rows[0].last().unwrap());
}

fn get_diffs(nums: &[i64]) -> Vec<i64> {
    let mut diffs = Vec::with_capacity(nums.len());

    for i in 0..nums.len() - 1 {
        diffs.push(nums[i + 1] - nums[i]);
    }

    diffs
}

struct InfiniteGrid<'a> {
    grid: &'a FiniteGrid,
}

impl InfiniteGrid<'_> {
    fn get_tile(&self, position: &Position) -> Tile {
        self.grid.get_tile(&Position {
            x: position.x.rem_euclid(self.grid.width as i64),
            y: position.y.rem_euclid(self.grid.height as i64),
        })
    }

    fn is_within_bounds(&self, _: &Position) -> bool {
        true
    }
}

impl<'infinite_grid, 'grid: 'infinite_grid> From<&'grid FiniteGrid>
    for InfiniteGrid<'infinite_grid>
{
    fn from(value: &'grid FiniteGrid) -> Self {
        Self { grid: &value }
    }
}

impl Grid for FiniteGrid {
    fn get_tile(&self, position: &Position) -> Tile {
        self.get_tile(position)
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        self.is_within_bounds(position)
    }
}

impl Grid for InfiniteGrid<'_> {
    fn get_tile(&self, position: &Position) -> Tile {
        self.get_tile(position)
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        self.is_within_bounds(position)
    }
}
