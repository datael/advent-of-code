use std::ops::RangeInclusive;

use fancy_regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve::<Part1>(INPUT);
    println!("Part 1: {part_1_result}");

    let part_2_result = solve::<Part2>(INPUT);
    println!("Part 2: {part_2_result}");
}

fn solve<S: Strategy>(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|range| {
            let range = parse_range(range);
            range.get_invalid_ids_sum::<S>()
        })
        .sum()
}

fn parse_range(input: &str) -> RangeInclusive<usize> {
    let (from, to) = input.split_once('-').unwrap();
    from.parse().unwrap()..=to.parse().unwrap()
}

trait ValidatedRangeExt {
    fn get_invalid_ids_sum<S: Strategy>(&self) -> usize;
}

impl ValidatedRangeExt for RangeInclusive<usize> {
    fn get_invalid_ids_sum<S: Strategy>(&self) -> usize {
        let strategy = S::default();

        self.clone()
            .filter(|number| strategy.is_invalid(*number))
            .sum()
    }
}

trait Strategy: Default {
    fn is_invalid(&self, number: usize) -> bool;
}

struct Part1(Regex);

impl Default for Part1 {
    fn default() -> Self {
        Self(Regex::new("^(.+)\\1$").expect("Should compile"))
    }
}

impl Strategy for Part1 {
    fn is_invalid(&self, number: usize) -> bool {
        self.0.is_match(format!("{number}").as_str()).unwrap()
    }
}

struct Part2(Regex);

impl Default for Part2 {
    fn default() -> Self {
        Self(Regex::new("^(.+)\\1+$").expect("Should compile"))
    }
}

impl Strategy for Part2 {
    fn is_invalid(&self, number: usize) -> bool {
        self.0.is_match(format!("{number}").as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

        assert_eq!(solve::<Part1>(input), 1227775554);
    }

    #[test]
    fn test_is_valid_part1() {
        for (input, expected) in [
            ("11-22", 33),
            ("95-115", 99),
            ("998-1012", 1010),
            ("1188511880-1188511890", 1188511885),
            ("222220-222224", 222222),
            ("1698522-1698528", 0),
            ("446443-446449", 446446),
            ("38593856-38593862", 38593859),
            ("565653-565659", 0),
            ("824824821-824824827", 0),
            ("2121212118-2121212124", 0),
        ] {
            assert_eq!(
                parse_range(dbg!(input)).get_invalid_ids_sum::<Part1>(),
                expected
            );
        }
    }

    #[test]
    fn test_input_part2() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

        assert_eq!(solve::<Part2>(input), 4174379265);
    }

    #[test]
    fn test_is_valid_part2() {
        for (input, expected) in [
            ("11-22", 33),
            ("95-115", 210),
            ("998-1012", 2009),
            ("1188511880-1188511890", 1188511885),
            ("222220-222224", 222222),
            ("1698522-1698528", 0),
            ("446443-446449", 446446),
            ("38593856-38593862", 38593859),
            ("565653-565659", 565656),
            ("824824821-824824827", 824824824),
            ("2121212118-2121212124", 2121212121),
        ] {
            assert_eq!(
                parse_range(dbg!(input)).get_invalid_ids_sum::<Part2>(),
                expected
            );
        }
    }
}
