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
    grid.regions
        .iter()
        .map(|(island, num_positions)| grid.count_fences(*island) * num_positions.len())
        .sum()
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len();
        let height = value.lines().count();

        let mut squares = Vec::<char>::with_capacity(width * height);
        for line in value.lines() {
            for c in line.chars() {
                squares.push(c);
            }
        }

        let mut grid = Grid {
            width: width as isize,
            height: height as isize,
            squares,
            regions: HashMap::new(),
        };

        let all_positions = grid.iter_positions().collect::<Vec<_>>();
        let mut square_regions = vec![Option::<usize>::None; width * height];
        let mut next_region_id = 0;
        for position in all_positions {
            let index = grid.to_index_unchecked(position);

            if square_regions[index].is_some() {
                continue;
            }

            let our_region = Some(next_region_id);
            next_region_id += 1;

            let mut to_check = HashSet::<Position>::new();
            to_check.insert(position);

            while let Some(next_position) = to_check.iter().next().copied() {
                to_check.remove(&next_position);

                let Some(next_index) = grid.to_index(next_position) else {
                    continue;
                };

                if square_regions[next_index].is_some() {
                    continue;
                }

                if grid.squares[next_index] == grid.squares[index] {
                    square_regions[next_index] = our_region;
                    for neighbour in next_position.iter_surrounding_4() {
                        to_check.insert(neighbour);
                    }
                }
            }
        }

        let all_positions = grid.iter_positions().collect::<Vec<_>>();
        for position in all_positions {
            let index = grid.to_index_unchecked(position);

            let region = square_regions[index]
                .expect("We should have assigned regions to everything by this point");

            grid.regions.entry(region).or_default().insert(position);
        }

        grid
    }
}

#[derive(Clone)]
struct Grid {
    width: isize,
    height: isize,
    squares: Vec<char>,
    regions: HashMap<usize, HashSet<Position>>,
}

impl Grid {
    fn is_on_grid(&self, Position(x, y): Position) -> bool {
        0 <= x && x < self.width && 0 <= y && y < self.height
    }

    fn to_index(&self, position: Position) -> Option<usize> {
        if !self.is_on_grid(position) {
            return None;
        }

        Some(self.to_index_unchecked(position))
    }

    fn to_index_unchecked(&self, Position(x, y): Position) -> usize {
        (y * self.width + x) as usize
    }

    fn iter_positions(&self) -> impl Iterator<Item = Position> {
        (0..self.width).flat_map(|x| (0..self.height).map(move |y| Position(x, y)))
    }

    fn count_fences(&self, region_id: usize) -> usize {
        let Some(region_positions) = self.regions.get(&region_id) else {
            return 0;
        };

        let mut num_fences = 0;
        for position in region_positions {
            for neighbour in position.iter_surrounding_4() {
                if !region_positions.contains(&neighbour) {
                    num_fences += 1;
                }
            }
        }

        num_fences
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
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

impl Position {
    fn iter_surrounding_4(self) -> impl Iterator<Item = Position> {
        Direction::iter()
            .map(Direction::into_offset)
            .map(move |offset| self + offset)
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
    grid.regions
        .iter()
        .map(|(island, num_positions)| grid.count_fence_edges(*island) * num_positions.len())
        .sum()
}

impl Grid {
    fn count_fence_edges(&self, region_id: usize) -> usize {
        let Some(region_positions) = self.regions.get(&region_id) else {
            return 0;
        };

        let mut num_fence_edges = 0;

        // Horizontal edges

        {
            for y in 0..self.height {
                let mut counting_down = false;
                let mut counting_up = false;
                for x in 0..self.width {
                    let position = Position(x, y);

                    if region_positions.contains(&position) {
                        if !region_positions.contains(&(position + Direction::Down.into_offset())) {
                            counting_down = true;
                        } else {
                            if counting_down {
                                num_fence_edges += 1;
                            }
                            counting_down = false;
                        }

                        if !region_positions.contains(&(position + Direction::Up.into_offset())) {
                            counting_up = true;
                        } else {
                            if counting_up {
                                num_fence_edges += 1;
                            }
                            counting_up = false;
                        }
                    } else {
                        if counting_down {
                            num_fence_edges += 1;
                        }
                        if counting_up {
                            num_fence_edges += 1;
                        }

                        counting_down = false;
                        counting_up = false;
                    }
                }

                if counting_down {
                    num_fence_edges += 1;
                }

                if counting_up {
                    num_fence_edges += 1;
                }
            }
        }

        // Vertical edges

        {
            for x in 0..self.width {
                let mut counting_left = false;
                let mut counting_right = false;
                for y in 0..self.height {
                    let position = Position(x, y);

                    if region_positions.contains(&position) {
                        if !region_positions.contains(&(position + Direction::Left.into_offset())) {
                            counting_left = true;
                        } else {
                            if counting_left {
                                num_fence_edges += 1;
                            }
                            counting_left = false;
                        }

                        if !region_positions.contains(&(position + Direction::Right.into_offset()))
                        {
                            counting_right = true;
                        } else {
                            if counting_right {
                                num_fence_edges += 1;
                            }
                            counting_right = false;
                        }
                    } else {
                        if counting_left {
                            num_fence_edges += 1;
                        }
                        if counting_right {
                            num_fence_edges += 1;
                        }

                        counting_left = false;
                        counting_right = false;
                    }
                }

                if counting_left {
                    num_fence_edges += 1;
                }

                if counting_right {
                    num_fence_edges += 1;
                }
            }
        }

        num_fence_edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [
            (
                r"AAAA
BBCD
BBCC
EEEC",
                140,
            ),
            (
                r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
                772,
            ),
            (
                r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
                1930,
            ),
        ] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [
            (
                r"AAAA
BBCD
BBCC
EEEC",
                80,
            ),
            (
                r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
                436,
            ),
            (
                r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
                236,
            ),
            (
                r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
                368,
            ),
            (
                r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
                1206,
            ),
        ] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
