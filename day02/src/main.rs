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
        .map(parse_dimensions)
        .map(required_paper_for_dimensions)
        .sum()
}

fn parse_dimensions(line: &str) -> [usize; 3] {
    let (x, rest) = line.split_once('x').unwrap();
    let (y, z) = rest.split_once('x').unwrap();

    let x = x.parse().unwrap();
    let y = y.parse().unwrap();
    let z = z.parse().unwrap();

    [x, y, z]
}

fn required_paper_for_dimensions([x, y, z]: [usize; 3]) -> usize {
    let sides = [x * y, y * z, z * x];
    let smallest_side = sides.iter().min().unwrap();

    2 * sides.iter().sum::<usize>() + smallest_side
}

fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_dimensions)
        .map(required_total_ribbon_for_dimensions)
        .sum()
}

fn required_total_ribbon_for_dimensions(dimensions: [usize; 3]) -> usize {
    required_ribbon_wrap_for_dimensions(dimensions) + required_ribbon_bow_for_dimensions(dimensions)
}

fn required_ribbon_wrap_for_dimensions(dimensions: [usize; 3]) -> usize {
    match dimensions
        .iter()
        .position(|d| d == dimensions.iter().max().unwrap())
        .unwrap()
    {
        0 => 2 * (dimensions[1] + dimensions[2]),
        1 => 2 * (dimensions[0] + dimensions[2]),
        2 => 2 * (dimensions[0] + dimensions[1]),
        _ => unreachable!(),
    }
}

fn required_ribbon_bow_for_dimensions(dimensions: [usize; 3]) -> usize {
    dimensions.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [("2x3x4", 58), ("1x1x10", 43)] {
            assert_eq!(
                required_paper_for_dimensions(parse_dimensions(input)),
                expected
            );
        }
    }

    #[test]
    fn test_part2_wrap() {
        for (input, expected) in [("2x3x4", 10), ("1x1x10", 4)] {
            assert_eq!(
                required_ribbon_wrap_for_dimensions(parse_dimensions(input)),
                expected
            );
        }
    }

    #[test]
    fn test_part2_bow() {
        for (input, expected) in [("2x3x4", 24), ("1x1x10", 10)] {
            assert_eq!(
                required_ribbon_bow_for_dimensions(parse_dimensions(input)),
                expected
            );
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [("2x3x4", 34), ("1x1x10", 14)] {
            assert_eq!(
                required_total_ribbon_for_dimensions(parse_dimensions(input)),
                expected
            );
        }
    }
}
