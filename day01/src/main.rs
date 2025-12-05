use std::time::Instant;

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
    input
        .lines()
        .map(parse_line)
        .fold(
            (50i16, 0usize),
            |(mut current_position, mut zeroes), next| {
                current_position += next;

                current_position = (current_position + 100) % 100;
                if current_position == 0 {
                    zeroes += 1;
                }

                (current_position, zeroes)
            },
        )
        .1
}

fn parse_line(line: &str) -> i16 {
    let sign: i16 = if line.starts_with('L') { -1 } else { 1 };

    let num = line[1..].parse::<i16>().unwrap();

    num * sign
}

fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .fold(
            (50i16, 0usize),
            |(mut current_position, mut zeroes), next| {
                let was_zero = current_position == 0;
                let prev_sign = current_position > 0;
                current_position += next;
                let new_sign = current_position > 0;

                if !was_zero && prev_sign != new_sign {
                    zeroes += 1;
                }

                zeroes += current_position.unsigned_abs() as usize / 100;
                while current_position < 0 {
                    current_position += 100;
                }
                current_position %= 100;

                (current_position, zeroes)
            },
        )
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        assert_eq!(solve_part1(input), 3);
    }

    #[test]
    fn test_input_part2() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        assert_eq!(solve_part2(input), 6);
    }
}
