use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> i32 {
    parse_input(input).calculate_optimal_layout_happiness()
}

fn parse_input(input: &str) -> Relationships<'_> {
    input.lines().map(Relationship::from).into()
}

#[derive(Debug, Default)]
struct Relationships<'a>(
    Vec<&'a str>,
    HashMap<&'a str, HashMap<&'a str, Relationship<'a>>>,
);

impl<'a, 'relationship, I> From<I> for Relationships<'a>
where
    I: Iterator<Item = Relationship<'relationship>>,
    'relationship: 'a,
{
    fn from(relationships_iter: I) -> Self {
        let mut relationships = relationships_iter.fold(Self::default(), |mut acc, r| {
            acc.insert_relationship(r);
            acc
        });

        relationships.0 = relationships.1.keys().copied().collect();

        relationships
    }
}

impl<'a, 'relationship> Relationships<'a>
where
    'relationship: 'a,
{
    fn insert_relationship(&mut self, relationship: Relationship<'relationship>) {
        self.1
            .entry(relationship.subject)
            .or_default()
            .insert(relationship.other_person, relationship);
    }

    fn insert_relationships_with_all(
        mut self,
        person: &'relationship str,
        happiness_change: i32,
    ) -> Self {
        for other_person in &self.0 {
            if *other_person != person {
                self.1.entry(person).or_default().insert(
                    other_person,
                    Relationship {
                        subject: person,
                        other_person,
                        happiness_change,
                    },
                );
                self.1.entry(other_person).or_default().insert(
                    person,
                    Relationship {
                        subject: other_person,
                        other_person: person,
                        happiness_change,
                    },
                );
            }
        }

        self.0.push(person);

        self
    }
}

impl Relationships<'_> {
    fn get_net_change(&self, person_a: &str, person_b: &str) -> i32 {
        self.1[person_a][person_b].happiness_change + self.1[person_b][person_a].happiness_change
    }

    fn calculate_optimal_layout_happiness(&self) -> i32 {
        let num_people = self.0.len();
        let people = &self.0;

        // Once again, a small search space of 8 and 9 factorial (40,320 and 362,880), so the search space is small enough to brute force.
        people
            .iter()
            .permutations(num_people)
            .map(|permutation| {
                permutation
                    .iter()
                    .circular_tuple_windows()
                    .map(|(&person_a, &person_b)| self.get_net_change(person_a, person_b))
                    .sum::<i32>()
            })
            .max()
            .unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Relationship<'a> {
    subject: &'a str,
    other_person: &'a str,
    happiness_change: i32,
}

impl<'a> From<&'a str> for Relationship<'a> {
    fn from(line: &'a str) -> Self {
        let (subject, rem) = line.split_once(" would ").unwrap();
        let (direction, rem) = rem.split_once(' ').unwrap();
        let (happiness_change, rem) = rem
            .split_once(" happiness units by sitting next to ")
            .unwrap();
        let other_person = rem.trim_end_matches('.');

        let happiness_change = happiness_change.parse::<i32>().unwrap();
        let happiness_change = match direction {
            "gain" => happiness_change,
            "lose" => -happiness_change,
            _ => unreachable!(),
        };

        Self {
            subject,
            other_person,
            happiness_change,
        }
    }
}

fn solve_part2(input: &str) -> i32 {
    parse_input(input)
        .insert_relationships_with_all("Me", 0)
        .calculate_optimal_layout_happiness()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_parser() {
        for (input, expected) in [
            (
                "Alice would gain 54 happiness units by sitting next to Bob.",
                Relationship {
                    subject: "Alice",
                    other_person: "Bob",
                    happiness_change: 54,
                },
            ),
            (
                "Alice would lose 79 happiness units by sitting next to Carol.",
                Relationship {
                    subject: "Alice",
                    other_person: "Carol",
                    happiness_change: -79,
                },
            ),
            (
                "Alice would lose 2 happiness units by sitting next to David.",
                Relationship {
                    subject: "Alice",
                    other_person: "David",
                    happiness_change: -2,
                },
            ),
            (
                "Bob would gain 83 happiness units by sitting next to Alice.",
                Relationship {
                    subject: "Bob",
                    other_person: "Alice",
                    happiness_change: 83,
                },
            ),
            (
                "Bob would lose 7 happiness units by sitting next to Carol.",
                Relationship {
                    subject: "Bob",
                    other_person: "Carol",
                    happiness_change: -7,
                },
            ),
            (
                "Bob would lose 63 happiness units by sitting next to David.",
                Relationship {
                    subject: "Bob",
                    other_person: "David",
                    happiness_change: -63,
                },
            ),
            (
                "Carol would lose 62 happiness units by sitting next to Alice.",
                Relationship {
                    subject: "Carol",
                    other_person: "Alice",
                    happiness_change: -62,
                },
            ),
            (
                "Carol would gain 60 happiness units by sitting next to Bob.",
                Relationship {
                    subject: "Carol",
                    other_person: "Bob",
                    happiness_change: 60,
                },
            ),
            (
                "Carol would gain 55 happiness units by sitting next to David.",
                Relationship {
                    subject: "Carol",
                    other_person: "David",
                    happiness_change: 55,
                },
            ),
            (
                "David would gain 46 happiness units by sitting next to Alice.",
                Relationship {
                    subject: "David",
                    other_person: "Alice",
                    happiness_change: 46,
                },
            ),
            (
                "David would lose 7 happiness units by sitting next to Bob.",
                Relationship {
                    subject: "David",
                    other_person: "Bob",
                    happiness_change: -7,
                },
            ),
            (
                "David would gain 41 happiness units by sitting next to Carol.",
                Relationship {
                    subject: "David",
                    other_person: "Carol",
                    happiness_change: 41,
                },
            ),
        ] {
            assert_eq!(Relationship::from(input), expected);
        }
    }

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            "Alice would gain 54 happiness units by sitting next to Bob.\n\
                Alice would lose 79 happiness units by sitting next to Carol.\n\
                Alice would lose 2 happiness units by sitting next to David.\n\
                Bob would gain 83 happiness units by sitting next to Alice.\n\
                Bob would lose 7 happiness units by sitting next to Carol.\n\
                Bob would lose 63 happiness units by sitting next to David.\n\
                Carol would lose 62 happiness units by sitting next to Alice.\n\
                Carol would gain 60 happiness units by sitting next to Bob.\n\
                Carol would gain 55 happiness units by sitting next to David.\n\
                David would gain 46 happiness units by sitting next to Alice.\n\
                David would lose 7 happiness units by sitting next to Bob.\n\
                David would gain 41 happiness units by sitting next to Carol.",
            330,
        )] {
            assert_eq!(solve_part1(input), expected);
        }
    }
}
