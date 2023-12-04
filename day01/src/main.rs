const INPUT: &str = include_str!("../input.txt");

fn main() {
    solve_part1(INPUT);
    solve_part2(INPUT);
}

fn solve_part1(input: &str) {
    let sum = solve::<Part1Strategy>(input);

    println!("Part 1: {}", sum);
}

fn solve_part2(input: &str) {
    let sum = solve::<Part2Strategy>(input);

    println!("Part 2: {}", sum);
}

fn solve<T: Day01Strategy>(input: &str) -> u32 {
    input
        .lines()
        .map(T::get_first_and_last_integers)
        .map(|(a, b)| a * 10 + b)
        .sum()
}

trait Day01Strategy {
    fn get_first_and_last_integers(line: &str) -> (u32, u32);
}

struct Part1Strategy;

impl Day01Strategy for Part1Strategy {
    fn get_first_and_last_integers(line: &str) -> (u32, u32) {
        let first = 'first: loop {
            for i in 0..line.len() {
                let c = line.chars().nth(i).unwrap();
                match c {
                    '0'..='9' => break 'first (c as u32) - ('0' as u32),
                    _ => continue,
                }
            }
        };

        let last = 'last: loop {
            for i in 0..line.len() {
                let c = line.chars().nth_back(i).unwrap();
                match c {
                    '0'..='9' => break 'last (c as u32) - ('0' as u32),
                    _ => continue,
                }
            }
        };

        (first, last)
    }
}

struct Part2Strategy;

impl Day01Strategy for Part2Strategy {
    fn get_first_and_last_integers(line: &str) -> (u32, u32) {
        let map: Vec<(&str, u32)> = vec![
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        let mut l = &line[..];

        let first = 'first: loop {
            for (s, n) in map.iter() {
                if l.starts_with(s) {
                    break 'first *n;
                }
            }

            l = &l[1..];
        };

        l = &line[..];

        let last = 'first: loop {
            for (s, n) in map.iter() {
                if l.ends_with(s) {
                    break 'first *n;
                }
            }

            l = &l[..l.len() - 1];
        };

        (first, last)
    }
}
