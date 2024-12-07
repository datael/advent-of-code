const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    input.lines().flat_map(solve_for::<2>).sum()
}

fn solve_for<const BITS: usize>(line: &str) -> Option<usize> {
    let (target, numbers) = line.split_once(": ").unwrap();

    let target = target.parse::<usize>().unwrap();
    let numbers = numbers
        .split(" ")
        .flat_map(str::parse::<usize>)
        .collect::<Vec<_>>();

    let mut operators = Vec::<usize>::with_capacity(numbers.len() - 1);
    for _ in 0..numbers.len() - 1 {
        operators.push(0);
    }

    'outer: loop {
        let mut total = numbers[0];

        for (i, n) in numbers.iter().enumerate().skip(1) {
            match operators[i - 1] {
                0 => {
                    total += n;
                }
                1 => {
                    total *= n;
                }
                2 => {
                    let mut nn = *n;
                    while nn > 0 {
                        total *= 10;
                        nn /= 10;
                    }
                    total += n;
                }
                _ => unreachable!(),
            }
        }

        if total == target {
            return Some(target);
        }

        for i in 0..operators.len() {
            if operators[i] + 1 == BITS {
                operators[i] = 0;
            } else {
                operators[i] += 1;
                continue 'outer;
            }
        }

        break;
    }

    None
}

fn solve_part2(input: &str) -> usize {
    input.lines().flat_map(solve_for::<3>).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
            3749,
        )] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [(
            r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
            11387,
        )] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
