use std::{
    iter,
    ops::{Add, AddAssign, Mul},
};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 62);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 952_408_144_115);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> u64 {
    solve::<Part1Strategy>(input)
}

fn solve<S: ParseCommandsStrategy>(input: &str) -> u64 {
    let commands = input.lines().map(S::parse_command).collect::<Vec<_>>();

    let mut path_commands = commands.iter();
    let mut position = Position { x: 0, y: 0 };

    let path = iter::once(position)
        .chain(iter::from_fn(|| {
            if let Some(command) = path_commands.next() {
                let offset = command.direction.as_offset() * command.count as i64;
                position += offset;
                Some(position)
            } else {
                None
            }
        }))
        .collect::<Vec<_>>();

    let windows = path.iter().zip(path.iter().skip(1)).collect::<Vec<_>>();

    // Shoelace formula
    let shoelace = windows
        .iter()
        .map(|(a, b)| a.x * b.y - a.y * b.x)
        .sum::<i64>() as u64
        / 2;

    // Pick's theorem
    let pick = 1 + commands.iter().map(|command| command.count).sum::<u64>() / 2;

    shoelace + pick
}

trait ParseCommandsStrategy {
    fn parse_command(input: &str) -> Command;
}

struct Part1Strategy;

impl ParseCommandsStrategy for Part1Strategy {
    fn parse_command(input: &str) -> Command {
        let (movement, _) = input.split_once(" (").unwrap();
        let (direction, count) = movement.split_once(' ').unwrap();

        Command {
            direction: direction.chars().nth(0).unwrap().into(),
            count: count.parse::<u64>().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Command {
    direction: Direction,
    count: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Offset {
    x: i64,
    y: i64,
}

impl Mul<i64> for Offset {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
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

impl AddAssign<Offset> for Position {
    fn add_assign(&mut self, rhs: Offset) {
        *self = *self + rhs;
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
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

fn solve_part2(input: &str) -> u64 {
    solve::<Part2Strategy>(input)
}

struct Part2Strategy;

impl ParseCommandsStrategy for Part2Strategy {
    fn parse_command(input: &str) -> Command {
        let (_, command) = input.split_once(" (#").unwrap();
        let command = command.trim_end_matches(')');

        let count = u64::from_str_radix(&command[0..5], 16).unwrap();
        let direction = match command.chars().nth(5).unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => unreachable!(),
        };

        Command { direction, count }
    }
}
