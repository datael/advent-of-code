const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    solve::<40>(input)
}

fn solve<const N: usize>(input: &str) -> usize {
    let mut line = input.lines().next().unwrap().to_string();

    for _ in 0..N {
        line = look_and_say(line);
    }

    line.len()
}

fn look_and_say(input: impl AsRef<str>) -> String {
    let mut chars = input.as_ref().chars();

    let mut result = String::new();
    let mut c = chars.next().unwrap();
    let mut count = 1;

    for next in chars {
        if next == c {
            count += 1;
        } else {
            result += format!("{}{}", count, c).as_str();
            c = next;
            count = 1;
        }
    }

    result += format!("{}{}", count, c).as_str();

    result
}

fn solve_part2(input: &str) -> usize {
    solve::<50>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [
            ("1", "11"),
            ("11", "21"),
            ("21", "1211"),
            ("1211", "111221"),
            ("111221", "312211"),
        ] {
            assert_eq!(look_and_say(input), expected);
        }
    }
}
