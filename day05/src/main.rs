use std::{iter, vec};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 35);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 46);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

struct Remap {
    from_start: u64,
    to_start: u64,
    num: u64,
}

impl Remap {
    fn maybe_remap(&self, value: u64) -> Option<u64> {
        if self.from_start <= value && value < (self.from_start + self.num) {
            Some(value + self.to_start - self.from_start)
        } else {
            None
        }
    }
}

struct AlmanacRemapper {
    remaps: Vec<Remap>,
}

impl AlmanacRemapper {
    fn new() -> Self {
        Self { remaps: vec![] }
    }

    fn add(&mut self, remap: Remap) {
        self.remaps.push(remap);
    }

    fn remap(&self, value: u64) -> u64 {
        for remap in &self.remaps {
            if let Some(remapped) = remap.maybe_remap(value) {
                return remapped;
            }
        }

        value
    }
}

fn solve_part1(input: &str) -> u64 {
    let mut input = input.lines();

    let (_, seeds) = input.next().unwrap().split_once(':').unwrap();
    let seeds = seeds
        .trim()
        .split(' ')
        .filter_map(|s| s.trim().parse::<u64>().ok());

    let remappers = build_almanac_remappers(input);

    find_optimal_location(seeds, &remappers)
}

fn find_optimal_location<'a, Seeds: Iterator<Item = u64>>(
    seeds: Seeds,
    remappers: &Vec<AlmanacRemapper>,
) -> u64 {
    seeds
        .map(|seed| {
            remappers
                .iter()
                .fold(seed, |seed, remapper| remapper.remap(seed))
        })
        .min()
        .unwrap()
}

fn build_almanac_remappers<'a, Lines: Iterator<Item = &'a str>>(
    lines: Lines,
) -> Vec<AlmanacRemapper> {
    let mut remappers = vec![];

    let mut lines = lines.peekable();

    while lines.peek().is_some() {
        if let Some(remapper) = build_almanac_remapper(&mut lines) {
            remappers.push(remapper);
        }
    }

    remappers
}

fn build_almanac_remapper<'a, Lines: Iterator<Item = &'a str>>(
    lines: &mut Lines,
) -> Option<AlmanacRemapper> {
    if let Some(line) = lines.next() {
        if line.is_empty() {
            return None;
        } else {
            let mut remapper = AlmanacRemapper::new();

            while let Some(line) = lines.next() {
                if line.is_empty() {
                    break;
                }

                let (to_start, rem) = line.split_once(' ').unwrap();
                let (from_start, num) = rem.split_once(' ').unwrap();

                let from_start = from_start.trim().parse().unwrap();
                let to_start = to_start.trim().parse().unwrap();
                let num = num.trim().parse().unwrap();

                remapper.add(Remap {
                    from_start,
                    to_start,
                    num,
                });
            }

            return Some(remapper);
        }
    }

    None
}

fn solve_part2(input: &str) -> u64 {
    let mut input = input.lines();

    let (_, seed_ranges) = input.next().unwrap().split_once(':').unwrap();
    let mut seed_ranges = seed_ranges
        .trim()
        .split(' ')
        .filter_map(|s| s.trim().parse::<u64>().ok());

    let seeds = iter::from_fn(move || {
        if let (Some(from), Some(count)) = (seed_ranges.next(), seed_ranges.next()) {
            Some(from..(from + count))
        } else {
            None
        }
    })
    .flatten();

    let remappers = build_almanac_remappers(input);

    find_optimal_location(seeds, &remappers)
}
