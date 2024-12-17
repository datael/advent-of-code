use core::fmt;
use std::{
    fmt::Write,
    ops::{Add, Mul},
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1::<101, 103>(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1<const X: usize, const Y: usize>(input: &str) -> usize {
    let grid_size = Offset {
        x: X as isize,
        y: Y as isize,
    };

    let midpoint_x = (X / 2) as isize;
    let midpoint_y = (Y / 2) as isize;

    input
        .lines()
        .map(Robot::from)
        .map(|robot| {
            robot
                .start_position
                .move_by(&robot.velocity, 100, &grid_size)
        })
        .filter(|position| position.x != midpoint_x && position.y != midpoint_y)
        .map(|position| {
            // slot into one of 4 quadrant IDs
            let x_part = position.x < midpoint_x;
            let y_part = position.y < midpoint_y;

            (x_part as u8 + 2 * y_part as u8) as usize
        })
        .fold(vec![0, 0, 0, 0], |mut vec, quadrant| {
            vec[quadrant] += 1;
            vec
        })
        .iter()
        .product()
}

struct Robot {
    start_position: Offset,
    velocity: Offset,
}

impl<S> From<S> for Robot
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let (start_position, velocity) = value.as_ref().split_once(' ').unwrap();

        let (_, start_position) = start_position.split_once('=').unwrap();
        let (_, velocity) = velocity.split_once('=').unwrap();

        Self {
            start_position: start_position.into(),
            velocity: velocity.into(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Offset {
    x: isize,
    y: isize,
}

impl Add<Offset> for Offset {
    type Output = Offset;

    fn add(self, rhs: Offset) -> Self::Output {
        Offset {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<usize> for Offset {
    type Output = Offset;

    fn mul(self, rhs: usize) -> Self::Output {
        Offset {
            x: self.x * rhs as isize,
            y: self.y * rhs as isize,
        }
    }
}

impl<S> From<S> for Offset
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let (x, y) = value.as_ref().split_once(',').unwrap();

        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl Offset {
    fn move_by(&self, velocity: &Offset, time: usize, grid_size: &Offset) -> Offset {
        let end_position = *self + *velocity * time;

        Self {
            x: end_position.x.rem_euclid(grid_size.x),
            y: end_position.y.rem_euclid(grid_size.y),
        }
    }
}

fn solve_part2(input: &str) -> usize {
    const X: isize = 101;
    const Y: isize = 103;

    let grid_size = Offset { x: X, y: Y };

    let robots = input.lines().map(Robot::from).collect::<Vec<_>>();
    let mut grid = Grid::new(&grid_size);

    for time in 0..100_000 {
        // Me:       How does one identify a tree?
        //           ...maybe a run of 10 robots in a horizontal line?
        // Narrator: That is exactly how one identifies a tree.
        fn is_possible_match(grid: &Grid) -> bool {
            let target = [true; 10];
            for row in grid.filled.iter() {
                if row.windows(target.len()).any(|window| window == target) {
                    return true;
                }
            }

            false
        }

        grid.clear();
        grid.fill(robots.iter(), time);
        if is_possible_match(&grid) {
            return time;
        }
    }

    0
}

struct Grid {
    filled: Vec<Vec<bool>>,
    size: Offset,
}

impl Grid {
    fn new(size: &Offset) -> Self {
        let mut grid = Self {
            filled: Vec::with_capacity(size.y as usize),
            size: *size,
        };

        for _ in 0..size.y {
            grid.filled.push(vec![false; size.x as usize]);
        }

        grid
    }

    fn clear(&mut self) {
        for row in self.filled.iter_mut() {
            for col in row.iter_mut() {
                *col = false;
            }
        }
    }

    fn fill<'r>(&mut self, robots: impl Iterator<Item = &'r Robot>, time: usize) {
        robots
            .map(|robot| {
                robot
                    .start_position
                    .move_by(&robot.velocity, time, &self.size)
            })
            .for_each(|offset| self.filled[offset.y as usize][offset.x as usize] = true)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.filled.iter() {
            for col in row.iter() {
                f.write_char(if *col { '#' } else { '.' })?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
            12,
        )] {
            assert_eq!(solve_part1::<11, 7>(input), expected);
        }
    }
}
