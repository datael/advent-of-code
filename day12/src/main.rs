use std::collections::HashMap;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 21);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 525152);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let lines = input.lines();

    lines
        .map(parse_record)
        .map(|record| count_combinations(record, &mut Default::default()))
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let lines = input.lines();

    lines
        .map(parse_record)
        .map(|record| expand_record_for_part2(record, 5))
        .map(|record| count_combinations(record, &mut Default::default()))
        .sum()
}

fn parse_record(line: &str) -> ConditionRecord {
    let (springs, damaged_groups) = line.split_once(' ').unwrap();

    let springs = springs.chars().map(|c| c.into()).collect();
    let damaged_groups = damaged_groups
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    ConditionRecord {
        springs,
        damaged_groups,
    }
}

fn count_combinations(
    record: ConditionRecord,
    memo: &mut HashMap<ConditionRecord, usize>, // Need to memoize otherwise it takes absolutely forever
) -> usize {
    if let Some(&count) = memo.get(&record) {
        count
    } else {
        let key = record.clone();

        let res = (|| {
            // Get the next damaged group
            // If there are no more remaining, then as long as the remaining springs
            // are all operational, or unknown, then we have a valid combination.
            let Some(_) = record.damaged_groups.first() else {
                if record
                    .springs
                    .iter()
                    .any(|spring| *spring == Spring::Damaged)
                {
                    return 0;
                } else {
                    return 1;
                }
            };

            // Get the next spring
            // If we have run out of springs, then this combination is not valid
            let Some(next_spring) = record.springs.first() else {
                return 0;
            };

            // Since we haven't run out, we can move onto the next spring
            return match next_spring {
                Spring::Operational => count_combinations(
                    ConditionRecord {
                        springs: record.springs[1..].into(),
                        damaged_groups: record.damaged_groups,
                    },
                    memo,
                ),
                Spring::Damaged => count_with_damaged(record, memo),
                // If the next spring is unknown, take the sum of either possibility's
                // combinations
                Spring::Unknown => {
                    count_combinations(
                        ConditionRecord {
                            springs: record.springs[1..].into(),
                            damaged_groups: record.damaged_groups.clone(),
                        },
                        memo,
                    ) + count_with_damaged(record, memo)
                }
            };
        })();

        memo.insert(key, res);

        res
    }
}

fn count_with_damaged(
    record: ConditionRecord,
    memo: &mut HashMap<ConditionRecord, usize>,
) -> usize {
    let damaged_length = record.damaged_groups[0];

    // If we go over the end, this isn't a valid match
    if record.springs.len() < damaged_length {
        return 0;
    }

    // Check all springs for this damaged group's run length:
    // if any are supposed to be operational then this combination isn't valid
    if record
        .springs
        .iter()
        .take(damaged_length)
        .any(|spring| *spring == Spring::Operational)
    {
        return 0;
    }

    // If the length is exact, and have no more remaining groups, then
    // this is a valid combination
    if record.springs.len() == damaged_length {
        if record.damaged_groups.len() == 1 {
            return 1;
        } else {
            return 0;
        }
    }

    match record.springs[damaged_length] {
        // If we're expecting a damaged one next then this group was an invalid
        // combination
        Spring::Damaged => 0,
        // Otherwise, check the remainder
        _ => count_combinations(
            ConditionRecord {
                springs: record.springs[damaged_length + 1..].into(),
                damaged_groups: record.damaged_groups[1..].into(),
            },
            memo,
        ),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ConditionRecord {
    springs: Vec<Spring>,
    damaged_groups: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => unreachable!(),
        }
    }
}

fn expand_record_for_part2(record: ConditionRecord, repetitions: usize) -> ConditionRecord {
    let mut springs = Vec::with_capacity(repetitions * record.springs.len() + repetitions - 1);

    for _ in 0..repetitions - 1 {
        for spring in record.springs.iter() {
            springs.push(*spring);
        }
        springs.push(Spring::Unknown);
    }

    for spring in record.springs.iter() {
        springs.push(*spring);
    }

    let damaged_groups = record.damaged_groups.repeat(5);

    ConditionRecord {
        springs,
        damaged_groups,
    }
}
