use std::{cmp::Ordering, collections::HashMap};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let (ordering_rules, update_pages) = input.split_once("\n\n").unwrap();

    let ordering_rules = OrderingRules::from(ordering_rules);
    let updates = update_pages.lines().map(Update::from).collect::<Vec<_>>();

    updates
        .iter()
        .filter(|update| update.is_in_order(&ordering_rules))
        .map(|in_order| in_order.0[in_order.0.len() / 2])
        .sum()
}

struct OrderingRules(HashMap<usize, Vec<usize>>);

impl From<&str> for OrderingRules {
    fn from(value: &str) -> Self {
        let rules = value
            .lines()
            .map(|line| line.split_once('|').unwrap())
            .map(|(left, right)| {
                (
                    left.parse::<usize>().unwrap(),
                    right.parse::<usize>().unwrap(),
                )
            });

        let mut rules_map = HashMap::<usize, Vec<usize>>::new();
        for rule in rules {
            rules_map.entry(rule.0).or_default().push(rule.1);
        }

        Self(rules_map)
    }
}

struct Update(Vec<usize>);

impl From<&str> for Update {
    fn from(value: &str) -> Self {
        let rules = value.split(",").flat_map(str::parse::<usize>).collect();

        Self(rules)
    }
}

impl Update {
    fn is_in_order(&self, ordering_rules: &OrderingRules) -> bool {
        for (i, page) in self.0.iter().skip(1).enumerate() {
            if let Some(must_not_be_after) = ordering_rules.0.get(page) {
                for prev_page in self.0[0..=i].iter() {
                    if must_not_be_after.contains(prev_page) {
                        return false;
                    }
                }
            }
        }

        true
    }
}

fn solve_part2(input: &str) -> usize {
    let (ordering_rules, update_pages) = input.split_once("\n\n").unwrap();

    let ordering_rules = OrderingRules::from(ordering_rules);
    let mut updates = update_pages.lines().map(Update::from).collect::<Vec<_>>();

    updates
        .iter_mut()
        .filter(|update| !update.is_in_order(&ordering_rules))
        .map(|not_in_order| {
            not_in_order.sort_with_rules(&ordering_rules);
            not_in_order.0[not_in_order.0.len() / 2]
        })
        .sum()
}

impl Update {
    fn sort_with_rules(&mut self, ordering_rules: &OrderingRules) {
        self.0.sort_by(|a, b| {
            if let Some(must_not_be_after) = ordering_rules.0.get(a) {
                if must_not_be_after.contains(b) {
                    return Ordering::Less;
                }
            }

            if let Some(must_not_be_after) = ordering_rules.0.get(b) {
                if must_not_be_after.contains(a) {
                    return Ordering::Greater;
                }
            }

            Ordering::Equal
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
            143,
        )] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [(
            r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
            123,
        )] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
