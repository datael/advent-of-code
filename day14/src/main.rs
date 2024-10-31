use std::{collections::HashMap, time::Duration};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> u64 {
    const DURATION: Duration = Duration::from_secs(2503);

    parse_input(input)
        .map(|reindeer| reindeer.distance_travelled_after_duration(&DURATION))
        .max()
        .unwrap()
}

fn parse_input(input: &str) -> impl Iterator<Item = Reindeer> {
    input.lines().map(Reindeer::from)
}

#[derive(Debug, PartialEq)]
struct Reindeer<'a> {
    name: &'a str,
    speed: u64,
    fly_duration: Duration,
    rest_duration: Duration,
}

impl<'a> From<&'a str> for Reindeer<'a> {
    fn from(value: &'a str) -> Self {
        let (name, rest) = value.split_once(" can fly ").unwrap();
        let (speed, rest) = rest.split_once(" km/s for ").unwrap();
        let (fly_duration, rest) = rest
            .split_once(" seconds, but then must rest for ")
            .unwrap();
        let (rest_duration, _) = rest.split_once(" ").unwrap();

        let speed = speed.parse::<u64>().unwrap();
        let fly_duration = Duration::from_secs(fly_duration.parse::<u64>().unwrap());
        let rest_duration = Duration::from_secs(rest_duration.parse::<u64>().unwrap());

        Self {
            name,
            speed,
            fly_duration,
            rest_duration,
        }
    }
}

impl Reindeer<'_> {
    fn distance_travelled_after_duration(&self, duration: &Duration) -> u64 {
        let cycle_duration = self.fly_duration + self.rest_duration;

        let num_cycles = duration.div_duration_f64(cycle_duration);
        let (num_full_cycles, partial_cycle) = (num_cycles.floor() as u64, num_cycles % 1.0);

        let full_cycles_distance = num_full_cycles * self.fly_duration.as_secs() * self.speed;

        let partial_cycle_duration = cycle_duration.mul_f64(partial_cycle);
        let partial_cycle_fly_duration = partial_cycle_duration.min(self.fly_duration);
        let partial_cycle_distance = partial_cycle_fly_duration.as_secs() * self.speed;

        full_cycles_distance + partial_cycle_distance
    }
}

fn solve_part2(input: &str) -> u64 {
    let reindeers = parse_input(input).collect::<Vec<_>>();

    let mut wins = HashMap::with_capacity(reindeers.len());
    for reindeer in reindeers.iter() {
        wins.insert(reindeer.name, 0);
    }

    (1..=2503)
        .map(Duration::from_secs)
        .flat_map(|duration| winner_names_at_duration_from_start(&duration, &reindeers))
        .for_each(|name| *wins.get_mut(name).unwrap() += 1);

    *wins.iter().max_by_key(|(_, num_wins)| *num_wins).unwrap().1
}

fn winner_names_at_duration_from_start<'r>(
    duration: &Duration,
    reindeers: &'r [Reindeer],
) -> Vec<&'r str> {
    let mut current_max = None;
    let mut res = Vec::new();

    for reindeer in reindeers.iter() {
        let distance = reindeer.distance_travelled_after_duration(duration);

        match current_max {
            None => {
                current_max = Some(distance);
                res.push(reindeer.name);
            }
            Some(max) => {
                if max <= distance {
                    current_max = Some(distance);
                    if max < distance {
                        res.clear();
                    }
                    res.push(reindeer.name);
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reindeer_parser() {
        for (input, expected) in [
            (
                "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
                Reindeer {
                    name: "Comet",
                    speed: 14,
                    fly_duration: Duration::from_secs(1) * 10,
                    rest_duration: Duration::from_secs(1) * 127,
                },
            ),
            (
                "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
                Reindeer {
                    name: "Dancer",
                    speed: 16,
                    fly_duration: Duration::from_secs(1) * 11,
                    rest_duration: Duration::from_secs(1) * 162,
                },
            ),
        ] {
            assert_eq!(Reindeer::from(input), expected);
        }
    }

    #[test]
    fn test_distance() {
        const DURATION: Duration = Duration::from_secs(1000);

        assert_eq!(
            1120,
            Reindeer {
                name: "Comet",
                speed: 14,
                fly_duration: Duration::from_secs(1) * 10,
                rest_duration: Duration::from_secs(1) * 127,
            }
            .distance_travelled_after_duration(&DURATION)
        );

        assert_eq!(
            1056,
            Reindeer {
                name: "Dancer",
                speed: 16,
                fly_duration: Duration::from_secs(1) * 11,
                rest_duration: Duration::from_secs(1) * 162,
            }
            .distance_travelled_after_duration(&DURATION)
        );
    }
}
