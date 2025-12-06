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

#[inline(never)]
fn solve_part1(input: &str) -> usize {
    // commutative ops, so we can reverse safely and pull the ops out "first"
    let mut lines = input.lines().rev();

    let ops_iter = lines.next().unwrap().split_whitespace();
    let mut input_line_iters = lines.map(str::split_whitespace).collect::<Vec<_>>();

    let mut grand_total = 0;

    for op in ops_iter {
        let op: Op = op.into();

        let mut result = *op.identity();

        for input_line_iter in input_line_iters.iter_mut() {
            let next_value = input_line_iter
                .next()
                .expect("the same number of columns per line")
                .parse::<usize>()
                .expect("input lines should be numbers");

            result = op.append(&result, &next_value);
        }

        grand_total += result;
    }

    grand_total
}

#[derive(PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => panic!("unexpected op"),
        }
    }
}

trait Monoid<T> {
    fn identity(&self) -> &T;
    fn append(&self, a: &T, b: &T) -> T;
}

impl Monoid<usize> for Op {
    fn identity(&self) -> &usize {
        match self {
            Self::Add => &0,
            Self::Mul => &1,
        }
    }

    fn append(&self, a: &usize, b: &usize) -> usize {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
        }
    }
}

#[inline(never)]
fn solve_part2(input: &str) -> usize {
    let mut lines = input.lines().rev();

    let ops_iter = lines.next().unwrap().split_whitespace();
    let mut input_line_iters = lines
        .rev()
        .map(|line| line.chars().peekable())
        .collect::<Vec<_>>();

    let mut grand_total = 0;

    for op in ops_iter {
        let op: Op = op.into();

        let mut result = *op.identity();

        loop {
            let mut next_value = 0usize;
            let mut should_continue = false;

            for input_line_iter in input_line_iters.iter_mut() {
                if let Some(c) = input_line_iter.next()
                    && c != ' '
                {
                    next_value *= 10;
                    next_value += (c as u8 - b'0') as usize;

                    should_continue |= input_line_iter.peek().is_some_and(|next_c| *next_c != ' ');
                }
            }

            result = op.append(&result, &next_value);

            if !should_continue {
                for input_line_iter in input_line_iters.iter_mut() {
                    _ = input_line_iter.next();
                }
                break;
            }
        }

        grand_total += result;
    }

    grand_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let input = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#;

        assert_eq!(solve_part1(input), 4277556);
    }

    #[test]
    fn test_input_part2() {
        let input = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#;

        assert_eq!(solve_part2(input), 3263827);
    }
}
