use std::{collections::HashSet, time::Instant};

use advent_of_code_2025_lib::Offset;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let before = Instant::now();
    let part_1_result = solve_part1(INPUT);
    let after = Instant::now();
    println!("Part 1: {part_1_result}");
    println!("Part 1 took: {:?}", after - before);

    let before = Instant::now();
    let part_2_result = solve_part2(INPUT);
    let after = Instant::now();
    println!("Part 2: {part_2_result}");
    println!("Part 2 took: {:?}", after - before);
}

#[inline(never)]
fn solve_part1(input: &str) -> usize {
    let red_tiles = input
        .lines()
        .flat_map(Offset::<isize, 2>::from_comma_separated_str)
        .collect::<Vec<_>>();

    red_tiles
        .iter()
        .enumerate()
        .flat_map(|(i, a)| red_tiles.iter().skip(i + 1).map(move |b| (a, b)))
        .map(|(a, b)| {
            let diff = (a - b).abs();
            (diff.x().cast_unsigned() + 1) * (diff.y().cast_unsigned() + 1)
        })
        .max()
        .expect("we have at least one pair")
}

#[inline(never)]
fn solve_part2(input: &str) -> usize {
    let red_tiles = input
        .lines()
        .flat_map(Offset::<isize, 2>::from_comma_separated_str)
        .collect::<Vec<_>>();

    let mut wall_tiles = HashSet::with_capacity(red_tiles.len() * 10);

    for i in 0..red_tiles.len() {
        let from = red_tiles[i];
        let to = red_tiles[(i + 1) % red_tiles.len()];

        let mut delta = to - from;
        *delta.x_mut() = delta.x().signum();
        *delta.y_mut() = delta.y().signum();

        let mut next = from + delta;
        while next != to {
            wall_tiles.insert(next);
            next += delta;
        }
    }

    red_tiles
        .iter()
        .enumerate()
        .flat_map(|(i, a)| red_tiles.iter().skip(i + 1).map(move |b| (a, b)))
        .map(|(&a, &b)| Square::from(a, b))
        .fold(usize::MIN, |biggest, square| {
            if square.area > biggest
                && wall_tiles
                    .iter()
                    .all(|wall_tile| square.does_not_contain(*wall_tile))
            {
                square.area
            } else {
                biggest
            }
        })
}

#[derive(Default)]
struct Square {
    area: usize,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Square {
    fn from(a: Offset<isize, 2>, b: Offset<isize, 2>) -> Self {
        Square {
            area: {
                let diff = (a - b).abs();
                (diff.x().cast_unsigned() + 1) * (diff.y().cast_unsigned() + 1)
            },
            x_min: *a.x().min(b.x()) + 1,
            x_max: *a.x().max(b.x()) - 1,
            y_min: *a.y().min(b.y()) + 1,
            y_max: *a.y().max(b.y()) - 1,
        }
    }

    fn does_not_contain(&self, offset: Offset<isize, 2>) -> bool {
        *offset.x() < self.x_min
            || self.x_max < *offset.x()
            || *offset.y() < self.y_min
            || self.y_max < *offset.y()
    }
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        self.area == other.area
            && self.x_min == other.x_min
            && self.x_max == other.x_max
            && self.y_min == other.y_min
            && self.y_max == other.y_max
    }
}

impl Eq for Square {}

impl PartialOrd for Square {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Square {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area.cmp(&other.area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

        assert_eq!(solve_part1(input), 50);
    }

    #[test]
    fn test_input_part2() {
        let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

        assert_eq!(solve_part2(input), 24);
    }
}
