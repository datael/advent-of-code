use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> u32 {
    let (mut left, mut right) = parse_lists(input);

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .fold((vec![], vec![]), |(mut left, mut right), line| {
            let (l, r) = parse_entry(line);
            left.push(l);
            right.push(r);
            (left, right)
        })
}

fn parse_entry(line: &str) -> (u32, u32) {
    let (l, r) = line.split_once("   ").unwrap();
    (l.parse().unwrap(), r.parse().unwrap())
}

fn solve_part2(input: &str) -> u32 {
    let (left, right) = parse_lists(input);

    let counts_in_right = right
        .iter()
        .fold(HashMap::<&u32, u32>::new(), |mut counts, r| {
            *counts.entry(r).or_default() += 1;

            counts
        });

    left.iter()
        .map(|l| l * counts_in_right.get(l).copied().unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entry() {
        for (input, expected) in [("79931   16117", (79931, 16117))] {
            assert_eq!(parse_entry(input), expected);
        }
    }

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            r"3   4
4   3
2   5
1   3
3   9
3   3",
            11,
        )] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [(
            r"3   4
4   3
2   5
1   3
3   9
3   3",
            31,
        )] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
