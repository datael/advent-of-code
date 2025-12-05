use std::{iter, time::Instant};

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

fn solve_part1(input: &str) -> usize {
    let (fresh_ranges, ingredients) = input.split_once("\n\n").unwrap();

    let fresh_ranges = fresh_ranges
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(from, to)| str::parse(from).unwrap()..=str::parse(to).unwrap())
        .collect::<Vec<_>>();

    ingredients
        .lines()
        .flat_map(str::parse::<usize>)
        .filter(|ingredient| {
            fresh_ranges
                .iter()
                .any(|fresh_range| fresh_range.contains(ingredient))
        })
        .count()
}

#[inline(never)]
fn solve_part2(input: &str) -> usize {
    let (fresh_ranges, _) = input.split_once("\n\n").unwrap();

    let mut fresh_ranges = fresh_ranges
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(from, to)| str::parse::<usize>(from).unwrap()..=str::parse(to).unwrap())
        .collect::<Vec<_>>();

    fresh_ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let first = fresh_ranges[0].clone();

    let (ranges, working) = fresh_ranges.into_iter().skip(1).fold(
        (Vec::new(), first),
        |(mut ranges, mut working), fresh_range| {
            if working.end() >= fresh_range.start() {
                // merge if they overlap
                working = *working.start()..=*working.end().max(fresh_range.end());
            } else {
                // if they don't, then push current working to the finished ranges and start a new one
                ranges.push(working);
                working = fresh_range;
            }

            (ranges, working)
        },
    );

    ranges
        .iter()
        .chain(iter::once(&working))
        .map(|range| range.end() + 1 - range.start())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

        assert_eq!(solve_part1(input), 3);
    }

    #[test]
    fn test_input_part2() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

        assert_eq!(solve_part2(input), 14);
    }
}
