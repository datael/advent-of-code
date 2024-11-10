use std::mem;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let mut grid = Grid::from(input);
    let mut layout_scratch = vec![];
    let mut neighbours_scratch = vec![];

    for _ in 0..100 {
        grid.advance::<Part1Strategy>(&mut layout_scratch, &mut neighbours_scratch);
    }

    grid.num_on()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    width: usize,
    height: usize,
    layout: Vec<bool>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut width = None;
        let mut height = 0;
        let mut layout = vec![];

        for line in value.lines() {
            let line_width = line.len();

            match width {
                None => width = Some(line_width),
                Some(known_width) => {
                    if line_width != known_width {
                        continue;
                    }
                }
            }

            height += 1;

            layout.reserve(line.len());

            for c in line.chars() {
                match c {
                    '.' => layout.push(false),
                    '#' => layout.push(true),
                    _ => {}
                }
            }
        }

        Self {
            width: width.unwrap(),
            height,
            layout,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Location {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Location {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl Grid {
    fn populate_with_neighbours(&self, location: Location, neighbours: &mut Vec<Location>) {
        neighbours.clear();

        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                let x = location.x + dx;
                let y = location.y + dy;

                if (0 <= x && x < self.width as i32)
                    && (0 <= y && y < self.height as i32)
                    && (dx != 0 || dy != 0)
                {
                    neighbours.push((x, y).into())
                }
            }
        }
    }

    fn advance<S>(&mut self, layout_scratch: &mut Vec<bool>, neighbours_scratch: &mut Vec<Location>)
    where
        S: Strategy,
    {
        layout_scratch.clear();

        for y in 0..self.height {
            for x in 0..self.width {
                let this_location = (x as i32, y as i32).into();
                let next_state = S::get_next_state(self, this_location, neighbours_scratch);
                layout_scratch.push(next_state);
            }
        }

        mem::swap(&mut self.layout, layout_scratch);
    }

    fn get_index(&self, location: Location) -> usize {
        self.width * location.y as usize + location.x as usize
    }

    fn num_on(&self) -> usize {
        self.layout.iter().map(|active| *active as usize).sum()
    }
}

trait Strategy {
    fn get_current_state(grid: &Grid, location: Location) -> bool;

    fn get_next_state(
        grid: &Grid,
        location: Location,
        neighbours_scratch: &mut Vec<Location>,
    ) -> bool;
}

struct Part1Strategy;

impl Strategy for Part1Strategy {
    fn get_current_state(grid: &Grid, location: Location) -> bool {
        grid.layout[grid.get_index(location)]
    }

    fn get_next_state(
        grid: &Grid,
        location: Location,
        neighbours_scratch: &mut Vec<Location>,
    ) -> bool {
        grid.populate_with_neighbours(location, neighbours_scratch);

        let num_neighbours_active = neighbours_scratch
            .iter()
            .map(|location| Self::get_current_state(grid, *location) as u8)
            .sum::<u8>();

        let current_state = Self::get_current_state(grid, location);

        match num_neighbours_active {
            2 if current_state => true,
            3 => true,
            _ => false,
        }
    }
}

fn solve_part2(input: &str) -> usize {
    let mut grid = Grid::from(input);
    let mut layout_scratch = vec![];
    let mut neighbours_scratch = vec![];

    for _ in 0..100 {
        grid.advance::<Part2Strategy>(&mut layout_scratch, &mut neighbours_scratch);
    }

    grid.num_on()
}

struct Part2Strategy;

impl Part2Strategy {
    fn is_stuck_on(grid: &Grid, location: Location) -> bool {
        for x in [0, grid.width - 1] {
            for y in [0, grid.height - 1] {
                if location == (x as i32, y as i32).into() {
                    return true;
                }
            }
        }

        false
    }
}

impl Strategy for Part2Strategy {
    fn get_current_state(grid: &Grid, location: Location) -> bool {
        // If stuck on, always true
        if Self::is_stuck_on(grid, location) {
            return true;
        }

        // Otherwise, the same as part 1
        grid.layout[grid.get_index(location)]
    }

    fn get_next_state(
        grid: &Grid,
        location: Location,
        neighbours_scratch: &mut Vec<Location>,
    ) -> bool {
        // If stuck on, always true
        if Self::is_stuck_on(grid, location) {
            return true;
        }

        // Otherwise, the same as part 1
        grid.populate_with_neighbours(location, neighbours_scratch);

        let num_neighbours_active = neighbours_scratch
            .iter()
            .map(|location| Self::get_current_state(grid, *location) as u8)
            .sum::<u8>();

        let current_state = Self::get_current_state(grid, location);

        match num_neighbours_active {
            2 if current_state => true,
            3 => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_parser() {
        for (input, expected) in [
            (
                "#.\n.#\n",
                Grid {
                    width: 2,
                    height: 2,
                    layout: vec![true, false, false, true],
                },
            ),
            (
                "..\n##\n.#\n#.\n..\n.#\n",
                Grid {
                    width: 2,
                    height: 6,
                    layout: vec![
                        false, false, true, true, false, true, true, false, false, false, false,
                        true,
                    ],
                },
            ),
            (
                "#.#\n.#.\n\n\n",
                Grid {
                    width: 3,
                    height: 2,
                    layout: vec![true, false, true, false, true, false],
                },
            ),
        ] {
            assert_eq!(Grid::from(input), expected);
        }
    }

    #[test]
    fn test_neighbours() {
        for (input, expected) in [
            (
                (
                    Grid {
                        width: 3,
                        height: 3,
                        layout: [true; 9].into(),
                    },
                    (1, 1).into(),
                ),
                vec![
                    (0, 0).into(),
                    (0, 1).into(),
                    (0, 2).into(),
                    (1, 0).into(),
                    (1, 2).into(),
                    (2, 0).into(),
                    (2, 1).into(),
                    (2, 2).into(),
                ],
            ),
            (
                (
                    Grid {
                        width: 3,
                        height: 3,
                        layout: [true; 9].into(),
                    },
                    (0, 0).into(),
                ),
                vec![(0, 1).into(), (1, 0).into(), (1, 1).into()],
            ),
            (
                (
                    Grid {
                        width: 3,
                        height: 3,
                        layout: [true; 9].into(),
                    },
                    (2, 2).into(),
                ),
                vec![(1, 1).into(), (1, 2).into(), (2, 1).into()],
            ),
        ] {
            let mut neighbours = vec![];
            input.0.populate_with_neighbours(input.1, &mut neighbours);

            assert_eq!(neighbours, expected);
        }
    }

    #[test]
    fn test_advance_part1() {
        for (input, expected) in [
            (
                ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..",
                "..##..\n..##.#\n...##.\n......\n#.....\n#.##..",
            ),
            (
                "..##..\n..##.#\n...##.\n......\n#.....\n#.##..",
                "..###.\n......\n..###.\n......\n.#....\n.#....",
            ),
            (
                "..###.\n......\n..###.\n......\n.#....\n.#....",
                "...#..\n......\n...#..\n..##..\n......\n......",
            ),
            (
                "...#..\n......\n...#..\n..##..\n......\n......",
                "......\n......\n..##..\n..##..\n......\n......",
            ),
        ] {
            let mut input = Grid::from(input);
            input.advance::<Part1Strategy>(&mut vec![], &mut vec![]);

            assert_eq!(input, Grid::from(expected));
        }
    }

    #[test]
    fn test_advance_part2() {
        for (input, expected) in [
            (
                "##.#.#\n...##.\n#....#\n..#...\n#.#..#\n####.#",
                "#.##.#\n####.#\n...##.\n......\n#...#.\n#.####",
            ),
            (
                "#.##.#\n####.#\n...##.\n......\n#...#.\n#.####",
                "#..#.#\n#....#\n.#.##.\n...##.\n.#..##\n##.###",
            ),
            (
                "#..#.#\n#....#\n.#.##.\n...##.\n.#..##\n##.###",
                "#...##\n####.#\n..##.#\n......\n##....\n####.#",
            ),
            (
                "#...##\n####.#\n..##.#\n......\n##....\n####.#",
                "#.####\n#....#\n...#..\n.##...\n#.....\n#.#..#",
            ),
            (
                "#.####\n#....#\n...#..\n.##...\n#.....\n#.#..#",
                "##.###\n.##..#\n.##...\n.##...\n#.#...\n##...#",
            ),
        ] {
            let mut input = Grid::from(input);
            input.advance::<Part2Strategy>(&mut vec![], &mut vec![]);

            assert_eq!(input, Grid::from(expected));
        }
    }
}
