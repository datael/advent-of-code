use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::{self, Write},
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    Grid::from(input).find_optimal_path().cost
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => panic!("Unexpected input"),
        }
    }
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Start => 'S',
            Tile::End => 'E',
        }
    }
}

impl Tile {
    fn is_traversible(&self) -> bool {
        matches!(self, Tile::Empty | Tile::Start | Tile::End)
    }
}

#[derive(PartialEq, Eq)]
struct Grid {
    width: isize,
    height: isize,
    tiles: Vec<Tile>,
    start_offset: Offset,
    end_offset: Offset,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grid")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("start_offset", &self.start_offset)
            .field("end_offset", &self.end_offset)
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

impl<S> From<S> for Grid
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let height = value.as_ref().lines().count();
        let width = value.as_ref().lines().next().unwrap().len();

        let mut grid = Grid {
            width: width as isize,
            height: height as isize,
            tiles: Vec::with_capacity(width * height),
            start_offset: Offset { x: 0, y: 0 },
            end_offset: Offset { x: 0, y: 0 },
        };

        for (y, line) in value.as_ref().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile = Tile::from(c);

                grid.tiles.push(tile);

                if tile == Tile::Start {
                    grid.start_offset = Offset {
                        x: x as isize,
                        y: y as isize,
                    };
                } else if tile == Tile::End {
                    grid.end_offset = Offset {
                        x: x as isize,
                        y: y as isize,
                    };
                }
            }
        }

        grid
    }
}

impl Grid {
    fn is_on_grid(&self, Offset { x, y }: Offset) -> bool {
        0 <= x && x < self.width && 0 <= y && y < self.height
    }

