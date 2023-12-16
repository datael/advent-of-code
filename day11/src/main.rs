use std::iter;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 374);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve::<9>(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 1030);

    let test_result = solve::<99>(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 8410);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    solve::<1>(&input)
}

fn solve<const EXPANSION: usize>(input: &str) -> usize {
    let (image, (empty_xs, empty_ys)) = generate_image(input);

    let image = expand(&image, (&empty_xs, &empty_ys), EXPANSION);

    sum_distance_pairs(&image)
}

fn generate_image(input: &str) -> (Vec<(usize, usize)>, (Vec<usize>, Vec<usize>)) {
    let mut image = Vec::new();
    let mut max = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            max = (x, y).max(max);

            if c == '#' {
                image.push((x, y));
            }
        }
    }

    let empty_xs: Vec<_> = (0..=max.0)
        .filter(|x| image.iter().all(|(xx, _)| *xx != *x))
        .collect();

    let empty_ys: Vec<_> = (0..=max.1)
        .filter(|y| image.iter().all(|(_, yy)| *yy != *y))
        .collect();

    (
        image.iter().map(|(x, y)| (*x, *y)).collect(),
        (empty_xs, empty_ys),
    )
}

fn expand(
    image: &Vec<(usize, usize)>,
    empties: (&Vec<usize>, &Vec<usize>),
    amount: usize,
) -> Vec<(usize, usize)> {
    let (empty_xs, empty_ys) = empties;

    image
        .iter()
        .map(|(x, y)| {
            (
                *x + empty_xs.iter().filter(|xx| **xx < *x).count() * amount,
                *y + empty_ys.iter().filter(|yy| **yy < *y).count() * amount,
            )
        })
        .collect()
}

fn sum_distance_pairs(image: &Vec<(usize, usize)>) -> usize {
    image
        .iter()
        .enumerate()
        .map(|(i, a)| image.iter().skip(i + 1).zip(iter::repeat(*a)))
        .flatten()
        .map(|(a, b)| ((a.0.max(b.0) - a.0.min(b.0)) + (a.1.max(b.1) - a.1.min(b.1))))
        .sum()
}

fn solve_part2(input: &str) -> usize {
    solve::<999_999>(&input)
}
