use std::collections::HashMap;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");
const INPUT_TEST_PART2: &str = include_str!("../input_test_part2.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 6);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST_PART2);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 6);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> u32 {
    let mut lines = input.lines();

    let mut directions = lines
        .next()
        .unwrap()
        .chars()
        .map(as_direction_index)
        .cycle();

    lines.next(); // blank line

    let all_maps: HashMap<&str, [&str; 2]> = HashMap::from_iter(lines.map(parse_map));

    let mut steps = 0;
    let mut current_key = "AAA";

    while current_key != "ZZZ" {
        current_key = all_maps.get(current_key).unwrap()[directions.next().unwrap()];
        steps += 1;
    }

    steps
}

fn as_direction_index(c: char) -> usize {
    match c {
        'L' => 0,
        'R' => 1,
        _ => unreachable!(),
    }
}

fn parse_map(line: &str) -> (&str, [&str; 2]) {
    let (key, rest) = line.split_once(" = (").unwrap();
    let (left, right) = rest.split_once(", ").unwrap();
    let right = right.trim_end_matches(')');

    (key, [left, right])
}

fn solve_part2(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut directions = lines
        .next()
        .unwrap()
        .chars()
        .map(as_direction_index)
        .cycle();

    lines.next(); // blank line

    let all_maps: HashMap<&str, [&str; 2]> = HashMap::from_iter(lines.map(parse_map));

    let all_steps = all_maps
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|mut key| {
            let mut steps = 0u64;

            while !key.ends_with('Z') {
                let direction = directions.next().unwrap();
                key = &all_maps.get(key).unwrap()[direction];
                steps += 1;
            }

            steps
        })
        .collect::<Vec<_>>();

    lcm(all_steps[0], &all_steps[1..])
}

fn lcm(a: u64, b: &[u64]) -> u64 {
    if b.len() == 0 {
        return a;
    }
    let b = lcm(b[0], &b[1..]);

    a * (b / gcd(a, b))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}
