use std::collections::HashSet;

use lib::read_all_lines_from_stdin;

// One Elf has the important job of loading all of the rucksacks with supplies
// for the jungle journey. Unfortunately, that Elf didn't quite follow the
// packing instructions, and so a few items now need to be rearranged.

// Each rucksack has two large compartments. All items of a given type are
// meant to go into exactly one of the two compartments. The Elf that did the
// packing failed to follow this rule for exactly one item type per rucksack.

struct Rucksack {
    left_compartment: HashSet<char>,
    right_compartment: HashSet<char>,
}

// The Elves have made a list of all of the items currently in each rucksack
// (your puzzle input), but they need your help finding the errors. Every item
// type is identified by a single lowercase or uppercase letter (that is, a
// and A refer to different types of items).

impl Rucksack {
    fn find_duplicate(self) -> char {
        self.left_compartment
            .intersection(&self.right_compartment)
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

    fn fill(input: String) -> Self {
        let (left, right) = input.split_at(input.len() / 2);

        Rucksack {
            left_compartment: left.chars().collect(),
            right_compartment: right.chars().collect(),
        }
    }
}

// To help prioritize item rearrangement, every item type can be converted to
// a priority:
// - Lowercase item types a through z have priorities 1 through 26.
// - Uppercase item types A through Z have priorities 27 through 52.

fn to_priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 96,
        'A'..='Z' => c as i32 - 38,
        _ => panic!("Invalid input"),
    }
}

fn main() {
    let part1_sum = read_all_lines_from_stdin()
        .into_iter()
        .map(Rucksack::fill)
        .map(Rucksack::find_duplicate)
        .map(to_priority)
        .sum::<i32>();

    println!("Sum of priorities: {}", part1_sum);
}
