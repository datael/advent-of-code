use std::cmp::Ordering;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(Report::from)
        .filter(Report::is_safe)
        .count()
}

struct Report {
    levels: Vec<u32>,
}

impl<S> From<S> for Report
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        Self {
            levels: value.as_ref().split(' ').flat_map(str::parse).collect(),
        }
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut known_cmp = Ordering::Equal;

        let adjacent_level_pairs = self.levels.iter().zip(self.levels[1..].iter());

        for (a, b) in adjacent_level_pairs {
            // Any two adjacent levels differ by at least one and at most three.
            if a.abs_diff(*b) < 1 || 3 < a.abs_diff(*b) {
                return false;
            }

            // The levels are either all increasing or all decreasing.
            match a.cmp(b) {
                Ordering::Equal => {
                    return false;
                }
                Ordering::Greater if known_cmp == Ordering::Less => {
                    return false;
                }
                Ordering::Less if known_cmp == Ordering::Greater => {
                    return false;
                }
                cmp => known_cmp = cmp,
            }
        }

        true
    }
}

fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(Report::from)
        .filter(Report::is_safe_allow_removal)
        .count()
}

impl Report {
    fn is_safe_allow_removal(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        // Try all combinations of removing 1 level from this report
        for i in 0..self.levels.len() {
            let (left, right) = self.levels.split_at(i);
            let adjusted_levels = [left, &right[1..]].concat();

            let adjusted_report = Report {
                levels: adjusted_levels,
            };

            if adjusted_report.is_safe() {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
            2,
        )] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [(
            r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
            4,
        )] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
