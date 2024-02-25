const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(Command::try_from)
        .fold(LightGrid::<1000, 1000>::default(), |mut grid, command| {
            command.apply::<1000, 1000, Part1Strategy>(&mut grid);
            grid
        })
        .count_lights()
}

struct Command {
    action: Action,
    region: Region,
}

impl TryFrom<&str> for Command {
    type Error = ();

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut split = line.split(' ');

        let action = match split.next() {
            Some("turn") => match split.next() {
                Some("on") => Action::TurnOn,
                Some("off") => Action::TurnOff,
                _ => return Err(()),
            },
            Some("toggle") => Action::Toggle,
            _ => return Err(()),
        };

        let from = split.next().unwrap().into();
        _ = split.next(); // "through"
        let to = split.next().unwrap().into();

        Ok(Self {
            action,
            region: Region { from, to },
        })
    }
}

enum Action {
    TurnOn,
    Toggle,
    TurnOff,
}

struct Region {
    from: Position,
    to: Position,
}

struct Position {
    x: usize,
    y: usize,
}

impl From<&str> for Position {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(',').expect("Invalid input is unexpected");

        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        Self { x, y }
    }
}

struct LightGrid<const W: usize, const H: usize> {
    lights: Vec<u8>,
}

impl<const W: usize, const H: usize> Default for LightGrid<W, H> {
    fn default() -> Self {
        Self {
            lights: vec![0; W * H],
        }
    }
}

impl<const W: usize, const H: usize> LightGrid<W, H> {
    fn count_lights(&self) -> usize {
        self.lights.iter().filter(|&&light| light > 0).count()
    }
}

impl Command {
    fn apply<const W: usize, const H: usize, S: Strategy>(&self, grid: &mut LightGrid<W, H>) {
        let maxx = std::cmp::min(self.region.to.x, W - 1);
        let maxy = std::cmp::min(self.region.to.y, H - 1);

        assert!(maxy * maxx < W * H);

        for y in self.region.from.y..=maxy {
            for x in self.region.from.x..=maxx {
                let light = grid.lights.get_mut(y * W + x).unwrap();
                S::apply(&self.action, light);
            }
        }
    }
}

trait Strategy {
    fn apply(action: &Action, light: &mut u8);
}

struct Part1Strategy;

impl Strategy for Part1Strategy {
    fn apply(action: &Action, light: &mut u8) {
        match action {
            Action::TurnOn => *light = 1,
            Action::TurnOff => *light = 0,
            Action::Toggle => *light = if *light == 1 { 0 } else { 1 },
        }
    }
}

fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .flat_map(Command::try_from)
        .fold(LightGrid::<1000, 1000>::default(), |mut grid, command| {
            command.apply::<1000, 1000, Part2Strategy>(&mut grid);
            grid
        })
        .total_brightness()
}

struct Part2Strategy;

impl Strategy for Part2Strategy {
    fn apply(action: &Action, light: &mut u8) {
        match action {
            Action::TurnOn => *light = light.saturating_add(1),
            Action::TurnOff => *light = light.saturating_sub(1),
            Action::Toggle => *light = light.saturating_add(2),
        }
    }
}

impl<const W: usize, const H: usize> LightGrid<W, H> {
    fn total_brightness(&self) -> usize {
        self.lights.iter().copied().map(usize::from).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [
            ("turn on 0,0 through 999,999", 1_000_000),
            ("toggle 0,0 through 999,0", 1_000),
            (
                "turn on 0,0 through 999,999\nturn off 499,499 through 500,500",
                1_000_000 - 4,
            ),
        ] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [
            ("turn on 0,0 through 0,0", 1),
            ("toggle 0,0 through 999,999", 2_000_000),
        ] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
