use itertools::Itertools;
use std::collections::HashSet;

use lib::read_all_lines_from_stdin;

// One Elf has the important job of loading all of the rucksacks with supplies
// for the jungle journey. Unfortunately, that Elf didn't quite follow the
// packing instructions, and so a few items now need to be rearranged.

// Each rucksack has two large compartments. All items of a given type are
// meant to go into exactly one of the two compartments. The Elf that did the
// packing failed to follow this rule for exactly one item type per rucksack.

struct Rucksack(HashSet<char>, HashSet<char>);

// The Elves have made a list of all of the items currently in each rucksack
// (your puzzle input), but they need your help finding the errors. Every item
// type is identified by a single lowercase or uppercase letter (that is, a
// and A refer to different types of items).

impl Rucksack {
    fn find_duplicate(self) -> char {
        self.0
            .intersection(&self.1)
            .into_iter()
            .next()
            .unwrap()
            .to_owned()
    }

    // The list of items for each rucksack is given as characters all on a single
    // line. A given rucksack always has the same number of items in each of its
    // two compartments, so the first half of the characters represent items in
    // the first compartment, while the second half of the characters represent
    // items in the second compartment.

    fn fill(input: &String) -> Self {
        let compartments = input.split_at(input.len() / 2);

        Rucksack(
            compartments.0.chars().collect(),
            compartments.1.chars().collect(),
        )
    }
}

// To help prioritize item rearrangement, every item type can be converted to
// a priority:
// - Lowercase item types a through z have priorities 1 through 26.
// - Uppercase item types A through Z have priorities 27 through 52.

fn to_priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'A'..='Z' => c as i32 - 'A' as i32 + 27,
        _ => panic!("Invalid input"),
    }
}

// As you finish identifying the misplaced items, the Elves come to you with
// another issue.

// For safety, the Elves are divided into groups of three. Every Elf carries a
// badge that identifies their group. For efficiency, within each group of
// three Elves, the badge is the only item type carried by all three Elves.
// That is, if a group's badge is item type B, then all three Elves will have
// item type B somewhere in their rucksack, and at most two of the Elves will
// be carrying any other item type.

// The problem is that someone forgot to put this year's updated authenticity
// sticker on the badges. All of the badges need to be pulled out of the
// rucksacks so the new authenticity stickers can be attached.

// Additionally, nobody wrote down which item type corresponds to each group's
// badges. The only way to tell which item type is the right one is by finding
// the one item type that is common between all three Elves in each group.

// Every set of three lines in your list corresponds to a single group, but
// each group can have a different badge item type. So, in the above example,
// the first group's rucksacks are the first three lines:

// vJrwpWtwJgWrhcsFMMfFFhFp
// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
// PmmdzqPrVvPwwTWBwg

// And the second group's rucksacks are the next three lines:

// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
// ttgJtRGJQctTZtZT
// CrZsJsPPZsGzwwsLwLmpwMDw

// In the first group, the only item type that appears in all three rucksacks
// is lowercase r; this must be their badges. In the second group, their badge
// item type must be Z.

fn find_common_item<I: IntoIterator<Item = Rucksack>>(rucksacks: I) -> char {
    // union of left and right compartments gives all items in each rucksack
    let rucksack_items: Vec<HashSet<char>> = rucksacks
        .into_iter()
        .map(|r| r.0.union(&r.1).copied().collect())
        .collect();

    // reduce with hashset intersections to find the one item common in all rucksacks
    rucksack_items
        .iter()
        .skip(1)
        .fold(rucksack_items[0].clone(), |a, b| {
            a.intersection(&b).copied().collect()
        })
        .iter()
        .copied()
        .next()
        .unwrap()
}

fn main() {
    let input: Vec<_> = read_all_lines_from_stdin().into_iter().collect();

    let part1_sum = input
        .iter()
        .map(Rucksack::fill)
        .map(Rucksack::find_duplicate)
        .map(to_priority)
        .sum::<i32>();

    println!("Part 1 sum of priorities: {}", part1_sum);

    // Priorities for these items must still be found to organize the sticker
    // attachment efforts: here, they are 18 (r) for the first group and 52 (Z)
    // for the second group. The sum of these is 70.

    let part2_sum = input
        .iter()
        .map(Rucksack::fill)
        .chunks(3)
        .into_iter()
        .map(find_common_item)
        .map(to_priority)
        .sum::<i32>();

    println!("Part 2 sum of priorities: {}", part2_sum);
}
