const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    // The rules are as follows:
    // - Initial velocity is equal to the time we accelerate
    //     for since we accelerate at a rate of one unit per second.
    //     - There is no mention of slowing or accelerating from zero,
    //         so we can assume instantaneous acceleration and a constant
    //         velocity once we have started moving.
    // - Duration spent traveling is equal to the total time minus the
    //     time we spent accelerating.
    //
    // Or as formulas:
    //   v = acc_time
    //   d = v * t
    //   d = v * (time - acc_time)
    //   d = acc_time * (time - acc_time)
    //
    // We rearrange to form a quadratic equation:
    //   d = acc_time * (time - acc_time)
    //   d = acc_time * time - acc_time^2
    //   acc_time^2 - time * acc_time + d = 0
    //
    // Our inputs are time and target distance, so we can solve for acc_time
    //   using the quadratic formula:
    //
    //   x = (-b +- sqrt(b^2 - 4ac)) / 2a
    //
    // Where:
    //
    //   a = 1
    //   b = -time
    //   c = (target) distance
    //   x = time spent accelerating
    //
    // If we find the two solutions, we will have found the two times where
    //   the boat perfectly matches the target distance.
    // We can then base our answer on the rounded values of those, where we round the lower
    //   value up and the higher value down.

    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 288);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 71503);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> u64 {
    let time_distance_pairs = build_time_distance_pairs(&mut input.lines());

    time_distance_pairs
        .map(find_range)
        .fold(1, |acc, r| acc * r)
}

fn build_time_distance_pairs<'a, Lines: Iterator<Item = &'a str>>(
    mut lines: Lines,
) -> impl Iterator<Item = (u64, u64)> + 'a {
    let (_, times) = lines.next().unwrap().split_once(":").unwrap();
    let (_, distances) = lines.next().unwrap().split_once(":").unwrap();

    let times = times.split(" ").flat_map(|time| time.trim().parse().ok());

    let distances = distances
        .split(" ")
        .flat_map(|time| time.trim().parse().ok());

    times.zip(distances)
}

fn find_range((time, record_distance): (u64, u64)) -> u64 {
    // x = (-b +- sqrt(b^2 - 4ac)) / 2a
    //
    // a = 1
    // b = -time
    // c = (target) distance
    // x = (-time +- sqrt((-time)^2 - 4*distance)) / 2
    // x = (-time +- sqrt(time^2 - 4*distance)) / 2

    let time = time as f64;
    let distance = (record_distance + 1) as f64;

    let acc1 = (time + ((time * time) - 4.0 * distance).sqrt()) / 2.0;
    let acc2 = (time - ((time * time) - 4.0 * distance).sqrt()) / 2.0;

    let (acc1, acc2) = if acc1 < acc2 {
        (acc1, acc2)
    } else {
        (acc2, acc1)
    };

    let acc1 = acc1.ceil() as u64;
    let acc2 = acc2.floor() as u64;

    1 + acc2 - acc1
}

fn solve_part2(input: &str) -> u64 {
    let time_distance_pair = build_time_distance_pair(&mut input.lines());

    find_range(time_distance_pair)
}

fn build_time_distance_pair<'a, Lines: Iterator<Item = &'a str>>(mut lines: Lines) -> (u64, u64) {
    let (_, times) = lines.next().unwrap().split_once(":").unwrap();
    let (_, distances) = lines.next().unwrap().split_once(":").unwrap();

    (
        times
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .collect::<String>()
            .parse()
            .unwrap(),
        distances
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .collect::<String>()
            .parse()
            .unwrap(),
    )
}
