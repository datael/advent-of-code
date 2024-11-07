use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let containers = input
        .lines()
        .flat_map(str::parse::<u32>)
        .collect::<Vec<_>>();

    let results = count_combinations::<150>(&containers);

    results.values().sum()
}

fn count_combinations<const TARGET: u32>(containers: &[u32]) -> HashMap<usize, usize> {
    let mut scratch = Vec::with_capacity(containers.len());

    (0..(1 << containers.len()))
        .filter_map(|n| {
            scratch.clear();
            nth_combination(containers, n, &mut scratch);
            if TARGET == scratch.iter().sum() {
                Some(scratch.len())
            } else {
                None
            }
        })
        .fold(HashMap::new(), |mut map, res| {
            *map.entry(res).or_default() += 1;
            map
        })
}

fn nth_combination(containers: &[u32], n: usize, result: &mut Vec<u32>) {
    result.extend(containers.iter().enumerate().filter_map(|(bit, capacity)| {
        if n & (1 << bit) != 0 {
            Some(capacity)
        } else {
            None
        }
    }));
}

fn solve_part2(input: &str) -> usize {
    let containers = input
        .lines()
        .flat_map(str::parse::<u32>)
        .collect::<Vec<_>>();

    let results = count_combinations::<150>(&containers);

    *results.iter().min_by_key(|a| a.0).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [([20, 15, 10, 5, 5], 4)] {
            assert_eq!(
                count_combinations::<25>(&input).values().sum::<usize>(),
                expected
            );
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [([20, 15, 10, 5, 5], 3)] {
            assert_eq!(
                *count_combinations::<25>(&input)
                    .iter()
                    .min_by_key(|a| a.0)
                    .unwrap()
                    .1,
                expected
            );
        }
    }
}
