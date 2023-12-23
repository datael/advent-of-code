use std::iter;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 405);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 400);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    extract_patterns(input)
        .map(|(rows, columns)| {
            100 * (find_reflection_index::<0>(&rows)
                .map(|i| i + 1)
                .unwrap_or(0))
                + (find_reflection_index::<0>(&columns)
                    .map(|i| i + 1)
                    .unwrap_or(0))
        })
        .sum()
}

fn extract_patterns<'a>(input: &'a str) -> impl Iterator<Item = (Vec<u64>, Vec<u64>)> + 'a {
    let mut lines = input.lines().peekable();

    iter::from_fn(move || {
        let mut pattern = Vec::new();

        while let Some(line) = lines.next() {
            if line == "" {
                break;
            }

            pattern.push(line);
        }

        if pattern.len() == 0 {
            None
        } else {
            Some(parse_pattern(pattern.iter().copied()))
        }
    })
}

fn parse_pattern<'a>(rows: impl Iterator<Item = &'a str>) -> (Vec<u64>, Vec<u64>) {
    let mut rows = rows.peekable();
    let width = rows.peek().unwrap().len();

    let rows = rows.map(line_to_row).collect::<Vec<_>>();
    let columns = transpose(&rows, width);

    (rows, columns)
}

fn line_to_row(line: &str) -> u64 {
    let mut row = 0u64;

    for (i, c) in line.chars().enumerate() {
        if c == '#' {
            row |= 1 << i;
        }
    }

    row
}

fn transpose(rows: &Vec<u64>, width: usize) -> Vec<u64> {
    let mut columns = vec![0u64; width];

    for (r, row) in rows.iter().enumerate() {
        for (c, column) in columns.iter_mut().enumerate() {
            *column |= ((row >> c) & 1) << r;
        }
    }

    columns
}

fn find_reflection_index<const DIFFS: u32>(input: &Vec<u64>) -> Option<usize> {
    for i in 0..input.len() - 1 {
        let mut ii = i;
        let mut jj = i + 1;
        let mut diffs = 0u32;

        while jj < input.len() {
            diffs += (input[ii] ^ input[jj]).count_ones();

            if diffs > DIFFS {
                break;
            }

            if ii == 0 {
                break;
            }

            ii -= 1;
            jj += 1;
        }

        if diffs == DIFFS {
            return Some(i);
        }
    }

    None
}

fn solve_part2(input: &str) -> usize {
    extract_patterns(input)
        .map(|(rows, columns)| {
            100 * (find_reflection_index::<1>(&rows)
                .map(|i| i + 1)
                .unwrap_or(0))
                + (find_reflection_index::<1>(&columns)
                    .map(|i| i + 1)
                    .unwrap_or(0))
        })
        .sum()
}
