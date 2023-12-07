use std::collections::HashSet;
use std::iter::FromIterator;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 13);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 30);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut sets = line.split(':').nth(1).unwrap().split('|');

        let winning = HashSet::<u32>::from_iter(
            sets.next()
                .unwrap()
                .split(' ')
                .filter_map(|s| s.parse::<u32>().ok()),
        );

        let ours = HashSet::from_iter(
            sets.next()
                .unwrap()
                .split(' ')
                .filter_map(|s| s.trim().parse::<u32>().ok()),
        );

        let winners = winning.intersection(&ours).count();

        if winners == 0 {
            continue;
        }

        sum += 2_u32.pow(winning.intersection(&ours).count() as u32 - 1);
    }

    sum
}

fn solve_part2(input: &str) -> u32 {
    let mut winner_counts = vec![];

    for line in input.lines() {
        let mut sets = line.split(':').nth(1).unwrap().split('|');

        let winning = HashSet::<u32>::from_iter(
            sets.next()
                .unwrap()
                .split(' ')
                .filter_map(|s| s.trim().parse::<u32>().ok()),
        );

        let ours = HashSet::from_iter(
            sets.next()
                .unwrap()
                .split(' ')
                .filter_map(|s| s.trim().parse::<u32>().ok()),
        );

        let winners = winning.intersection(&ours).count();

        winner_counts.push(winners);
    }

    let mut scratch_counts = vec![1; winner_counts.len()];

    for (i, winner_count) in winner_counts.iter().enumerate() {
        for j in i + 1..i + 1 + winner_count {
            scratch_counts[j] += scratch_counts[i];
        }
    }

    scratch_counts.iter().sum()
}
