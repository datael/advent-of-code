use std::ops::{Add, Deref, Mul};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(ClawMachine::from)
        .flat_map(|claw_machine| claw_machine.calculate_optimal_solution())
        .map(|(a_pushes, b_pushes)| {
            a_pushes * ClawMachine::get_push_a_cost() + b_pushes * ClawMachine::get_push_b_cost()
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(ClawMachine::from)
        .map(|mut claw_machine| {
            claw_machine.prize.0 = claw_machine.prize.0
                + Offset {
                    x: 10000000000000,
                    y: 10000000000000,
                };
            claw_machine
        })
        .flat_map(|claw_machine| claw_machine.calculate_optimal_solution())
        .map(|(a_pushes, b_pushes)| {
            a_pushes * ClawMachine::get_push_a_cost() + b_pushes * ClawMachine::get_push_b_cost()
        })
        .sum()
}

struct ClawMachine {
    a: Button,
    b: Button,
    prize: Prize,
}

impl<S> From<S> for ClawMachine
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let mut lines = value.as_ref().lines();

        Self {
            a: lines.next().unwrap().into(),
            b: lines.next().unwrap().into(),
            prize: lines.next().unwrap().into(),
        }
    }
}

impl ClawMachine {
    fn get_push_a_cost() -> usize {
        3
    }
    fn get_push_b_cost() -> usize {
        1
    }

    fn calculate_optimal_solution(&self) -> Option<(usize, usize)> {
        // n = number of A presses
        // m = number of B presses
        // p = prize

        // n*ax + m*bx = px
        // n*ay + m*by = py

        //    A     M  =   P
        // ⌈ax bx⌉ ⌈n⌉ = ⌈px⌉
        // ⌊ay by⌋ ⌊m⌋   ⌊py⌋

        // Cramer's Rule: https://en.wikipedia.org/wiki/Cramer%27s_rule

        //     |px bx|
        //     |py by|   px*by - bx*py
        // n = ------- = -------------
        //     |ax bx|   ax*by - bx*ay
        //     |ay by|

        //     |ax px|
        //     |ax py|   ax*py - px*ay
        // m = ------- = -------------
        //     |ax bx|   ax*by - bx*ay
        //     |ay by|

        let det_a = self.a.x * self.b.y - self.b.x * self.a.y;
        let det_n = self.prize.x * self.b.y - self.b.x * self.prize.y;
        let det_m = self.a.x * self.prize.y - self.prize.x * self.a.y;

        if det_a == 0 {
            return None;
        }

        let (n, n_rem) = (det_n / det_a, det_n % det_a);
        let (m, m_rem) = (det_m / det_a, det_m % det_a);

        if n_rem != 0 || m_rem != 0 {
            return None;
        }

        Some((n as usize, m as usize))
    }
}

struct Button(Offset);

impl<S> From<S> for Button
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let (_, def) = value.as_ref().split_once(": ").unwrap();
        let (x, y) = def.split_once(", ").unwrap();
        let (_, x) = x.split_once('+').unwrap();
        let (_, y) = y.split_once('+').unwrap();

        let x = x.parse::<isize>().unwrap();
        let y = y.parse::<isize>().unwrap();

        Self(Offset { x, y })
    }
}

impl Deref for Button {
    type Target = Offset;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Prize(Offset);

impl<S> From<S> for Prize
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let (_, def) = value.as_ref().split_once(": ").unwrap();
        let (x, y) = def.split_once(", ").unwrap();
        let (_, x) = x.split_once('=').unwrap();
        let (_, y) = y.split_once('=').unwrap();

        let x = x.parse::<isize>().unwrap();
        let y = y.parse::<isize>().unwrap();

        Self(Offset { x, y })
    }
}

impl Deref for Prize {
    type Target = Offset;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Offset {
    x: isize,
    y: isize,
}

impl Add<Offset> for Offset {
    type Output = Offset;

    fn add(self, rhs: Offset) -> Self::Output {
        Offset {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<usize> for Offset {
    type Output = Offset;

    fn mul(self, rhs: usize) -> Self::Output {
        Offset {
            x: self.x * rhs as isize,
            y: self.y * rhs as isize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimal_solution() {
        for (input, expected) in [
            (
                r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400",
                Some((80, 40)),
            ),
            (
                r"Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176",
                None,
            ),
            (
                r"Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450",
                Some((38, 86)),
            ),
            (
                r"Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
                None,
            ),
        ] {
            assert_eq!(
                ClawMachine::from(input).calculate_optimal_solution(),
                expected
            );
        }
    }

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
            480,
        )] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [(
            r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
            875318608908,
        )] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
