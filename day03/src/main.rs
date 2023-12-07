use std::collections::{HashMap, HashSet};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 4361);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 467835);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cell {
    x: i32, // allowing negative values so that we can be lazy about spilling outside of the grid
    y: i32,
}

fn solve_part1(input: &str) -> u32 {
    let mut hotspots = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !c.is_ascii_digit() && c != '.' {
                for dx in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        hotspots.insert(Cell {
                            x: x as i32 + dx,
                            y: y as i32 + dy,
                        });
                    }
                }
            }
        }
    }

    let mut to_include = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut x = 0;

        while x < line.len() {
            let mut len = line.len() - x;

            'next_number: while len > 0 {
                if let Ok(num) = line[x..x + len].parse::<u32>() {
                    'search: for cell_x in x..x + len {
                        if hotspots.contains(&Cell {
                            x: cell_x as i32,
                            y: y as i32,
                        }) {
                            to_include.push(num);
                            break 'search;
                        }
                    }

                    x += len;
                    break 'next_number;
                } else {
                    len -= 1;
                }
            }

            x += 1;
        }
    }

    to_include.iter().sum()
}

fn solve_part2(input: &str) -> u32 {
    let mut hotspots = HashMap::new();

    let mut gear_id = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                for dx in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        hotspots.insert(
                            Cell {
                                x: x as i32 + dx,
                                y: y as i32 + dy,
                            },
                            gear_id,
                        );
                    }
                }

                gear_id += 1;
            }
        }
    }

    let mut gears = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        let mut x = 0;

        while x < line.len() {
            let mut len = line.len() - x;

            'next_number: while len > 0 {
                if let Ok(num) = line[x..x + len].parse::<u32>() {
                    'search: for cell_x in x..x + len {
                        if let Some(gear_id) = hotspots.get(&Cell {
                            x: cell_x as i32,
                            y: y as i32,
                        }) {
                            gears.entry(gear_id).or_insert_with(|| vec![]).push(num);
                            break 'search;
                        }
                    }

                    x += len;
                    break 'next_number;
                } else {
                    len -= 1;
                }
            }

            x += 1;
        }
    }

    gears
        .values()
        .filter_map(|v| {
            if v.len() >= 2 {
                Some(v.iter().fold(1, |acc, x| acc * *x))
            } else {
                None
            }
        })
        .sum()
}
