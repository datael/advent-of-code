use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    solve_part1(INPUT);
    solve_part2(INPUT);
}

fn solve_part1(input: &str) {
    let sum: u32 = input
        .lines()
        .filter_map(|line| {
            let (game, possible) = Part1::was_possible(line);
            if possible {
                Some(game)
            } else {
                None
            }
        })
        .sum();

    println!("Part 1: {}", sum);
}

struct Part1;

impl Part1 {
    fn was_possible(line: &str) -> (u32, bool) {
        let available = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

        let mut initial = line.split(':');

        let game: u32 = initial
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let inputs = initial.next().unwrap();

        for set in inputs.split(';') {
            for readout in set.split(',') {
                let mut iter = readout.trim().split(' ');
                let num: u32 = iter.next().unwrap().trim().parse().unwrap();
                let color = iter.next().unwrap().trim();

                if let Some(available_num) = available.get(color) {
                    if num > *available_num {
                        return (game, false);
                    }
                } else {
                    continue;
                }
            }
        }

        (game, true)
    }
}

fn solve_part2(input: &str) {
    let sum: u32 = input.lines().map(Part2::calculate_power).sum();

    println!("Part 2: {}", sum);
}

struct Part2;

impl Part2 {
    fn calculate_power(line: &str) -> u32 {
        let mut maxes = HashMap::new();

        let mut initial = line.split(':');
        let inputs = initial.nth(1).unwrap();

        for set in inputs.split(';') {
            for readout in set.split(',') {
                let mut iter = readout.trim().split(' ');
                let num: u32 = iter.next().unwrap().trim().parse().unwrap();
                let color = iter.next().unwrap().trim();

                if let Some(old_max) = maxes.get_mut(color) {
                    *old_max = std::cmp::max(*old_max, num);
                } else {
                    maxes.insert(color, num);
                }
            }
        }

        maxes.values().fold(1, |acc, x| acc * x)
    }
}
