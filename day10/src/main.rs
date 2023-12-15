use std::collections::{HashMap, HashSet};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");
const INPUT_TEST_PART2: &str = include_str!("../input_test_part2.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 8);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST_PART2);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 10);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> u32 {
    let (start, map) = parse_map(input);

    (map.extract_loop(&start).len() / 2) as u32
}

fn parse_map(input: &str) -> (CellPosition, Map) {
    let lines = input.lines();

    let mut map = HashMap::new();
    let mut start = None;

    let mut max = (0, 0);

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            let position = CellPosition {
                x: x as i32,
                y: y as i32,
            };

            max = (x, y).max(max);

            let maybe_cell = match c {
                '.' => Some(MapCell::Empty),
                '-' => Some(MapCell::Pipe(Pipe(Direction::West, Direction::East))),
                '|' => Some(MapCell::Pipe(Pipe(Direction::North, Direction::South))),
                'L' => Some(MapCell::Pipe(Pipe(Direction::North, Direction::East))),
                'F' => Some(MapCell::Pipe(Pipe(Direction::South, Direction::East))),
                '7' => Some(MapCell::Pipe(Pipe(Direction::South, Direction::West))),
                'J' => Some(MapCell::Pipe(Pipe(Direction::North, Direction::West))),
                'S' => None, // We don't know what the start pipe is; calculate it later
                _ => unreachable!(),
            };

            if maybe_cell.is_some() {
                map.insert(position, maybe_cell.unwrap());
            } else {
                start = Some(position);
            }
        }
    }

    let start = start.unwrap();
    let start_cell_pipe = detect_start_cell_pipe(&map, &start);
    map.insert(start, MapCell::Pipe(start_cell_pipe));

    (
        start,
        Map {
            layout: map,
            max: max.into(),
        },
    )
}

fn detect_start_cell_pipe(map: &HashMap<CellPosition, MapCell>, start: &CellPosition) -> Pipe {
    let mut connections = 0;

    for ((offset_x, offset_y), wanted_connection) in [
        ((0, -1), Direction::South), // looking for a south connection on the pipe to the north of us
        ((1, 0), Direction::West),
        ((0, 1), Direction::North),
        ((-1, 0), Direction::East),
    ] {
        let position = CellPosition {
            x: start.x + offset_x,
            y: start.y + offset_y,
        };

        if let Some(cell) = map.get(&position) {
            if let MapCell::Pipe(pipe) = cell {
                let other_pipe_connections = pipe.as_mask();
                let connected_to_us = other_pipe_connections & wanted_connection.as_mask() != 0;

                if connected_to_us {
                    connections |= wanted_connection.opposite().as_mask();
                }
            }
        }
    }

    Pipe::from_mask(connections)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CellPosition {
    x: i32,
    y: i32,
}

impl From<(usize, usize)> for CellPosition {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

#[derive(Debug)]
struct Map {
    layout: HashMap<CellPosition, MapCell>,
    max: CellPosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MapCell {
    Empty,
    Pipe(Pipe),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pipe(Direction, Direction);

impl Pipe {
    fn from_mask(mask: isize) -> Self {
        match mask {
            0b0101 => Self(Direction::West, Direction::East),
            0b1010 => Self(Direction::North, Direction::South),
            0b1100 => Self(Direction::North, Direction::East),
            0b0110 => Self(Direction::South, Direction::East),
            0b0011 => Self(Direction::South, Direction::West),
            0b1001 => Self(Direction::North, Direction::West),
            _ => unreachable!(),
        }
    }

    fn as_mask(&self) -> isize {
        self.0.as_mask() | self.1.as_mask()
    }

    fn get_next_direction(&self, entered_from: Option<Direction>) -> Direction {
        match entered_from {
            None => self.0,
            Some(entered) => {
                if self.0 == entered {
                    self.1
                } else {
                    self.0
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North = 0b1000,
    East = 0b0100,
    South = 0b0010,
    West = 0b0001,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    fn as_mask(&self) -> isize {
        *self as isize
    }
}

impl Map {
    fn extract_loop(&self, start: &CellPosition) -> Vec<CellPosition> {
        let mut path = Vec::new();

        let mut current_cell = self.layout.get(start).unwrap();
        let mut current_pos = *start;
        let mut entered_from = None;

        loop {
            path.push(current_pos);

            if let MapCell::Pipe(pipe) = current_cell {
                let next_direction = pipe.get_next_direction(entered_from);

                match next_direction {
                    Direction::North => {
                        current_pos.y -= 1;
                    }
                    Direction::East => {
                        current_pos.x += 1;
                    }
                    Direction::South => {
                        current_pos.y += 1;
                    }
                    Direction::West => {
                        current_pos.x -= 1;
                    }
                }

                entered_from = Some(next_direction.opposite());
            }

            current_cell = self.layout.get(&current_pos).unwrap();

            if current_pos == *start {
                break;
            }
        }

        path
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.max.y {
            for x in 0..=self.max.x {
                let position = CellPosition { x, y };

                if let Some(cell) = self.layout.get(&position) {
                    match cell {
                        MapCell::Empty => write!(f, ".")?,
                        MapCell::Pipe(Pipe(Direction::West, Direction::East)) => write!(f, "-")?,
                        MapCell::Pipe(Pipe(Direction::North, Direction::South)) => write!(f, "|")?,
                        MapCell::Pipe(Pipe(Direction::North, Direction::East)) => write!(f, "╰")?,
                        MapCell::Pipe(Pipe(Direction::South, Direction::East)) => write!(f, "╭")?,
                        MapCell::Pipe(Pipe(Direction::South, Direction::West)) => write!(f, "╮")?,
                        MapCell::Pipe(Pipe(Direction::North, Direction::West)) => write!(f, "╯")?,
                        _ => unreachable!(),
                    }
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn solve_part2(input: &str) -> u32 {
    let (start, mut map) = parse_map(input);
    let main_loop = map
        .extract_loop(&start)
        .iter()
        .copied()
        .collect::<HashSet<_>>();

    // Mark all but the main loop as Empty
    for (position, cell) in map.layout.iter_mut() {
        if main_loop.contains(position) {
            continue;
        }

        *cell = MapCell::Empty;
    }

    let mut internal_cells = 0;

    for y in 0..map.max.y {
        let mut counting = false;
        let mut hoping_for = None;

        for x in 0..map.max.x {
            let position = CellPosition { x, y };

            if let Some(cell) = map.layout.get(&position) {
                match cell {
                    MapCell::Pipe(Pipe(Direction::North, Direction::South)) => {
                        counting = !counting;
                    }
                    MapCell::Pipe(Pipe(this_vertical, _))
                        if matches!(this_vertical, Direction::North | Direction::South) =>
                    {
                        match hoping_for {
                            None => {
                                hoping_for = Some(this_vertical.opposite());
                            }
                            Some(hoping_for_vertical) => {
                                if hoping_for_vertical == *this_vertical {
                                    counting = !counting;
                                }

                                hoping_for = None;
                            }
                        }
                    }
                    MapCell::Empty => {
                        if counting {
                            internal_cells += 1;
                        }
                    }
                    MapCell::Pipe(_) => {}
                }
            }
        }
    }

    internal_cells
}
