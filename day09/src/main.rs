use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    solve::<Part1Strategy>(input)
}

trait Strategy {
    fn initial_best() -> usize;
    fn choose_best(curr: usize, maybe_better: usize) -> usize;
}

struct Part1Strategy;

impl Strategy for Part1Strategy {
    fn initial_best() -> usize {
        usize::MAX
    }

    fn choose_best(curr: usize, maybe_better: usize) -> usize {
        curr.min(maybe_better)
    }
}

fn solve<S: Strategy>(input: &str) -> usize {
    // There's gotta be something to do with a binary heap here but I'm not seeing it.
    // There's only 7 nodes, so the search space (7^7 == 823,543) is small enough to brute force.

    let distances = input.lines().map(parse_line).collect::<HashMap<_, _>>();
    let all_cities = distances
        .keys()
        .fold(HashSet::new(), |mut acc, (from, to)| {
            acc.insert(from);
            acc.insert(to);
            acc
        })
        .iter()
        .copied()
        .copied()
        .collect::<Vec<_>>();

    let all_routes = all_cities.iter().permutations(all_cities.len());
    let mut best_distance = S::initial_best();

    for route in all_routes {
        let distance = route
            .iter()
            .tuple_windows()
            .map(|(from, to)| distances[&(**from, **to).sorted()])
            .sum::<usize>();

        best_distance = S::choose_best(best_distance, distance);
    }

    best_distance
}

fn parse_line(line: &str) -> ((&str, &str), usize) {
    let (route, distance) = line.split_once(" = ").unwrap();

    // Sort alphabetically so that we can reliably hash each route section.
    let (from, to) = route.split_once(" to ").unwrap().sorted();

    let distance = distance.parse().unwrap();

    ((from, to), distance)
}

trait SortedTupleExt<T>
where
    T: Ord,
{
    fn sorted(self) -> (T, T);
}

impl<T> SortedTupleExt<T> for (T, T)
where
    T: Ord,
{
    fn sorted(self) -> (T, T) {
        if self.0 < self.1 {
            (self.0, self.1)
        } else {
            (self.1, self.0)
        }
    }
}

fn solve_part2(input: &str) -> usize {
    solve::<Part2Strategy>(input)
}

struct Part2Strategy;

impl Strategy for Part2Strategy {
    fn initial_best() -> usize {
        usize::MIN
    }

    fn choose_best(curr: usize, maybe_better: usize) -> usize {
        curr.max(maybe_better)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            "London to Dublin = 464\n\
            London to Belfast = 518\n\
            Dublin to Belfast = 141",
            605,
        )] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [(
            "London to Dublin = 464\n\
            London to Belfast = 518\n\
            Dublin to Belfast = 141",
            982,
        )] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
