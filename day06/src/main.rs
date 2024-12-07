use std::{collections::HashSet, ops::Add};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let (grid, guard_position) = parse_grid(input);

    let visited = get_visited_from(&grid, &guard_position, &Direction::Up);
    visited.len()
}

fn get_visited_from(
    grid: &Grid,
    initial_position: &Position,
    initial_direction: &Direction,
) -> HashSet<Position> {
    let mut visited = HashSet::<Position>::new();
    visited.insert(*initial_position);

    let mut current_position = *initial_position;
    let mut current_direction = *initial_direction;

    while grid.is_on_grid(current_position) {
        let mut next_position = current_position + current_direction.get_offset();
        while grid.unpassable.contains(&next_position) {
            current_direction = current_direction.get_next();
            next_position = current_position + current_direction.get_offset();
        }

        if !grid.is_on_grid(next_position) {
            break;
        }

        visited.insert(next_position);

        current_position = next_position;
    }

    visited
}

fn parse_grid(input: &str) -> (Grid, Position) {
    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;

    let mut guard_position = Position(0_isize, 0_isize);
    let mut unpassable = HashSet::<Position>::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            match c {
                '#' => {
                    unpassable.insert(Position(x as isize, y as isize));
                }
                '^' => {
                    guard_position = Position(x as isize, y as isize);
                }
                _ => {}
            }
        }
    }

    (
        Grid {
            width,
            height,
            unpassable,
        },
        guard_position,
    )
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position(isize, isize);

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Clone)]
struct Grid {
    width: isize,
    height: isize,
    unpassable: HashSet<Position>,
}

impl Grid {
    fn is_on_grid(&self, Position(x, y): Position) -> bool {
        0 <= x && x < self.width && 0 <= y && y < self.height
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
    fn get_next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_offset(&self) -> Position {
        match self {
            Direction::Up => Position(0, -1),
            Direction::Right => Position(1, 0),
            Direction::Down => Position(0, 1),
            Direction::Left => Position(-1, 0),
        }
    }
}

fn solve_part2(input: &str) -> usize {
    let (grid, guard_position) = parse_grid(input);

    // Simplest optimization:
    //   We don't have to try adding obstacles in every grid square.
    //   Since the guard -- without any new obstructions -- travels along a path that we can determine ahead of time,
    //   we only need to add obstacles in the spaces the guard actually visits.

    let positions_to_block = get_visited_from(&grid, &guard_position, &Direction::Up);

    let guard_direction = Direction::Up;

    let mut visited = HashSet::<(Position, Direction)>::new();
    let mut grid = grid;

    let mut num_loops = 0;

    for position_to_block in positions_to_block.iter() {
        visited.insert((guard_position, guard_direction));

        let must_remove = grid.unpassable.insert(*position_to_block);

        let mut guard_position = guard_position;
        let mut guard_direction = Direction::Up;

        while grid.is_on_grid(guard_position) {
            let mut next_position = guard_position + guard_direction.get_offset();
            while grid.unpassable.contains(&next_position) {
                guard_direction = guard_direction.get_next();
                next_position = guard_position + guard_direction.get_offset();
            }

            if !grid.is_on_grid(next_position) {
                break;
            }

            if visited.contains(&(next_position, guard_direction)) {
                num_loops += 1;
                break;
            }

            visited.insert((next_position, guard_direction));

            guard_position = next_position;
        }

        if must_remove {
            grid.unpassable.remove(position_to_block);
        }

        visited.clear();
    }

    num_loops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
            41,
        )] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [(
            r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
            6,
        )] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
