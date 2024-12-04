use std::ops::{Add, Mul};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let grid = Grid::from(input);

    grid.iter_locations()
        .map(|location| grid.count_results_at_location::<Part1Strategy>(&location))
        .sum()
}

struct Grid {
    width: usize,
    height: usize,
    chars: Vec<char>,
}

impl<T> From<T> for Grid
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let height = value.as_ref().lines().count();
        let width = value.as_ref().lines().next().unwrap().len();

        let mut chars = Vec::<char>::with_capacity(height * width);
        for line in value.as_ref().lines() {
            chars.extend(line.chars());
        }

        assert_eq!(width * height, chars.len());

        Self {
            width,
            height,
            chars,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Location {
    x: isize,
    y: isize,
}

impl From<(usize, usize)> for Location {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DLocation {
    dx: isize,
    dy: isize,
}

impl From<(usize, usize)> for DLocation {
    fn from((dx, dy): (usize, usize)) -> Self {
        Self {
            dx: dx as isize,
            dy: dy as isize,
        }
    }
}

impl From<(i32, i32)> for DLocation {
    fn from((dx, dy): (i32, i32)) -> Self {
        Self {
            dx: dx as isize,
            dy: dy as isize,
        }
    }
}

impl Mul<(i32, i32)> for DLocation {
    type Output = DLocation;

    fn mul(self, (xm, ym): (i32, i32)) -> Self::Output {
        DLocation {
            dx: self.dx * xm as isize,
            dy: self.dy * ym as isize,
        }
    }
}

impl Add<DLocation> for Location {
    type Output = Location;

    fn add(self, rhs: DLocation) -> Self::Output {
        Location {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl Grid {
    fn char_at(&self, location: &Location) -> char {
        assert!(self.is_valid_location(location));

        self.chars[(location.y as usize) * self.width + (location.x as usize)]
    }

    fn iter_locations(&self) -> impl Iterator<Item = Location> {
        (0isize..self.width as isize)
            .flat_map(|x| (0isize..self.height as isize).map(move |y| (x, y)))
            .map(|(x, y)| Location { x, y })
    }

    fn get_chars<const LEN: usize>(
        &self,
        origin: &Location,
        offsets: [(i32, i32); LEN],
    ) -> Option<[char; LEN]> {
        let locations = offsets.map(DLocation::from).map(|dl| *origin + dl);

        if locations
            .iter()
            .all(|location| self.is_valid_location(location))
        {
            Some(locations.map(|location| self.char_at(&location)))
        } else {
            None
        }
    }

    fn is_valid_location(&self, location: &Location) -> bool {
        0 <= location.x
            && (location.x as usize) < self.width
            && 0 <= location.y
            && (location.y as usize) < self.height
    }

    fn count_results_at_location<S>(&self, location: &Location) -> usize
    where
        S: Strategy,
    {
        S::count_results_at_location(self, location)
    }
}

trait Strategy {
    fn count_results_at_location(grid: &Grid, location: &Location) -> usize;
}

struct Part1Strategy;

impl Strategy for Part1Strategy {
    fn count_results_at_location(grid: &Grid, location: &Location) -> usize {
        let mut num_results = 0;

        for xm in [-1, 0, 1] {
            for ym in [-1, 0, 1] {
                if let Some(chars) =
                    grid.get_chars(location, [0, 1, 2, 3].map(|n| (n * xm, n * ym)))
                {
                    if chars == ['X', 'M', 'A', 'S'] {
                        num_results += 1;
                    }
                }
            }
        }

        num_results
    }
}

fn solve_part2(input: &str) -> usize {
    let grid = Grid::from(input);

    grid.iter_locations()
        .map(|location| grid.count_results_at_location::<Part2Strategy>(&location))
        .sum()
}

struct Part2Strategy;

impl Strategy for Part2Strategy {
    fn count_results_at_location(grid: &Grid, location: &Location) -> usize {
        if grid.char_at(location) != 'A' {
            return 0;
        }

        let Some(bltr) = grid.get_chars(location, [(-1, -1), (1, 1)]) else {
            return 0;
        };
        let Some(tlbr) = grid.get_chars(location, [(-1, 1), (1, -1)]) else {
            return 0;
        };

        let bltr_ok = bltr == ['M', 'S'] || bltr == ['S', 'M'];
        let tlbr_ok = tlbr == ['M', 'S'] || tlbr == ['S', 'M'];

        if bltr_ok && tlbr_ok {
            return 1;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [
            ("XMAS", 1),
            (
                r"..X...
.SAMX.
.A..A.
XMAS.S
.X....",
                4,
            ),
            (
                r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
                18,
            ),
        ] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [
            (
                r"M.S
.A.
M.S",
                1,
            ),
            (
                r".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
                9,
            ),
        ] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
