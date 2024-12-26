use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    fmt::{self, Write},
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1::<70, 70, 1024>(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2::<70, 70>(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1<const MAX_X_COORD: usize, const MAX_Y_COORD: usize, const MAX_INPUTS: usize>(
    input: &str,
) -> usize {
    let mut grid = Grid::new(MAX_X_COORD + 1, MAX_Y_COORD + 1);

    for offset in input.lines().map(Offset::from).take(MAX_INPUTS) {
        grid.set_tile_at(offset, Tile::Filled)
            .expect("Input outside of expected range");
    }

    let from = Offset { x: 0, y: 0 };
    let to = Offset {
        x: MAX_X_COORD as isize,
        y: MAX_Y_COORD as isize,
    };

    grid.find_optimal_path(from, to)
        .expect("No path found; invalid input")
        .path_history
        .len()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Filled,
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Filled => '#',
        }
    }
}

impl Tile {
    fn is_traversible(&self) -> bool {
        matches!(self, Tile::Empty)
    }
}

#[derive(PartialEq, Eq)]
struct Grid {
    width: isize,
    height: isize,
    tiles: Vec<Tile>,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grid")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()?;

        for (i, square) in self.tiles.iter().enumerate() {
            if i as isize % self.width == 0 {
                f.write_char('\n')?;
            }
            f.write_char(square.into())?;
        }

        Ok(())
    }
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width: width as isize,
            height: height as isize,
            tiles: vec![Tile::Empty; width * height],
        }
    }

    fn is_on_grid(&self, Offset { x, y }: Offset) -> bool {
        0 <= x && x < self.width && 0 <= y && y < self.height
    }

    fn get_tile_at(&self, offset: Offset) -> Option<Tile> {
        let index = self.to_index(offset)?;
        Some(self.tiles[index])
    }

    fn set_tile_at(&mut self, offset: Offset, tile: Tile) -> Result<(), ()> {
        let index = self.to_index(offset).ok_or(())?;
        self.tiles[index] = tile;
        Ok(())
    }

    fn to_index(&self, offset: Offset) -> Option<usize> {
        if self.is_on_grid(offset) {
            Some(self.to_index_unchecked(offset))
        } else {
            None
        }
    }

    fn to_index_unchecked(&self, Offset { x, y }: Offset) -> usize {
        (y * self.width + x) as usize
    }

    fn find_optimal_path(&self, from: Offset, to: Offset) -> Option<Path> {
        let initial = Path {
            cost: 0,
            offset: from,
            path_history: Vec::new(),
        };

        let mut visited = HashSet::<Offset>::new();
        visited.insert(initial.offset);

        let mut nexts = BinaryHeap::<Reverse<Path>>::new();
        nexts.push(Reverse(initial));

        while let Some(Reverse(path)) = nexts.pop() {
            for direction in Direction::iter() {
                let next_offset = path.offset + direction.into_offset();

                let Some(tile) = self.get_tile_at(next_offset) else {
                    continue;
                };

                if !tile.is_traversible() {
                    continue;
                }

                // By necessity of the minheap, if we've visited this place then
                // we've already visited it in a more optimal way
                if visited.contains(&next_offset) {
                    continue;
                }

                visited.insert(next_offset);

                let next_cost = path.cost + 1;

                let mut path_history = path.path_history.clone();
                path_history.push(next_offset);

                let next_path = Path {
                    cost: next_cost,
                    offset: next_offset,
                    path_history,
                };

                // Are we there yet?
                if next_offset == to {
                    return Some(next_path);
                } else {
                    nexts.push(Reverse(next_path));
                }
            }
        }

        None
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Path {
    cost: usize,
    offset: Offset,
    path_history: Vec<Offset>,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Offset {
    x: isize,
    y: isize,
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

impl Add<Offset> for Offset {
    type Output = Offset;

    fn add(self, rhs: Offset) -> Self::Output {
        Offset {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Offset> for Offset {
    fn add_assign(&mut self, rhs: Offset) {
        *self = *self + rhs;
    }
}

impl Sub<Offset> for Offset {
    type Output = Offset;

    fn sub(self, rhs: Offset) -> Self::Output {
        Offset {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Offset> for Offset {
    fn sub_assign(&mut self, rhs: Offset) {
        *self = *self - rhs;
    }
}

impl Mul<Offset> for Offset {
    type Output = Offset;

    fn mul(self, rhs: Offset) -> Self::Output {
        Offset {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign<Offset> for Offset {
    fn mul_assign(&mut self, rhs: Offset) {
        *self = *self * rhs;
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

    fn into_offset(self) -> Offset {
        match self {
            Direction::Up => Offset { x: 0, y: -1 },
            Direction::Right => Offset { x: 1, y: 0 },
            Direction::Down => Offset { x: 0, y: 1 },
            Direction::Left => Offset { x: -1, y: 0 },
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("Unexpected input"),
        }
    }
}

fn solve_part2<const MAX_X_COORD: usize, const MAX_Y_COORD: usize>(input: &str) -> String {
    let mut grid = Grid::new(MAX_X_COORD + 1, MAX_Y_COORD + 1);

    // We'll go backwards this time; it feels like it's going to get better results.
    let from = Offset {
        x: MAX_X_COORD as isize,
        y: MAX_Y_COORD as isize,
    };
    let to = Offset { x: 0, y: 0 };

    // 1. Find a path
    // 2. Drop bytes until one hits the path (i.e. blocks the path)
    // 3. Repeat from 1 until we cannot find a path
    // 4. The last thing we dropped is the result

    // I wonder what we've got waiting for us that makes this take forever... (turns out there wasn't anything)
    let mut falling_bytes = input.lines().map(Offset::from);

    let mut current_path = grid
        .find_optimal_path(from, to)
        .expect("The grid is empty; there should definitely be a path!");

    let path_blocker_offset = 'done: loop {
        let Some(next_falling_byte) = falling_bytes.next() else {
            panic!("We were expecting something to block the path, but nothing did...");
        };

        grid.set_tile_at(next_falling_byte, Tile::Filled)
            .expect("Input was out of range");

        if current_path.path_history.contains(&next_falling_byte) {
            match grid.find_optimal_path(from, to) {
                None => break 'done next_falling_byte, // No path? We found the blocking byte.
                Some(next_path) => current_path = next_path,
            };
        }
    };

    format!("{},{}", path_blocker_offset.x, path_blocker_offset.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
            22,
        )] {
            assert_eq!(solve_part1::<6, 6, 12>(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [(
            r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
            "6,1",
        )] {
            assert_eq!(solve_part2::<6, 6>(input), expected);
        }
    }
}
