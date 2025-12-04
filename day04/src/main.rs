use std::time::Instant;

use advent_of_code_2025_lib::Grid;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {part_1_result}");

    let before = Instant::now();
    let part_2_result = solve_part2(INPUT);
    let after = Instant::now();

    dbg!(after - before);
    println!("Part 2: {part_2_result}");
}

fn solve_part1(input: &str) -> usize {
    let grid = Grid::<Tile>::from(input);

    grid.iter_offsets()
        .filter(|offset| grid.get_tile_at_unchecked(*offset) == &Tile::RollOfPaper)
        .map(|offset| {
            grid.iter_surrounding(offset)
                .filter(|surrounding_offset| {
                    grid.get_tile_at_unchecked(*surrounding_offset) == &Tile::RollOfPaper
                })
                .count()
        })
        .filter(|num_surrounding_rolls| num_surrounding_rolls < &4)
        .count()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    RollOfPaper,
    Accessible,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::RollOfPaper,
            '.' => Self::Empty,
            'x' => Self::Accessible,
            _ => panic!(),
        }
    }
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        match value {
            Tile::RollOfPaper => '@',
            Tile::Empty => '.',
            Tile::Accessible => 'x',
        }
    }
}

fn solve_part2(input: &str) -> usize {
    let mut grid = Grid::<Tile>::from(input);

    loop {
        let removable_offsets = grid
            .iter_offsets()
            .filter(|offset| grid.get_tile_at_unchecked(*offset) == &Tile::RollOfPaper)
            .map(|offset| {
                (
                    offset,
                    grid.iter_surrounding(offset)
                        .filter(|surrounding_offset| {
                            grid.get_tile_at_unchecked(*surrounding_offset) == &Tile::RollOfPaper
                        })
                        .count(),
                )
            })
            .filter_map(|(offset, num_surrounding_rolls)| {
                (num_surrounding_rolls < 4).then_some(offset)
            })
            .collect::<Vec<_>>();

        if removable_offsets.is_empty() {
            break;
        }

        for offset in removable_offsets.iter() {
            *grid.get_tile_at_mut_unchecked(*offset) = Tile::Accessible;
        }
    }

    grid.iter_offsets()
        .filter(|offset| grid.get_tile_at_unchecked(*offset) == &Tile::Accessible)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

        assert_eq!(solve_part1(input), 13);
    }

    #[test]
    fn test_input_part2() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

        assert_eq!(solve_part2(input), 43);
    }
}
