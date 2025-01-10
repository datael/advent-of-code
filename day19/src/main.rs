use std::collections::HashMap;

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let (towel_patterns, designs) = input.split_once("\n\n").unwrap();

    let towel_patterns = towel_patterns
        .split(", ")
        .map(|pattern| format!("(?:{})", pattern))
        .collect::<Vec<_>>();

    let regex_pattern = format!("^({})+$", towel_patterns.join("|"));
    let regex = Regex::new(regex_pattern.as_str()).expect("Invalid regex");

    designs.lines().filter(|line| regex.is_match(line)).count()
}

fn solve_part2(input: &str) -> usize {
    let (towel_patterns, designs) = input.split_once("\n\n").unwrap();

    let towel_patterns = towel_patterns.split(", ").collect::<Vec<_>>();

    // Approach this similarly to the stones one?
    // Let's try something naïve first...

    let mut num = 0;
    let mut memo = HashMap::<&str, usize>::new();

    for design in designs.lines() {
        let design_num = recurse(design, &towel_patterns, &mut memo);

        fn recurse<'a>(
            rem: &'a str,
            towel_patterns: &[&str],
            memo: &mut HashMap<&'a str, usize>,
        ) -> usize {
            if let Some(memo) = memo.get(rem) {
                return *memo;
            }

            let mut recurse_num = 0;

            for pattern in towel_patterns {
                if let Some(rem) = rem.strip_prefix(pattern) {
                    if rem.is_empty() {
                        recurse_num += 1;
                    } else {
                        recurse_num += recurse(rem, towel_patterns, memo);
                    }
                }
            }

            memo.insert(rem, recurse_num);
            recurse_num
        }

        num += design_num;
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
            6,
        )] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [(
            r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
            16,
        )] {
            assert_eq!(solve_part2(input), expected);
        }
    }

    #[test]
    fn test_part2_singles() {
        let towels = "r, wr, b, g, bwu, rb, gb, br";

        for (input, expected) in [
            ("brwrr", 2),
            ("bggr", 1),
            ("gbbr", 4),
            ("rrbgbr", 6),
            ("ubwu", 0),
            ("bwurrg", 1),
            ("brgr", 2),
            ("bbrgwb", 0),
        ] {
            assert_eq!(
                solve_part2((towels.to_string() + "\n\n" + input).as_str()),
                expected
            );
        }
    }
}
