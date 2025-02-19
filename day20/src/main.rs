use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

use advent_of_code_2024_lib::{Direction, Grid as TileGrid, Offset, Traversability};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1::<100>(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2::<100, 20>(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1<const MIN_SAVING: usize>(input: &str) -> usize {
    let grid = Grid::from(input);

    let original_optimal_path = grid
        .find_optimal_path(grid.start_offset, grid.end_offset)
        .expect("no path; this is unexpected");

    let original_optimal_cost = original_optimal_path.cost;

    let costs_from_start = grid.calculate_all_costs_from(grid.start_offset);
    let costs_from_end = grid.calculate_all_costs_from(grid.end_offset);

    let mut accepted = 0;

    for offset_a in grid.iter_offsets() {
        let tile_a = grid.get_tile_at_unchecked(offset_a);

        // Skip anything that's traversable already as we wouldn't change anything by making it traversable.
        // We'll catch the "but tile_b is intraversable" case in a later loop when that tile is tile_a.
        if tile_a.is_traversable() {
            continue;
        }

        let mut optimal_in: Option<(usize, Offset)> = None;
        let mut optimal_out: Option<(usize, Offset)> = None;

        for offset in Direction::iter().map(Direction::into_offset) {
            let offset_b = offset_a + offset;

            if !grid.is_on_grid(offset_b) {
                continue;
            }

            if let Some(&cost_from_start) = costs_from_start.get(&offset_b) {
                if optimal_in.is_none() || optimal_in.unwrap().0 > cost_from_start {
                    optimal_in = Some((cost_from_start, offset_b));
                }
            }

            if let Some(&cost_from_end) = costs_from_end.get(&offset_b) {
                if optimal_out.is_none() || optimal_out.unwrap().0 > cost_from_end {
                    optimal_out = Some((cost_from_end, offset_b));
                }
            }
        }

        let Some((cost_in, offset_in)) = optimal_in else {
            continue;
        };

        let Some((cost_out, offset_out)) = optimal_out else {
            continue;
        };

        if offset_out == offset_in {
            // Nothing changed
            continue;
        }

        let distance = offset_out.manhattan_distance(offset_in);
        let this_cost = cost_in + cost_out + distance;

        if original_optimal_cost - this_cost >= MIN_SAVING {
            accepted += 1;
        }
    }

    accepted
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

impl Traversability for Tile {
    fn is_traversable(&self) -> bool {
        matches!(self, Tile::Empty | Tile::Start | Tile::End)
    }
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

#[derive(PartialEq, Eq, Clone)]
struct Grid {
    inner: TileGrid<Tile>,
    start_offset: Offset,
    end_offset: Offset,
}

impl Deref for Grid {
    type Target = TileGrid<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<S> From<S> for Grid
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let inner = TileGrid::<Tile>::from(value);

        let start_offset = inner
            .iter_offsets()
            .find(|&offset| inner.get_tile_at_unchecked(offset) == &Tile::Start)
            .expect("invalid input");

        let end_offset = inner
            .iter_offsets()
            .find(|&offset| inner.get_tile_at_unchecked(offset) == &Tile::End)
            .expect("invalid input");

        Self {
            inner,
            start_offset,
            end_offset,
        }
    }
}

fn solve_part2<const MIN_SAVING: usize, const CHEAT_DURATION: usize>(input: &str) -> usize {
    let range = CHEAT_DURATION as isize;

    let grid = Grid::from(input);

    let original_optimal_path = grid
        .find_optimal_path(grid.start_offset, grid.end_offset)
        .expect("no path; this is unexpected");

    let original_optimal_cost = original_optimal_path.cost;

    let costs_from_start = grid.calculate_all_costs_from(grid.start_offset);
    let costs_from_end = grid.calculate_all_costs_from(grid.end_offset);

    let mut potential_cheats = HashSet::<Cheat>::new();

    for offset_a in grid.iter_offsets() {
        for x in -range..=range {
            for y in -(range - x.abs())..=(range - x.abs()) {
                let offset_b = offset_a + Offset { x, y };
                if !grid.is_on_grid(offset_b) {
                    continue;
                }

                let cheat = Cheat::from((offset_a, offset_b));
                potential_cheats.insert(cheat);
            }
        }
    }

    potential_cheats
        .iter()
        .filter(|Cheat(from, to)| {
            let distance = from.manhattan_distance(*to);

            for (a, b) in [(from, to), (to, from)] {
                let Some(cost_in) = costs_from_start.get(a) else {
                    continue;
                };
                let Some(cost_out) = costs_from_end.get(b) else {
                    continue;
                };

                let this_cost = cost_in + cost_out + distance;

                if this_cost < original_optimal_cost
                    && original_optimal_cost - this_cost >= MIN_SAVING
                {
                    return true;
                }
            }

            false
        })
        .count()
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Cheat(Offset, Offset);

impl From<(Offset, Offset)> for Cheat {
    fn from((a, b): (Offset, Offset)) -> Self {
        Self(a.min(b), a.max(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
            44,
        )] {
            assert_eq!(solve_part1::<2>(input), expected);
            assert_eq!(solve_part2::<2, 1>(input), solve_part1::<2>(input));
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [(
            r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
            285,
        )] {
            assert_eq!(solve_part2::<50, 20>(input), expected);
        }
    }

    #[test]
    fn test_cost_calculations() {
        let grid = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

        let grid = Grid::from(grid);

        let optimal_path = grid
            .find_optimal_path(grid.start_offset, grid.end_offset)
            .unwrap();
        let costs = grid.calculate_all_costs_from(grid.end_offset);

        assert_eq!(optimal_path.cost, *costs.get(&grid.start_offset).unwrap());
    }
}
