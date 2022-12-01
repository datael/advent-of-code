use std::io::*;

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

#[allow(dead_code)]
fn read_elf_input_take_while(all_lines: Vec<String>) -> Vec<Elf> {
    let mut lines = all_lines.iter().peekable();
    let mut elves = vec![];

    while lines.peek() != None {
        elves.push(Elf {
            inventory: Inventory {
                items: lines
                    .by_ref()
                    .take_while(|line| **line != "")
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
fn read_elf_input(all_lines: Vec<String>) -> Vec<Elf> {
    let mut lines = all_lines.iter();

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

fn read_all_lines<R: BufRead>(reader: R) -> Vec<String> {
    reader.lines().flatten().collect()
}

fn main() {
    let elves = read_elf_input(read_all_lines(stdin().lock()));

    // Part 1
    let total_calories: Vec<_> = elves
        .iter()
        .map(|elf| elf.inventory.total_calories())
        .collect();

    let max_calories = total_calories.iter().max();
    println!("Max calories carried by any elf: {}", max_calories.unwrap());

    // Part 2
    let mut sorted_total_calories = total_calories;
    sorted_total_calories.sort_by(|a, b| b.cmp(a));

    let top_3_calories_total: i32 = sorted_total_calories.iter().take(3).sum();
    println!(
        "Total calories carried by the 3 elves with the most calories: {}",
        top_3_calories_total
    );
}
