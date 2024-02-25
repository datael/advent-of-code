use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let line = input.lines().next().unwrap();
    let route = line.chars().map(to_offset);

    gather_houses_on_route(route).len()
}

fn to_offset(c: char) -> (isize, isize) {
    match c {
        '^' => (0, 1),
        'v' => (0, -1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => unreachable!(),
    }
}

fn gather_houses_on_route(
    offsets: impl Iterator<Item = (isize, isize)>,
) -> HashSet<(isize, isize)> {
    offsets
        .fold(
            ((0, 0), HashSet::from([(0, 0)])),
            |(mut position, mut visited), next_offset| {
                position = (position.0 + next_offset.0, position.1 + next_offset.1);
                visited.insert(position);
                (position, visited)
            },
        )
        .1
}

fn solve_part2(input: &str) -> usize {
    let line = input.lines().next().unwrap();

    let santa_route =
        line.chars()
            .enumerate()
            .filter_map(|(i, c)| if i % 2 == 0 { Some(to_offset(c)) } else { None });

    let robot_route =
        line.chars()
            .enumerate()
            .filter_map(|(i, c)| if i % 2 == 1 { Some(to_offset(c)) } else { None });

    let santa_houses = gather_houses_on_route(santa_route);
    let robot_houses = gather_houses_on_route(robot_route);

    santa_houses.union(&robot_houses).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
