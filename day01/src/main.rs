use itertools::Itertools;
use lib::*;

// The jungle must be too overgrown and difficult to navigate in vehicles or
// access from the air; the Elves' expedition traditionally goes on foot. As
// your boats approach land, the Elves begin taking inventory of their
// supplies. One important consideration is food - in particular, the number
// of Calories each Elf is carrying (your puzzle input).

struct Elf {
    pub inventory: Inventory,
}

impl Elf {
    fn new() -> Self {
        Elf {
            inventory: Inventory { items: vec![] },
        }
    }
}

struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    fn total_calories(&self) -> i32 {
        self.items.iter().map(|item| item.calories).sum()
    }
}

struct Item {
    pub calories: i32,
}

// The Elves take turns writing down the number of Calories contained by the
// various meals, snacks, rations, etc. that they've brought with them, one
// item per line. Each Elf separates their own inventory from the previous
// Elf's inventory (if any) by a blank line.

#[allow(dead_code)]
fn read_elf_input_group_by<I: IntoIterator<Item = String>>(all_lines: I) -> Vec<Elf> {
    all_lines
        .into_iter()
        .group_by(|line| *line != "")
        .into_iter()
        .map(|elf_lines| Elf {
            inventory: Inventory {
                items: elf_lines
                    .1
                    .map(|line| line.parse())
                    .flatten()
                    .map(|calories| Item { calories })
                    .collect(),
            },
        })
        .collect()
}

#[allow(dead_code)]
fn read_elf_input_take_while<I: IntoIterator<Item = String>>(all_lines: I) -> Vec<Elf> {
    let mut lines = all_lines.into_iter().peekable();
    let mut elves = vec![];

    while lines.peek() != None {
        elves.push(Elf {
            inventory: Inventory {
                items: lines
                    .by_ref()
                    .take_while(|line| *line != "")
                    .map(|line| line.parse())
                    .flatten()
                    .map(|calories| Item { calories })
                    .collect(),
            },
        })
    }

    elves
}

#[allow(dead_code)]
fn read_elf_input<I: IntoIterator<Item = String>>(all_lines: I) -> Vec<Elf> {
    let mut lines = all_lines.into_iter();

    let mut elves = vec![];
    let mut next_elf = Elf::new();

    while let Some(line) = lines.next() {
        if line == "" {
            elves.push(next_elf);
            next_elf = Elf::new();
        } else {
            next_elf.inventory.items.push(Item {
                calories: line.parse().unwrap(),
            })
        }
    }

    elves.push(next_elf);

    elves
}

// 1000
// 2000
// 3000

// 4000

// 5000
// 6000

// 7000
// 8000
// 9000

// 10000

// In case the Elves get hungry and need extra snacks, they need to know which
// Elf to ask: they'd like to know how many Calories are being carried by the
// Elf carrying the most Calories. In the example above, this is 24000
// (carried by the fourth Elf).

fn main() {
    let elves = read_elf_input_group_by(read_all_lines_from_stdin());
    // let elves = read_elf_input_take_while(read_all_lines_from_stdin());
    // let elves = read_elf_input(read_all_lines(stdin().lock()));

    // Find the Elf carrying the most Calories. How many total Calories is that
    // Elf carrying?
    let total_calories: Vec<_> = elves
        .iter()
        .map(|elf| elf.inventory.total_calories())
        .collect();

    let max_calories = total_calories.iter().max();
    println!("Max calories carried by any elf: {}", max_calories.unwrap());

    // By the time you calculate the answer to the Elves' question, they've
    // already realized that the Elf carrying the most Calories of food might
    // eventually run out of snacks.

    // To avoid this unacceptable situation, the Elves would instead like to know
    // the total Calories carried by the top three Elves carrying the most
    // Calories. That way, even if one of those Elves runs out of snacks, they
    // still have two backups.

    // In the example above, the top three Elves are the fourth Elf (with 24000
    // Calories), then the third Elf (with 11000 Calories), then the fifth Elf
    // (with 10000 Calories). The sum of the Calories carried by these three elves
    // is 45000.

    // Find the top three Elves carrying the most Calories. How many Calories are
    // those Elves carrying in total?

    let mut sorted_total_calories = total_calories;
    sorted_total_calories.sort_by(|a, b| b.cmp(a));

    let top_3_calories_total: i32 = sorted_total_calories.iter().take(3).sum();
    println!(
        "Total calories carried by the 3 elves with the most calories: {}",
        top_3_calories_total
    );
}
