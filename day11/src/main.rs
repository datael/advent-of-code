use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, LinkedList},
    ops::{ControlFlow, Deref, DerefMut},
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let mut stones = LinkedList::<Stone>::new();

    for stone in input
        .lines()
        .next()
        .expect("Single-line input expected")
        .split(' ')
        .map(Stone::from)
    {
        stones.push_back(stone);
    }

    let mut next = LinkedList::<Stone>::new();
    for _ in 0..25 {
        blink(&mut stones, &mut next);
    }

    stones.len()
}

fn blink(stones: &mut LinkedList<Stone>, next: &mut LinkedList<Stone>) {
    assert!(next.is_empty());

    while let Some(stone) = stones.pop_front() {
        if stone == Stone(0) {
            next.push_back(Stone(1));
            continue;
        }

        let num_digits = (stone.0 as f64).log10() as u32;
        if num_digits % 2 == 1 {
            let divisor = 10usize.pow((num_digits + 1) / 2);
            next.push_back(Stone(stone.0 / divisor));
            next.push_back(Stone(stone.0 % divisor));
            continue;
        }

        next.push_back(Stone(stone.0 * 2024));
    }

    std::mem::swap(stones, next);
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
struct Stone(usize);

impl<S> From<S> for Stone
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        Self(value.as_ref().parse::<usize>().expect("Invalid input"))
    }
}

impl Deref for Stone {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn solve_part2(input: &str) -> usize {
    let input = input
        .lines()
        .next()
        .expect("Single-line input expected")
        .split(' ')
        .map(Stone::from)
        .map(|stone| (75, stone))
        .collect::<Vec<_>>();

    let mut to_calculate = BinaryHeap::<Reverse<(u8, Stone)>>::new();
    let mut known_lengths = HashMap::<(u8, Stone), usize>::new();

    for entry in input.iter() {
        to_calculate.push(Reverse(*entry));
    }

    while let Some(Reverse(entry)) = to_calculate.pop() {
        if entry.0 == 0 {
            known_lengths.insert(entry, 1);
            continue;
        }

        let expanded = expand(entry);

        let maybe_sum = expanded.iter().try_fold(0, |length_sum, entry| {
            if let Some(known_length) = known_lengths.get(entry) {
                ControlFlow::Continue(length_sum + known_length)
            } else {
                ControlFlow::Break(())
            }
        });

        if let ControlFlow::Continue(sum) = maybe_sum {
            known_lengths.insert(entry, sum);
        } else {
            expanded
                .iter()
                .filter(|entry| !known_lengths.contains_key(entry))
                .for_each(|entry| {
                    to_calculate.push(Reverse(*entry));
                });
            to_calculate.push(Reverse(entry));
        }
    }

    input.iter().fold(0, |sum, entry| {
        sum + known_lengths.get(entry).expect("We just calculated this")
    })
}

fn expand((depth, stone): (u8, Stone)) -> Vec<(u8, Stone)> {
    assert!(depth > 0);

    let next_depth = depth - 1;

    if stone == Stone(0) {
        return vec![(next_depth, Stone(1))];
    }

    let num_digits = (stone.0 as f64).log10() as u32;
    if num_digits % 2 == 1 {
        let divisor = 10usize.pow((num_digits + 1) / 2);
        return vec![
            (next_depth, Stone(stone.0 / divisor)),
            (next_depth, Stone(stone.0 % divisor)),
        ];
    }

    vec![(next_depth, Stone(stone.0 * 2024))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [(r"125 17", 55312)] {
            assert_eq!(solve_part1(input), expected);
        }
    }
}
