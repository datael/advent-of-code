use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    regex
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap())
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let regex =
        Regex::new(r"(?:do\(\))|(?:don't\(\))|(?:mul\(([0-9]{1,3}),([0-9]{1,3})\))").unwrap();

    #[derive(PartialEq, Eq)]
    enum State {
        Do,
        Dont,
    }

    let mut state = State::Do;
    let mut sum = 0;

    for capture in regex.captures_iter(input) {
        match capture.get(0).unwrap().as_str() {
            "don't()" => state = State::Dont,
            "do()" => state = State::Do,
            _ => {
                if state == State::Do {
                    sum += capture.get(1).unwrap().as_str().parse::<usize>().unwrap()
                        * capture.get(2).unwrap().as_str().parse::<usize>().unwrap()
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            161,
        )] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            48,
        )] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
