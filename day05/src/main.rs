#![feature(iter_array_chunks)]

use lib::read_all_lines_from_stdin;

// The expedition can depart as soon as the final supplies have been unloaded
// from the ships. Supplies are stored in stacks of marked crates, but because
// the needed supplies are buried under many other crates, the crates need to
// be rearranged.

#[derive(Debug)]
struct SupplyStorage {
    stacks: Vec<Vec<Crate>>,
}

struct StackId(usize);
#[derive(Debug)]
struct Crate(char);

// The ship has a giant cargo crane capable of moving crates between stacks.
// To ensure none of the crates get crushed or fall over, the crane operator
// will rearrange them in a series of carefully-planned steps. After the
// crates are rearranged, the desired crates will be at the top of each stack.

struct GiantCargoCrane;

impl GiantCargoCrane {
    fn move_crate_between_stacks(storage: &mut SupplyStorage, from: &StackId, to: &StackId) {
        let c = storage.stacks[from.0 - 1].pop().unwrap();
        storage.stacks[to.0 - 1].push(c)
    }
}

// The Elves don't want to interrupt the crane operator during this delicate
// procedure, but they forgot to ask her which crate will end up where, and
// they want to be ready to unload them as soon as possible so they can
// embark.

// They do, however, have a drawing of the starting stacks of crates and the
// rearrangement procedure (your puzzle input). For example:

//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3
//
// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2

impl<I> From<&I> for SupplyStorage
where
    I: IntoIterator<Item = String> + Clone,
{
    fn from(value: &I) -> Self {
        let is_crate_line = |line: &String| line.contains('[');

        let mut iter = value.clone().into_iter();

        let mut stacks_lines = vec![];
        let num_stacks;

        loop {
            let next = iter.next().unwrap();
            if is_crate_line(&next) {
                stacks_lines.push(next);
            } else {
                num_stacks = next.split_ascii_whitespace().count();
                break;
            }
        }

        let mut stacks: Vec<Vec<Crate>> = vec![];
        for _ in 0..num_stacks {
            stacks.push(vec![]);
        }

        stacks_lines.reverse();

        for line in stacks_lines {
            let mut iter = line.chars().array_chunks::<4>();
            let mut index = 0;

            for value in iter.by_ref() {
                if value[0] == '[' {
                    stacks[index].push(value.into())
                }

                index += 1;
            }

            if let Some(mut remainder) = iter.into_remainder() {
                if let Some(c) = remainder.next() {
                    if c == '[' {
                        stacks[index].push(remainder.next().unwrap().into())
                    }
                }
            }
        }

        Self { stacks }
    }
}

impl From<[char; 4]> for Crate {
    fn from(value: [char; 4]) -> Self {
        Self(value[1])
    }
}

impl From<char> for Crate {
    fn from(value: char) -> Self {
        Self(value)
    }
}

struct RearrangementCommand {
    count: usize,
    from: StackId,
    to: StackId,
}

impl From<&String> for RearrangementCommand {
    fn from(value: &String) -> Self {
        let mut iter = value.split(' ');
        iter.next(); // move
        let count = iter.next().unwrap().parse().unwrap();
        iter.next(); // from
        let from = iter.next().unwrap().into();
        iter.next(); // to
        let to = iter.next().unwrap().into();

        RearrangementCommand { count, from, to }
    }
}

impl From<&str> for StackId {
    fn from(value: &str) -> Self {
        Self(value.parse().unwrap())
    }
}

impl GiantCargoCrane {
    fn apply(command: &RearrangementCommand, storage: &mut SupplyStorage) {
        for _ in 0..command.count {
            Self::move_crate_between_stacks(storage, &command.from, &command.to);
        }
    }
}

// In this example, there are three stacks of crates. Stack 1 contains two
// crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains
// three crates; from bottom to top, they are crates M, C, and D. Finally,
// stack 3 contains a single crate, P.

// Then, the rearrangement procedure is given. In each step of the procedure,
// a quantity of crates is moved from one stack to a different stack. In the
// first step of the above rearrangement procedure, one crate is moved from
// stack 2 to stack 1, resulting in this configuration:

// [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

// Then, both crates are moved from stack 2 to stack 1. Again, because crates
// are moved one at a time, crate C ends up below crate M:

//         [Z]
//         [N]
// [M]     [D]
// [C]     [P]
//  1   2   3

// Finally, one crate is moved from stack 1 to stack 2:

//         [Z]
//         [N]
//         [D]
// [C] [M] [P]
//  1   2   3

// The Elves just need to know which crate will end up on top of each stack;
// in this example, the top crates are C in stack 1, M in stack 2, and Z in
// stack 3, so you should combine these together and give the Elves the
// message CMZ.

impl SupplyStorage {
    fn top_of_each_stack(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.last())
            .map(|c| c.0)
            .collect()
    }
}

fn main() {
    let input: Vec<_> = read_all_lines_from_stdin().into_iter().collect();

    // After the rearrangement procedure completes, what crate ends up on top of
    // each stack?

    let mut supply_storage = SupplyStorage::from(&input);

    input
        .iter()
        .skip_while(|line| !line.starts_with("move"))
        .map(RearrangementCommand::from)
        .for_each(|command| GiantCargoCrane::apply(&command, &mut supply_storage));

    println!(
        "Crates at the top of each stack: {}",
        supply_storage.top_of_each_stack()
    );
}
