const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 1320);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 145);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let (input, _) = input.split_once('\n').unwrap();

    input
        .split(',')
        .map(calculate_hash)
        .map(|hash| hash as usize)
        .sum()
}

fn calculate_hash(input: &str) -> u8 {
    input
        .chars()
        .map(|c| c as u8)
        .fold(0_u8, |agg, c| agg.wrapping_add(c).wrapping_mul(17))
}

fn solve_part2(input: &str) -> usize {
    let (input, _) = input.split_once('\n').unwrap();

    input
        .split(',')
        .map(parse_command)
        .fold(vec![vec![]; 256], apply_command)
        .iter()
        .enumerate()
        .map(|(i, bucket)| {
            (i + 1)
                * bucket
                    .iter()
                    .enumerate()
                    .map(|(j, (_, focal_length))| (j + 1) * (*focal_length as usize))
                    .sum::<usize>()
        })
        .sum()
}

fn parse_command<'a>(input: &'a str) -> Command<'a> {
    let label_length = input.chars().position(|c| c == '=' || c == '-').unwrap();

    let label = &input[0..label_length];
    let label_hash = calculate_hash(&label);

    match input.chars().nth(label_length).unwrap() {
        '-' => Command::Remove(label_hash, label),
        '=' => Command::Append(
            label_hash,
            label,
            input[label_length + 1..].parse().unwrap(),
        ),
        _ => unreachable!(),
    }
}

enum Command<'a> {
    Remove(u8, &'a str),
    Append(u8, &'a str, u8),
}

fn apply_command<'a>(
    hashmap: Vec<Vec<(&'a str, u8)>>,
    command: Command<'a>,
) -> Vec<Vec<(&'a str, u8)>> {
    let mut hashmap = hashmap;

    match command {
        Command::Remove(label_hash, label) => {
            let bucket = &mut hashmap[label_hash as usize];

            if let Some(index) = bucket
                .iter()
                .position(|(this_label, _)| this_label == &label)
            {
                bucket.remove(index);
            }
        }
        Command::Append(label_hash, label, focal_length) => {
            let bucket = &mut hashmap[label_hash as usize];

            if let Some(entry) = bucket
                .iter_mut()
                .find(|(this_label, _)| this_label == &label)
            {
                entry.1 = focal_length;
            } else {
                bucket.push((label, focal_length));
            }
        }
    }

    hashmap
}
