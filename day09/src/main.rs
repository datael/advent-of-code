const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 114);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 2);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> i32 {
    let lines = input.lines();

    lines.map(extrapolate_forwards_next_value).sum()
}

fn extrapolate_forwards_next_value(line: &str) -> i32 {
    let mut nums = line
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut rows = Vec::new();
    loop {
        rows.push(nums);
        nums = get_diffs(&rows.last().unwrap());

        if nums.len() == 1 {
            panic!("No solution found for line: {}", line);
        }

        if nums.iter().all(|n| *n == 0) {
            rows.push(nums);
            break;
        }
    }

    rows.last_mut().unwrap().push(0);

    loop {
        let last_row = rows.pop().unwrap();
        let diff = last_row.last().unwrap();

        let next_row = rows.last_mut().unwrap();
        next_row.push(*diff + next_row.last().unwrap());

        if rows.len() == 1 {
            return *rows[0].last().unwrap();
        }
    }
}

fn get_diffs(nums: &[i32]) -> Vec<i32> {
    let mut diffs = Vec::with_capacity(nums.len());

    for i in 0..nums.len() - 1 {
        diffs.push(nums[i + 1] - nums[i]);
    }

    diffs
}

fn solve_part2(input: &str) -> i32 {
    let lines = input.lines();

    lines.map(extrapolate_backwards_next_value).sum()
}

fn extrapolate_backwards_next_value(line: &str) -> i32 {
    let mut nums = line
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut rows = Vec::new();
    loop {
        rows.push(nums);
        nums = get_diffs(&rows.last().unwrap());

        if nums.len() == 1 {
            panic!("No solution found for line: {}", line);
        }

        if nums.iter().all(|n| *n == 0) {
            rows.push(nums);
            break;
        }
    }

    rows.last_mut().unwrap().push(0);

    loop {
        let last_row = rows.pop().unwrap();
        let diff = *last_row.first().unwrap();

        let next_row = rows.last_mut().unwrap();
        next_row.reverse();
        next_row.push(next_row.last().unwrap() - diff);
        next_row.reverse();

        if rows.len() == 1 {
            return *rows[0].first().unwrap();
        }
    }
}
