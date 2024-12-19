use std::{
    collections::HashSet,
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
    let (grid, moves) = input.split_once("\n\n").unwrap();

    let mut grid = Grid::from(grid);
    let moves = moves
        .lines()
        .flat_map(|line| line.chars())
        .map(Direction::from);

    for direction in moves {
        grid.move_by(direction);
    }

    grid.iter_positions()
        .map(|position| (position, grid.get_tile_at_unchecked(position)))
        .filter(|(_, tile)| *tile == Tile::Box)
        .map(|(offset, _)| offset.x + 100 * offset.y)
        .sum::<isize>() as usize
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Wall),
            'O' => Ok(Tile::Box),
            '[' => Ok(Tile::BoxLeft),
            ']' => Ok(Tile::BoxRight),
            _ => Err(()),
        }
    }
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Box => 'O',
            Tile::BoxLeft => '[',
            Tile::BoxRight => ']',
        }
    }
}

impl Tile {
    fn is_box(&self) -> bool {
        matches!(self, Tile::Box | Tile::BoxLeft | Tile::BoxRight)
    }
}

#[derive(PartialEq, Eq)]
struct Grid {
    width: isize,
    height: isize,
    tiles: Vec<Tile>,
    robot_offset: Offset,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grid")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("robot_offset", &self.robot_offset)
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
            robot_offset: Offset { x: 0, y: 0 },
        };

        for (y, line) in value.as_ref().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Ok(tile_type) = Tile::try_from(c) {
                    grid.tiles.push(tile_type);
                } else {
                    grid.tiles.push(Tile::Empty);

                    assert!(c == '@');
                    grid.robot_offset = Offset {
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
    fn get_tile_at_unchecked(&self, offset: Offset) -> Tile {
        self.tiles[self.to_index_unchecked(offset)]
    }

    fn set_tile_at_unchecked(&mut self, offset: Offset, tile: Tile) {
        let index = self.to_index_unchecked(offset);
        self.tiles[index] = tile;
    }

    fn to_index_unchecked(&self, Offset { x, y }: Offset) -> usize {
        (y * self.width + x) as usize
    }

    fn iter_positions(&self) -> impl Iterator<Item = Offset> {
        (0..self.width).flat_map(|x| (0..self.height).map(move |y| Offset { x, y }))
    }

    fn move_by(&mut self, direction: Direction) {
        let next = self.robot_offset + direction.into_offset();
        let next_tile = self.get_tile_at_unchecked(next);

        // Short-cut the simple cases
        if next_tile == Tile::Wall {
            return;
        }
        if next_tile == Tile::Empty {
            self.robot_offset = next;
            return;
        }

        // Then potentially move any boxes
        let mut subsequent = next + direction.into_offset();
        while self.get_tile_at_unchecked(subsequent).is_box() {
            subsequent += direction.into_offset();
        }

        // Cannot move past a wall
        if self.get_tile_at_unchecked(subsequent) == Tile::Wall {
            return;
        }

        if self.get_tile_at_unchecked(subsequent) == Tile::Empty {
            self.set_tile_at_unchecked(subsequent, Tile::Box);
            self.set_tile_at_unchecked(next, Tile::Empty);
            self.robot_offset = next;
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
    fn into_offset(self) -> Offset {
        match self {
            Direction::Up => Offset { x: 0, y: -1 },
            Direction::Right => Offset { x: 1, y: 0 },
            Direction::Down => Offset { x: 0, y: 1 },
            Direction::Left => Offset { x: -1, y: 0 },
        }
    }

    fn is_vertical(self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }

    fn is_horizontal(self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
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
    let (grid, moves) = input.split_once("\n\n").unwrap();

    let mut grid = Grid::from(grid).expanded();
    let moves = moves
        .lines()
        .flat_map(|line| line.chars())
        .map(Direction::from);

    for direction in moves {
        grid.move_by_part2(direction);
    }

    grid.iter_positions()
        .map(|position| (position, grid.get_tile_at_unchecked(position)))
        .filter(|(_, tile)| *tile == Tile::BoxLeft)
        .map(|(offset, _)| offset.x + 100 * offset.y)
        .sum::<isize>() as usize
}

impl Grid {
    fn expanded(&self) -> Grid {
        let width = self.width * 2;
        let height = self.height;
        let robot_offset = self.robot_offset * Offset { x: 2, y: 1 };

        let mut tiles = Vec::with_capacity(width as usize * height as usize);

        for tile in self.tiles.iter() {
            match tile {
                Tile::BoxLeft => panic!("Already expanded"),
                Tile::BoxRight => panic!("Already expanded"),
                Tile::Box => {
                    tiles.push(Tile::BoxLeft);
                    tiles.push(Tile::BoxRight);
                }
                tile => {
                    tiles.push(*tile);
                    tiles.push(*tile);
                }
            }
        }

        Grid {
            width,
            height,
            tiles,
            robot_offset,
        }
    }

    fn move_by_part2(&mut self, direction: Direction) {
        let next = self.robot_offset + direction.into_offset();
        let next_tile = self.get_tile_at_unchecked(next);

        assert!(next_tile != Tile::Box); // Box should have been expanded already

        // Short-cut the simple cases
        if next_tile == Tile::Wall {
            return;
        }

        if next_tile == Tile::Empty {
            self.robot_offset = next;
            return;
        }

        // Then potentially move any boxes

        // Horizontal is the simplest case
        if direction.is_horizontal() {
            let mut subsequent = next + direction.into_offset();
            while self.get_tile_at_unchecked(subsequent).is_box() {
                subsequent += direction.into_offset();
            }

            // Cannot move past a wall
            if self.get_tile_at_unchecked(subsequent) == Tile::Wall {
                return;
            }

            if self.get_tile_at_unchecked(subsequent) == Tile::Empty {
                let offset = direction.into_offset();

                while subsequent != next {
                    self.swap_tiles(subsequent, subsequent - offset);
                    subsequent -= offset;
                }

                self.set_tile_at_unchecked(next, Tile::Empty);
                self.robot_offset = next;
            }

            return;
        }

        // And now vertical, where we need to branch outwards
        assert!(direction.is_vertical());

        let mut affected = HashSet::new();
        let can_push = self.recursively_check_push(next, direction, &mut affected);

        if !can_push {
            return;
        }

        // We're going to swap boxes starting from the furthest,
        // so sort into the correct Y order depending on direection
        let mut affected_sorted = affected.iter().collect::<Vec<_>>();
        affected_sorted.sort_by(|a, b| {
            if direction == Direction::Up {
                a.y.cmp(&b.y)
            } else {
                b.y.cmp(&a.y)
            }
        });

        for offset in affected_sorted {
            self.swap_tiles(*offset, *offset + direction.into_offset());
        }

        self.robot_offset = next;
    }

    fn recursively_check_push(
        &self,
        next: Offset,
        direction: Direction,
        affected: &mut HashSet<Offset>,
    ) -> bool {
        let next_tile = self.get_tile_at_unchecked(next);

        if next_tile == Tile::Wall {
            return false;
        } else if next_tile == Tile::Empty {
            return true;
        }

        let partner_offset = if next_tile == Tile::BoxLeft {
            Direction::Right
        } else {
            Direction::Left
        }
        .into_offset();

        affected.insert(next);
        affected.insert(next + partner_offset);

        let direct_next = next + direction.into_offset();
        let partner_next = next + direction.into_offset() + partner_offset;

        self.recursively_check_push(direct_next, direction, affected)
            && self.recursively_check_push(partner_next, direction, affected)
    }

    fn swap_tiles(&mut self, a: Offset, b: Offset) {
        let tile_a = self.get_tile_at_unchecked(a);
        let tile_b = self.get_tile_at_unchecked(b);

        self.set_tile_at_unchecked(a, tile_b);
        self.set_tile_at_unchecked(b, tile_a);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_against_wall() {
        let mut grid = Grid::from(
            r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );
        grid.move_by(Direction::Left);
        let expected = Grid::from(
            r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_movement_empty_target() {
        let mut grid = Grid::from(
            r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );
        grid.move_by(Direction::Up);
        let expected = Grid::from(
            r"########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_movement_push_one() {
        let mut grid = Grid::from(
            r"########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );
        grid.move_by(Direction::Right);
        let expected = Grid::from(
            r"########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_movement_push_two() {
        let mut grid = Grid::from(
            r"########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );
        grid.move_by(Direction::Right);
        let expected = Grid::from(
            r"########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_movement_push_against_freestanding_wall() {
        let mut grid = Grid::from(
            r"########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########",
        );
        grid.move_by(Direction::Left);
        let expected = Grid::from(
            r"########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########",
        );

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_part1() {
        for (input, expected) in [
            (
                r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
                2028,
            ),
            (
                r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
                10092,
            ),
        ] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_grid_expanded() {
        for (input, expected) in [(
            r"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######",
            r"##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############",
        )] {
            let expanded = Grid::from(input).expanded();
            assert_eq!(expanded, Grid::from(expected));
        }
    }

    #[test]
    fn test_movement_part2_push_one_vertical() {
        let mut grid = Grid::from(
            r"##############
##......##..##
##...[][]...##
##...@[]....##
##..........##
##..........##
##############",
        );
        grid.move_by_part2(Direction::Up);
        let expected = Grid::from(
            r"##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############",
        );

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_movement_part2_push_two_horizontal() {
        let mut grid = Grid::from(
            r"##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############",
        );
        grid.move_by_part2(Direction::Left);
        let expected = Grid::from(
            r"##############
##......##..##
##..........##
##...[][]@..##
##....[]....##
##..........##
##############",
        );

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_movement_part2_push_offset() {
        let mut grid = Grid::from(
            r"##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.....@....##
##############",
        );
        grid.move_by_part2(Direction::Up);
        let expected = Grid::from(
            r"##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############",
        );

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_movement_part2_push_offset_blocked() {
        let mut grid = Grid::from(
            r"##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############",
        );
        grid.move_by_part2(Direction::Up);
        let expected = Grid::from(
            r"##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############",
        );

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [(
            r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
            9021,
        )] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
