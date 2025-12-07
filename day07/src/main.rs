use std::{iter, time::Instant};

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
    let mut lines = input.lines();

    let top_row = lines.next().expect("we have at least one line");
    let width = top_row.len();

    let start_index = top_row
        .chars()
        .position(|c| c == 'S')
        .expect("first row has an S");

    let mut num_splits = 0;

    let mut beam_columns = iter::repeat_n(false, width).collect::<Vec<_>>();
    let mut next_beam_columns = beam_columns.clone();

    beam_columns[start_index] = true;

    for line in lines {
        for i in 0..width {
            if beam_columns[i] {
                if line.chars().nth(i) == Some('^') {
                    num_splits += 1;

                    if i > 0 {
                        next_beam_columns[i - 1] = true;
                    }
                    if i < width - 1 {
                        next_beam_columns[i + 1] = true;
                    }

                    next_beam_columns[i] = false;
                } else {
                    next_beam_columns[i] = true;
                }
            }
        }

        std::mem::swap(&mut beam_columns, &mut next_beam_columns);

        for c in next_beam_columns.iter_mut() {
            *c = false;
        }
    }

    num_splits
}

#[inline(never)]
fn solve_part2(input: &str) -> usize {
    let mut lines = input.lines();

    let top_row = lines.next().expect("we have at least one line");
    let width = top_row.len();

    let start_index = top_row
        .chars()
        .position(|c| c == 'S')
        .expect("first row has an S");

    let mut beam_columns = iter::repeat_n(0usize, width).collect::<Vec<_>>();
    let mut next_beam_columns = beam_columns.clone();

    beam_columns[start_index] = 1;

    for line in lines {
        for i in 0..width {
            if beam_columns[i] > 0 {
                if line.chars().nth(i) == Some('^') {
                    if i > 0 {
                        next_beam_columns[i - 1] += beam_columns[i];
                    }
                    if i < width - 1 {
                        next_beam_columns[i + 1] += beam_columns[i];
                    }
                } else {
                    next_beam_columns[i] += beam_columns[i];
                }
            }
        }

        std::mem::swap(&mut beam_columns, &mut next_beam_columns);

        for c in next_beam_columns.iter_mut() {
            *c = 0;
        }
    }

    beam_columns.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

        assert_eq!(solve_part1(input), 21);
    }

    #[test]
    fn test_input_part2() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

        assert_eq!(solve_part2(input), 40);
    }
}
