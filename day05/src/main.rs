use std::{
    cmp::Ordering,
    collections::HashMap,
    ops::{Deref, DerefMut},
};

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
        .filter(|update| ordering_rules.is_sorted(update))
        .map(|in_order| in_order.get_middle_page().0)
        .sum()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Page(usize);

impl From<&str> for Page {
    fn from(value: &str) -> Self {
        Page(value.parse::<usize>().unwrap())
    }
}

struct OrderingRules(HashMap<Page, Vec<Page>>);

impl From<&str> for OrderingRules {
    fn from(value: &str) -> Self {
        let rules = value
            .lines()
            .map(|line| line.split_once('|').unwrap())
            .map(|(left, right)| (Page::from(left), Page::from(right)))
            .fold(
                HashMap::<Page, Vec<Page>>::new(),
                |mut grouped_rules, rule| {
                    grouped_rules.entry(rule.0).or_default().push(rule.1);
                    grouped_rules
                },
            );

        Self(rules)
    }
}

impl OrderingRules {
    fn is_sorted(&self, pages: &[Page]) -> bool {
        pages.is_sorted_by(|a, b| self.compare(a, b) != Ordering::Greater)
    }

    fn sort(&self, pages: &mut [Page]) {
        pages.sort_by(|a, b| self.compare(a, b));
    }

    fn compare(&self, a: &Page, b: &Page) -> Ordering {
        if let Some(must_not_be_after) = self.0.get(a) {
            if must_not_be_after.contains(b) {
                return Ordering::Less;
            }
        }

        if let Some(must_not_be_after) = self.0.get(b) {
            if must_not_be_after.contains(a) {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    }
}

struct Update(Vec<Page>);

impl From<&str> for Update {
    fn from(value: &str) -> Self {
        let rules = value.split(",").map(Page::from).collect();

        Self(rules)
    }
}

impl Deref for Update {
    type Target = Vec<Page>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Update {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Update {
    fn get_middle_page(&self) -> &Page {
        &self[self.len() / 2]
    }
}

fn solve_part2(input: &str) -> usize {
    let (ordering_rules, update_pages) = input.split_once("\n\n").unwrap();

    let ordering_rules = OrderingRules::from(ordering_rules);
    let mut updates = update_pages.lines().map(Update::from).collect::<Vec<_>>();

    updates
        .iter_mut()
        .filter(|update| !ordering_rules.is_sorted(update))
        .map(|not_in_order| {
            ordering_rules.sort(not_in_order);
            not_in_order.get_middle_page().0
        })
        .sum()
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
