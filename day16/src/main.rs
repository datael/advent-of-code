use std::collections::HashSet;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 46);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 51);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let contraption = Contraption::from(input);

    let trace_result = trace(&contraption, Position { x: 0, y: 0 }, Direction::Right);

    trace_result.count_energized()
}

struct Contraption {
    grid: Vec<Vec<Option<Mirror>>>,
    height: usize,
    width: usize,
}

impl From<&str> for Contraption {
    fn from(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '/' => Some(Mirror::ForwardSlash),
                        '\\' => Some(Mirror::BackSlash),
                        '|' => Some(Mirror::Vertical),
                        '-' => Some(Mirror::Horizontal),
                        '.' => None,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect::<Vec<Vec<Option<Mirror>>>>();

        let height = grid.len();
        let width = grid[0].len();

        Self {
            grid,
            height,
            width,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Mirror {
    ForwardSlash,
    BackSlash,
    Vertical,
    Horizontal,
}

struct TraceResult {
    visits: Vec<Vec<bool>>,
}

impl TraceResult {
    fn count_energized(&self) -> usize {
        self.visits
            .iter()
            .map(|row| row.iter().filter(|&&v| v).count())
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_in_direction(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

fn trace<'a>(
    contraption: &'a Contraption,
    position: Position,
    direction: Direction,
) -> TraceResult {
    let mut trace_result = TraceResult {
        visits: vec![vec![false; contraption.width]; contraption.height],
    };

    fn may_move(contraption: &Contraption, position: &Position, direction: &Direction) -> bool {
        match direction {
            Direction::Up => position.y > 0,
            Direction::Down => position.y < contraption.height - 1,
            Direction::Left => position.x > 0,
            Direction::Right => position.x < contraption.width - 1,
        }
    }

    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    stack.push((position, direction));

    while let Some((position, direction)) = stack.pop() {
        if visited.contains(&(position, direction)) {
            continue;
        } else {
            visited.insert((position, direction));
        }

        macro_rules! maybe_push {
            ($next_direction: expr) => {
                if may_move(contraption, &position, &$next_direction) {
                    stack.push((
                        position.move_in_direction(&$next_direction),
                        $next_direction,
                    ));
                }
            };
        }

        trace_result.visits[position.y][position.x] = true;

        if let Some(mirror) = &contraption.grid[position.y][position.x] {
            let (next_direction, maybe_fork_direction) = mirror.get_outputs(&direction);

            maybe_push!(next_direction);

            if let Some(fork_direction) = maybe_fork_direction {
                maybe_push!(fork_direction);
            }
        } else {
            maybe_push!(direction);
        }
    }

    trace_result
}

impl Mirror {
    fn get_outputs(&self, input: &Direction) -> (Direction, Option<Direction>) {
        match self {
            Mirror::ForwardSlash => match input {
                Direction::Up => (Direction::Right, None),
                Direction::Down => (Direction::Left, None),
                Direction::Left => (Direction::Down, None),
                Direction::Right => (Direction::Up, None),
            },
            Mirror::BackSlash => match input {
                Direction::Up => (Direction::Left, None),
                Direction::Down => (Direction::Right, None),
                Direction::Left => (Direction::Up, None),
                Direction::Right => (Direction::Down, None),
            },
            Mirror::Vertical => match input {
                Direction::Up => (Direction::Up, None),
                Direction::Down => (Direction::Down, None),
                Direction::Left => (Direction::Up, Some(Direction::Down)),
                Direction::Right => (Direction::Up, Some(Direction::Down)),
            },
            Mirror::Horizontal => match input {
                Direction::Up => (Direction::Left, Some(Direction::Right)),
                Direction::Down => (Direction::Left, Some(Direction::Right)),
                Direction::Left => (Direction::Left, None),
                Direction::Right => (Direction::Right, None),
            },
        }
    }
}

fn solve_part2(input: &str) -> usize {
    let contraption = Contraption::from(input);

    let top = (0..contraption.width).map(|x| (Position { x, y: 0 }, Direction::Down));
    let bottom = (0..contraption.width).map(|x| {
        (
            Position {
                x,
                y: contraption.width - 1,
            },
            Direction::Up,
        )
    });
    let left = (0..contraption.height).map(|y| (Position { x: 0, y }, Direction::Right));
    let right = (0..contraption.height).map(|y| {
        (
            Position {
                x: contraption.height - 1,
                y,
            },
            Direction::Left,
        )
    });

    top.chain(bottom)
        .chain(left)
        .chain(right)
        .map(|(position, direction)| trace(&contraption, position, direction))
        .map(|trace_result| trace_result.count_energized())
        .max()
        .unwrap()
}