    fn get_tile_at(&self, offset: Offset) -> Option<Tile> {
        let index = self.to_index(offset)?;
        Some(self.tiles[index])
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

    fn find_optimal_path(&self) -> Path {
        let initial = Path {
            cost: 0,
            offset: self.start_offset,
            direction: Direction::Right,
            path_history: Vec::new(),
        };

        let mut visited = HashSet::<(Offset, Direction)>::new();
        visited.insert((initial.offset, initial.direction));

        let mut nexts = BinaryHeap::<Reverse<Path>>::new();
        nexts.push(Reverse(initial));

        while let Some(Reverse(path)) = nexts.pop() {
            for (next_element, next_offset, next_direction) in [
                (
                    PathElement::Advance,
                    path.offset + path.direction.into_offset(),
                    path.direction,
                ),
                (
                    PathElement::Turn(TurnDirection::Clockwise),
                    path.offset,
                    path.direction.rotated_by(TurnDirection::Clockwise),
                ),
                (
                    PathElement::Turn(TurnDirection::Counterclockwise),
                    path.offset,
                    path.direction.rotated_by(TurnDirection::Counterclockwise),
                ),
            ] {
                let Some(tile) = self.get_tile_at(next_offset) else {
                    continue;
                };

                if !tile.is_traversible() {
                    continue;
                }

                // By necessity of the minheap, if we've visited this place then
                // we've already visited it in a more optimal way
                if visited.contains(&(next_offset, next_direction)) {
                    continue;
                }

                visited.insert((next_offset, next_direction));

                let next_cost = path.cost + next_element.get_cost();

                let mut path_history = path.path_history.clone();
                path_history.push((next_element, next_offset, next_direction));

                let next_path = Path {
                    cost: next_cost,
                    offset: next_offset,
                    direction: next_direction,
                    path_history,
                };

                // Are we there yet?
                if next_offset == self.end_offset {
                    return next_path;
                } else {
                    nexts.push(Reverse(next_path));
                }
            }
        }

        panic!("Failed to find any path; input is invalid")
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Path {
    cost: usize,
    offset: Offset,
    direction: Direction,
    path_history: Vec<(PathElement, Offset, Direction)>,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TurnDirection {
    Clockwise,
    Counterclockwise,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PathElement {
    Turn(TurnDirection),
    Advance,
}

impl PathElement {
    fn get_cost(self) -> usize {
        match self {
            PathElement::Turn(_) => 1000,
            PathElement::Advance => 1,
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

    fn rotated_by(self, turn: TurnDirection) -> Direction {
        match turn {
            TurnDirection::Clockwise => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            TurnDirection::Counterclockwise => match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            },
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

fn solve_part2(input: &str) -> usize {
    Grid::from(input)
        .find_all_optimal_paths_offsets()
        .collect::<HashSet<_>>()
        .len()
}

impl Grid {
    fn find_all_optimal_paths_offsets(&self) -> impl Iterator<Item = Offset> {
        let initial = Path {
            cost: 0,
            offset: self.start_offset,
            direction: Direction::Right,
            path_history: Vec::new(),
        };

        let mut visited_bests = HashMap::<(Offset, Direction), usize>::new();
        visited_bests.insert((initial.offset, initial.direction), 0);

        let mut nexts = BinaryHeap::<Reverse<Path>>::new();
        nexts.push(Reverse(initial));

        // Find the optimal path, whilst also populating bests
        let optimal_path = 'found_optimal: loop {
            let Some(Reverse(path)) = nexts.pop() else {
                panic!("No optimal path found; input must be invalid.");
            };

            for (next_element, next_offset, next_direction) in [
                (
                    PathElement::Advance,
                    path.offset + path.direction.into_offset(),
                    path.direction,
                ),
                (
                    PathElement::Turn(TurnDirection::Clockwise),
                    path.offset,
                    path.direction.rotated_by(TurnDirection::Clockwise),
                ),
                (
                    PathElement::Turn(TurnDirection::Counterclockwise),
                    path.offset,
                    path.direction.rotated_by(TurnDirection::Counterclockwise),
                ),
            ] {
                let Some(tile) = self.get_tile_at(next_offset) else {
                    continue;
                };

                if !tile.is_traversible() {
                    continue;
                }

                // By necessity of the minheap, if we've visited this place then
                // we've already visited it in a more optimal, or equally optimal way
                if visited_bests.contains_key(&(next_offset, next_direction)) {
                    continue;
                }

                let next_cost = path.cost + next_element.get_cost();

                visited_bests.insert((next_offset, next_direction), next_cost);

                let mut path_history = path.path_history.clone();
                path_history.push((next_element, next_offset, next_direction));

                let next_path = Path {
                    cost: next_cost,
                    offset: next_offset,
                    direction: next_direction,
                    path_history,
                };

                // Are we there yet?
                if next_offset == self.end_offset {
                    break 'found_optimal next_path;
                } else {
                    nexts.push(Reverse(next_path));
                }
            }
        };

        // Now that we have both the optimal path and all bests, we can walk backwards from the end.
        // We will have explored all possible optimal paths by necessity of the minheap we used previously.
        //
        // Tracking the paths backwards this time, deduct the "next" step from the cost
        // and check if that value equals our known optimal value for that offet+direction.
        // If it does, then we are still on an optimal path and may continue it.

        let mut offsets_on_optimal_paths = HashSet::new();
        offsets_on_optimal_paths.insert(self.end_offset);

        let mut nexts = VecDeque::<(Offset, Direction, usize)>::new();

        for direction in Direction::iter() {
            nexts.push_back((self.end_offset, direction, optimal_path.cost));
        }

        while let Some((next_offset, next_direction, next_cost)) = nexts.pop_front() {
            for (potential_next_offset, potential_next_direction, potential_next_cost) in [
                (
                    next_offset,
                    next_direction.rotated_by(TurnDirection::Clockwise),
                    next_cost
                        .saturating_sub(PathElement::Turn(TurnDirection::Clockwise).get_cost()),
                ),
                (
                    next_offset,
                    next_direction.rotated_by(TurnDirection::Counterclockwise),
                    next_cost.saturating_sub(
                        PathElement::Turn(TurnDirection::Counterclockwise).get_cost(),
                    ),
                ),
                (
                    next_offset - next_direction.into_offset(),
                    next_direction,
                    next_cost.saturating_sub(PathElement::Advance.get_cost()),
                ),
            ] {
                let Some(tile) = self.get_tile_at(potential_next_offset) else {
                    continue;
                };

                if !tile.is_traversible() {
                    continue;
                }

                if let Some(&best_cost) =
                    visited_bests.get(&(potential_next_offset, potential_next_direction))
                {
                    if best_cost == potential_next_cost {
                        // We're still on an optimal path; store offset and continue to traverse
                        offsets_on_optimal_paths.insert(potential_next_offset);
                        nexts.push_front((
                            potential_next_offset,
                            potential_next_direction,
                            potential_next_cost,
                        ));
                    }
                }
            }
        }

        offsets_on_optimal_paths.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [
            (
                r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
                7036,
            ),
            (
                r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
                11048,
            ),
        ] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [
            (
                r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
                45,
            ),
            (
                r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
                64,
            ),
        ] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
