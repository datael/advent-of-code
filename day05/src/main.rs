use fancy_regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(evaluate_string_part1)
        .filter(|evaluation| *evaluation == Evaluation::Nice)
        .count()
}

fn evaluate_string_part1(input: &str) -> Evaluation {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    const FORBIDDEN: [&[u8; 2]; 4] = [b"ab", b"cd", b"pq", b"xy"];

    let mut has_run_of_two_same = false;
    let mut vowel_count = 0;
    let mut prev = None;

    for c in input.chars() {
        if VOWELS.contains(&c) {
            vowel_count += 1;
        }

        if let Some(prev) = prev {
            if FORBIDDEN.contains(&&[prev as u8, c as u8]) {
                return Evaluation::Naughty;
            }

            if prev == c {
                has_run_of_two_same = true;
            }
        }

        prev = Some(c);
    }

    if vowel_count < 3 {
        return Evaluation::Naughty;
    }

    if !has_run_of_two_same {
        return Evaluation::Naughty;
    }

    Evaluation::Nice
}

#[derive(Debug, PartialEq, Eq)]
enum Evaluation {
    Naughty,
    Nice,
}

fn solve_part2(input: &str) -> usize {
    let rules = [
        Regex::new(r"(\w\w).*\1").unwrap(),
        Regex::new(r"(\w).\1").unwrap(),
    ];

    input
        .lines()
        .map(|line| {
            if rules.iter().all(|rule| rule.is_match(line).unwrap()) {
                Evaluation::Nice
            } else {
                Evaluation::Naughty
            }
        })
        .filter(|evaluation| *evaluation == Evaluation::Nice)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [
            ("ugknbfddgicrmopn", Evaluation::Nice),
            ("aaa", Evaluation::Nice),
            ("jchzalrnumimnmhp", Evaluation::Naughty),
            ("haegwjzuvuyypxyu", Evaluation::Naughty),
            ("dvszwmarrgswjxmb", Evaluation::Naughty),
        ] {
            assert_eq!(evaluate_string_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [
            ("qjhvhtzxzqqjkmpb", 1),
            ("xxyxx", 1),
            ("uurcxstgmygtbstg", 0),
            ("ieodomkazucvgmuy", 0),
        ] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
