const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    solve::<Part1Strategy>(input)
}

fn solve<S: Strategy>(input: &str) -> usize {
    let input = input.trim();

    let initial_len = input.len();

    let mut buffer = [b' '; 32];
    for (i, c) in input.chars().enumerate() {
        buffer[i] = c as u8;
    }

    (0..)
        .find(|n| {
            let mut n_len = 0;
            let mut n = *n;

            while n > 0 {
                buffer[initial_len + n_len] = b'0' + (n % 10) as u8;

                n_len += 1;
                n /= 10;
            }

            buffer[initial_len..initial_len + n_len].reverse();

            let hash = md5::compute(&buffer[..initial_len + n_len]);

            S::is_valid(&hash)
        })
        .expect("There should be a solution")
}

trait Strategy {
    fn is_valid(hash: &[u8; 16]) -> bool;
}

struct Part1Strategy;

impl Strategy for Part1Strategy {
    fn is_valid(hash: &[u8; 16]) -> bool {
        hash[0] == 0 && hash[1] == 0 && hash[2] & 0xF0 == 0
    }
}

fn solve_part2(input: &str) -> usize {
    solve::<Part2Strategy>(input)
}

struct Part2Strategy;

impl Strategy for Part2Strategy {
    fn is_valid(hash: &[u8; 16]) -> bool {
        hash[0] == 0 && hash[1] == 0 && hash[2] == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [("abcdef", 609043), ("pqrstuv", 1048970)] {
            assert_eq!(solve_part1(input), expected);
        }
    }
}
