use std::collections::{HashMap, HashSet};
use std::ops::Add;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 94);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 154);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let grid = FiniteGrid::parse(input, |c| match c {
        '#' => Tile::Forest,
        '.' => Tile::Path,
        '^' => Tile::Slope(Direction::Up),
        'v' => Tile::Slope(Direction::Down),
        '<' => Tile::Slope(Direction::Left),
        '>' => Tile::Slope(Direction::Right),
        _ => unreachable!(),
    });

    let start = &find_start(&grid);
    let end = &find_end(&grid);

    grid.find_length_of_longest_path_between(start, end)
}

trait Grid {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn get_tile(&self, position: &Position) -> Tile;
    fn is_within_bounds(&self, position: &Position) -> bool;
}

struct FiniteGrid {
    grid: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
}

impl FiniteGrid {
    fn get_tile(&self, position: &Position) -> Tile {
        self.grid[position.y as usize][position.x as usize]
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        position.x >= 0
            && position.x < self.width as i64
            && position.y >= 0
            && position.y < self.height as i64
    }

    fn parse(value: &str, parser: impl Fn(char) -> Tile) -> Self {
        let grid: Vec<Vec<_>> = value
            .lines()
            .map(|line| line.chars().map(&parser).collect())
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
enum Tile {
    Path,
    Forest,
    Slope(Direction),
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Default)]
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

impl Grid for FiniteGrid {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get_tile(&self, position: &Position) -> Tile {
        self.get_tile(position)
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        self.is_within_bounds(position)
    }
}

fn get_next_allowed(
    grid: &impl Grid,
    current_position: &Position,
    predicate: impl Fn(&Position) -> bool,
) -> Vec<Position> {
    if let Tile::Slope(slope_direction) = grid.get_tile(current_position) {
        let next_position = *current_position + slope_direction.as_offset();
        if predicate(&next_position) {
            vec![*current_position + slope_direction.as_offset()]
        } else {
            vec![]
        }
    } else {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .map(|direction| *current_position + direction.as_offset())
        .filter(|next_position| {
            grid.is_within_bounds(next_position)
                && grid.get_tile(next_position) != Tile::Forest
                && predicate(next_position)
        })
        .collect::<Vec<_>>()
    }
}

fn find_start(grid: &impl Grid) -> Position {
    for x in 0..grid.width() as i64 {
        let position = Position { x, y: 0 };
        if grid.get_tile(&position) == Tile::Path {
            return position;
        }
    }

    unreachable!()
}

fn find_end(grid: &impl Grid) -> Position {
    let y = grid.height() as i64 - 1;

    for x in 0..grid.width() as i64 {
        let position = Position { x, y };
        if grid.get_tile(&position) == Tile::Path {
            return position;
        }
    }

    unreachable!()
}

trait Pathable: Grid + Sized {
    fn find_length_of_longest_path_between(&self, start: &Position, end: &Position) -> usize {
        let mut found = Vec::new();

        let mut next_jobs = vec![(0, *start, HashSet::from([*start]))];

        while let Some((distance, position, mut visited)) = next_jobs.pop() {
            let mut nexts = get_next_allowed(self, &position, |next| !visited.contains(next));
            let mut next_distance = distance + 1;

            // avoid cloning visited when we only have one option
            while nexts.len() == 1 {
                let next = &nexts[0];

                if next == end {
                    found.push(next_distance);
                    break;
                }

                visited.insert(*next);

                nexts = get_next_allowed(self, next, |next| !visited.contains(next));
                next_distance += 1;
            }

            for next in nexts.iter() {
                if next == end {
                    found.push(next_distance);
                    continue;
                }

                let mut visited = visited.clone();
                visited.insert(*next);

                next_jobs.push((next_distance, *next, visited));
            }
        }

        *found.iter().max().unwrap()
    }

    fn build_junctions(&self, start: &Position, end: &Position) -> HashMap<Position, Junction> {
        let mut junctions =
            HashMap::from([(*start, Junction::default()), (*end, Junction::default())]);

        for x in 0..self.width() as i64 {
            for y in 0..self.height() as i64 {
                let position = Position { x, y };

                if self.get_tile(&position) == Tile::Path
                    && get_next_allowed(self, &position, |_| true).len() > 2
                {
                    junctions.insert(position, Default::default());
                }
            }
        }

        let mut junction_jobs = junctions.keys().cloned().collect::<Vec<_>>();

        for junction in junction_jobs.iter_mut() {
            for next in get_next_allowed(self, junction, |_| true) {
                let mut distance = 0;

                let mut prev = *junction;
                let mut next = next;

                loop {
                    let nexts = get_next_allowed(self, &next, |it| it != &prev);
                    distance += 1;

                    if junctions.contains_key(&next) {
                        junctions
                            .get_mut(&next)
                            .unwrap()
                            .nexts
                            .insert(*junction, distance);

                        junctions
                            .get_mut(junction)
                            .unwrap()
                            .nexts
                            .insert(next, distance);

                        break;
                    } else if nexts.is_empty() {
                        break;
                    } else {
                        prev = next;
                        next = nexts[0];
                    }
                }
            }
        }

        junctions
    }

    fn find_length_of_longest_path_between_by_junctions(
        &self,
        start: &Position,
        end: &Position,
    ) -> usize {
        let junctions = self.build_junctions(start, end);

        let mut max_found = 0;
        let mut jobs = vec![(0, *start, HashSet::from([*start]))];

        while let Some((distance, position, visited)) = jobs.pop() {
            let nexts = &junctions[&position].nexts;

            for (next_position, next_distance) in
                nexts.iter().filter(|(next, _)| !visited.contains(next))
            {
                let distance = next_distance + distance;

                if next_position == end {
                    max_found = max_found.max(distance);
                } else {
                    let mut visited = visited.clone();
                    visited.insert(*next_position);
                    jobs.push((distance, *next_position, visited));
                }
            }
        }

        max_found
    }
}

impl<T: Grid> Pathable for T {}

fn solve_part2(input: &str) -> usize {
    let grid = FiniteGrid::parse(input, |c| match c {
        '#' => Tile::Forest,
        '.' | '^' | 'v' | '<' | '>' => Tile::Path,
        _ => unreachable!(),
    });

    let start = &find_start(&grid);
    let end = &find_end(&grid);

    grid.find_length_of_longest_path_between_by_junctions(start, end)
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct Junction {
    nexts: HashMap<Position, usize>,
}
