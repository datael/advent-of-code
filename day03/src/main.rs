use std::{cmp::Reverse, ops::Deref};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {part_1_result}");

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {part_2_result}");
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(Battery::from)
        .map(|battery| battery.get_highest_joltage::<2>())
        .sum()
}

struct Battery<'i>(&'i str);

impl<'i> From<&'i str> for Battery<'i> {
    fn from(value: &'i str) -> Self {
        Self(value)
    }
}

impl<'i> Deref for Battery<'i> {
    type Target = &'i str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Battery<'_> {
    fn get_highest_joltage<const N: usize>(&self) -> usize {
        let mut rchop = N - 1;
        let mut lchop = 0;

        let mut res = 0;
        loop {
            res *= 10;

            let haystack = &self[lchop..self.len() - rchop];

            let (i, max_char) = haystack
                .char_indices()
                .max_by(|a, b| (a.1, Reverse(a.0)).cmp(&(b.1, Reverse(b.0))))
                .unwrap();

            lchop += i + 1;

            res += (max_char as u8 - b'0') as usize;

            if rchop == 0 {
                break;
            }

            rchop -= 1;
        }

        res
    }
}

fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(Battery::from)
        .map(|battery| battery.get_highest_joltage::<12>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

        assert_eq!(solve_part1(input), 357);
    }

    #[test]
    fn test_get_largest_jolt_possible_part1() {
        for (input, expected) in [
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92),
        ] {
            assert_eq!(Battery::from(input).get_highest_joltage::<2>(), expected);
        }
    }

    #[test]
    fn test_get_largest_jolt_possible_regressions() {
        for (input, expected) in [(
            "387654662333668423563255665353333636485343343353551333336214434344445486668335343356376334456343533",
            88,
        )] {
            assert_eq!(Battery::from(input).get_highest_joltage::<2>(), expected);
        }
    }

    #[test]
    fn test_input_part2() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

        assert_eq!(solve_part2(input), 3121910778619);
    }

    #[test]
    fn test_get_largest_jolt_possible_part2() {
        for (input, expected) in [
            ("987654321111111", 987654321111),
            ("811111111111119", 811111111119),
            ("234234234234278", 434234234278),
            ("818181911112111", 888911112111),
        ] {
            assert_eq!(Battery::from(input).get_highest_joltage::<12>(), expected);
        }
    }
}
