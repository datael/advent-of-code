use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    iter,
    ops::{Add, Sub},
};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 5);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 7);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let mut initial_state = parse_initial_state(input).collect::<Vec<_>>();
    initial_state.sort_by(|a, b| a.get_min_position().z.cmp(&b.get_min_position().z));

    let (supporting, supported_by) = build_support_map(initial_state.as_slice());

    let mut can_remove = 0;
    'next: for i in 0..initial_state.len() {
        if let Some(s) = supporting.get(&i) {
            if s.is_empty() {
                can_remove += 1;
                continue;
            }

            for other_brick_index in s.iter() {
                if let Some(s) = supported_by.get(other_brick_index) {
                    // we're the only support
                    if s.len() == 1 {
                        continue 'next;
                    }
                }
            }
        }

        can_remove += 1;
    }

    can_remove
}

fn parse_initial_state(input: &str) -> impl Iterator<Item = (Position, Brick)> + '_ {
    input.lines().map(|line| {
        let (from, to) = line.split_once('~').unwrap();
        let from = Position::from(from);
        let to = Position::from(to);

        let origin = Offset {
            x: from.x.min(to.x),
            y: from.y.min(to.y),
            z: from.z.min(to.z),
        };

        let from = from - origin;
        let to = to - origin;

        let origin = origin.as_position();

        let width = (from.x - to.x).unsigned_abs() as usize + 1;
        let depth = (from.y - to.y).unsigned_abs() as usize + 1;
        let height = (from.z - to.z).unsigned_abs() as usize + 1;

        let brick = Brick {
            width,
            depth,
            height,
        };

        (origin, brick)
    })
}

fn build_support_map(
    positioned_bricks: &[(Position, Brick)],
) -> (
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
) {
    let extents = positioned_bricks
        .iter()
        .map(|positioned_brick| positioned_brick.get_max_position())
        .fold(Position::default(), Position::max);

    // Stores the max height for that column and the index of the brick at that point
    let mut max_occupancy_by_column =
        vec![vec![None; extents.y as usize + 1]; extents.x as usize + 1];

    let mut supporting = HashMap::new();
    let mut supported_by = HashMap::new();
    let mut supports = HashSet::<usize>::new();

    for (i, positioned_brick) in positioned_bricks.iter().enumerate() {
        let mut insert_height = 0;

        for Position { x, y, z: _ } in positioned_brick.get_occupied_columns() {
            let x = x as usize;
            let y = y as usize;

            if let Some((max_occupied, supporting_brick_index)) = max_occupancy_by_column[x][y] {
                if max_occupied >= insert_height {
                    insert_height = max_occupied + 1;
                    supports.clear();
                    supports.insert(supporting_brick_index);
                } else if max_occupied == insert_height - 1 {
                    supports.insert(supporting_brick_index);
                }
            }
        }

        for support in supports.iter() {
            supporting
                .entry(*support)
                .or_insert_with(HashSet::new)
                .insert(i);
        }

        supported_by.insert(i, supports.clone());
        supports.clear();

        for Position { x, y, z: _ } in positioned_brick.get_occupied_columns() {
            let x = x as usize;
            let y = y as usize;
            let insert_height = insert_height + positioned_brick.1.height - 1;

            max_occupancy_by_column[x][y] = Some((insert_height, i));
        }
    }

    (supporting, supported_by)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Offset {
    x: i64,
    y: i64,
    z: i64,
}

impl Offset {
    fn as_position(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Default)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl From<&str> for Position {
    fn from(s: &str) -> Self {
        match *s.split(',').map(|n| n.parse().unwrap()).collect::<Vec<_>>() {
            [x, y, z] => Self { x, y, z },
            _ => unreachable!(),
        }
    }
}

impl Position {
    fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

impl Add<Offset> for Position {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Offset> for Position {
    type Output = Self;

    fn sub(self, rhs: Offset) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

struct Brick {
    width: usize,
    depth: usize,
    height: usize,
}

trait Occupance {
    fn get_min_position(&self) -> Position;
    fn get_max_position(&self) -> Position;

    fn get_occupied_columns(&self) -> impl Iterator<Item = Position> + '_ {
        let min = self.get_min_position();
        let max = self.get_max_position();

        (min.x..=max.x)
            .flat_map(move |x| (min.y..=max.y).zip(iter::repeat(x)))
            .map(move |(x, y)| Position { x, y, z: min.z })
    }
}

impl Occupance for (Position, Brick) {
    fn get_min_position(&self) -> Position {
        self.0
    }

    fn get_max_position(&self) -> Position {
        self.0
            + Offset {
                x: self.1.width as i64 - 1,
                y: self.1.depth as i64 - 1,
                z: self.1.height as i64 - 1,
            }
    }
}

fn solve_part2(input: &str) -> usize {
    let mut initial_state = parse_initial_state(input).collect::<Vec<_>>();
    initial_state.sort_by(|a, b| a.get_min_position().z.cmp(&b.get_min_position().z));

    let (supporting, supported_by) = build_support_map(initial_state.as_slice());

    let mut would_fall = HashMap::new();

    for i in 0..initial_state.len() {
        let mut this_would_fall = HashSet::from([i]);
        let mut recurse = BinaryHeap::from([Reverse(i)]);

        while let Some(Reverse(i)) = recurse.pop() {
            if let Some(s) = supporting.get(&i) {
                for other_brick_index in s.iter() {
                    let this_supported_by = supported_by.get(other_brick_index).unwrap();

                    if this_supported_by.len()
                        == this_supported_by.intersection(&this_would_fall).count()
                    {
                        // all bricks supporting us are going to fall, so we will also fall
                        this_would_fall.insert(*other_brick_index);
                        recurse.push(Reverse(*other_brick_index));
                    }
                }
            }
        }

        would_fall.insert(i, this_would_fall);
    }

    would_fall.values().map(|it| it.len() - 1).sum()
}
