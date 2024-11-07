use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> u32 {
    let test = [
        ("children", Condition::Equals(3)),
        ("cats", Condition::Equals(7)),
        ("samoyeds", Condition::Equals(2)),
        ("pomeranians", Condition::Equals(3)),
        ("akitas", Condition::Equals(0)),
        ("vizslas", Condition::Equals(0)),
        ("goldfish", Condition::Equals(5)),
        ("trees", Condition::Equals(3)),
        ("cars", Condition::Equals(2)),
        ("perfumes", Condition::Equals(1)),
    ]
    .into();

    let sue = input
        .lines()
        .map(Sue::from)
        .find(|sue| sue.matches(&test))
        .unwrap();

    sue.number
}

#[derive(Debug, PartialEq, Eq)]
struct Sue<'a> {
    number: u32,
    properties: HashMap<&'a str, u32>,
}

impl<'a> From<&'a str> for Sue<'a> {
    fn from(value: &'a str) -> Self {
        let (_, rest) = value.split_once(" ").unwrap();
        let (number, rest) = rest.split_once(": ").unwrap();

        let properties = rest
            .split(", ")
            .flat_map(|property| property.split_once(": "))
            .flat_map(|(property_name, number)| {
                number.parse().map(|number| (property_name, number))
            })
            .collect();

        let number = number.parse().unwrap();

        Self { number, properties }
    }
}

impl Sue<'_> {
    fn matches(&self, test: &HashMap<&str, Condition>) -> bool {
        for (property, value) in self.properties.iter() {
            if let Some(condition) = test.get(property) {
                if !condition.matches(value) {
                    return false;
                }
            }
        }

        true
    }
}

enum Condition {
    Equals(u32),
    FewerThan(u32),
    GreaterThan(u32),
}

impl Condition {
    fn matches(&self, value: &u32) -> bool {
        match self {
            Condition::Equals(eq) => value == eq,
            Condition::FewerThan(ft) => value < ft,
            Condition::GreaterThan(gt) => gt < value,
        }
    }
}

fn solve_part2(input: &str) -> u32 {
    let test = [
        ("children", Condition::Equals(3)),
        ("cats", Condition::GreaterThan(7)),
        ("samoyeds", Condition::Equals(2)),
        ("pomeranians", Condition::FewerThan(3)),
        ("akitas", Condition::Equals(0)),
        ("vizslas", Condition::Equals(0)),
        ("goldfish", Condition::FewerThan(5)),
        ("trees", Condition::GreaterThan(3)),
        ("cars", Condition::Equals(2)),
        ("perfumes", Condition::Equals(1)),
    ]
    .into();

    let sue = input
        .lines()
        .map(Sue::from)
        .find(|sue| sue.matches(&test))
        .unwrap();

    sue.number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sue_parser() {
        for (input, expected) in [
            (
                "Sue 1: children: 1, cars: 8, vizslas: 7",
                Sue {
                    number: 1,
                    properties: [("children", 1), ("cars", 8), ("vizslas", 7)].into(),
                },
            ),
            (
                "Sue 123: cars: 7, akitas: 0, pomeranians: 0",
                Sue {
                    number: 123,
                    properties: [("cars", 7), ("akitas", 0), ("pomeranians", 0)].into(),
                },
            ),
        ] {
            assert_eq!(Sue::from(input), expected);
        }
    }
}
