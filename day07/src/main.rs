use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> Signal {
    let graph = parse_circuit_graph(input);
    evaluate_circuit(&graph)["a"]
}

type Signal = u16;

fn parse_circuit_graph(input: &str) -> HashMap<&str, Gate> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let ConnectedGate { gate, output } = s.into();
            (output, gate)
        })
        .collect()
}

fn evaluate_circuit<'a>(graph: &'a HashMap<&str, Gate>) -> HashMap<&'a str, Signal> {
    let mut known_values = HashMap::new();

    // Assuming the circuit is acyclic (it is), we can just recursively evaluate everything

    fn evaluate<'a>(
        output: &'a str,
        gates: &HashMap<&'a str, Gate<'a>>,
        values: &mut HashMap<&'a str, Signal>,
    ) -> Signal {
        if let Some(&output_value) = values.get(output) {
            return output_value;
        }

        // Close your eyes and pretend that this is clean code

        macro_rules! eval_input {
            ($input:ident) => {
                match $input {
                    Input::Immediate(value) => *value,
                    Input::Wire(input) => evaluate(input, gates, values),
                }
            };
        }

        let value = match gates.get(output).unwrap() {
            Gate::Not(input) => !eval_input!(input),
            Gate::And(input_a, input_b) => eval_input!(input_a) & eval_input!(input_b),
            Gate::Or(input_a, input_b) => eval_input!(input_a) | eval_input!(input_b),
            Gate::LShift(input, amount) => eval_input!(input) << amount,
            Gate::RShift(input, amount) => eval_input!(input) >> amount,
            Gate::Identity(input) => eval_input!(input),
        };

        values.insert(output, value);
        value
    }

    for output in graph.keys() {
        if known_values.contains_key(output) {
            continue;
        }

        evaluate(output, graph, &mut known_values);
    }

    known_values
}

struct ConnectedGate<'a> {
    gate: Gate<'a>,
    output: &'a str,
}

enum Input<'a> {
    Immediate(u16),
    Wire(&'a str),
}

enum Gate<'a> {
    Not(Input<'a>),
    And(Input<'a>, Input<'a>),
    Or(Input<'a>, Input<'a>),
    LShift(Input<'a>, usize),
    RShift(Input<'a>, usize),
    Identity(Input<'a>),
}

impl<'a> From<&'a str> for ConnectedGate<'a> {
    fn from(value: &'a str) -> Self {
        let (gate, output) = value.split_once(" -> ").unwrap();
        let gate = gate.into();

        Self { gate, output }
    }
}

impl<'a> From<&'a str> for Gate<'a> {
    fn from(value: &'a str) -> Self {
        let mut iter = value.split(' ');

        match [iter.next(), iter.next(), iter.next()] {
            [Some("NOT"), Some(input), ..] => Self::Not(input.into()),
            [Some(input), Some("AND"), Some(other)] => Self::And(input.into(), other.into()),
            [Some(input), Some("OR"), Some(other)] => Self::Or(input.into(), other.into()),
            [Some(input), Some("LSHIFT"), Some(amount)] => {
                Self::LShift(input.into(), amount.parse().unwrap())
            }
            [Some(input), Some("RSHIFT"), Some(amount)] => {
                Self::RShift(input.into(), amount.parse().unwrap())
            }
            [Some(input), ..] => Self::Identity(input.into()),
            _ => unreachable!(),
        }
    }
}

impl<'a> From<&'a str> for Input<'a> {
    fn from(value: &'a str) -> Self {
        if let Ok(value) = value.parse() {
            Self::Immediate(value)
        } else {
            Self::Wire(value)
        }
    }
}

fn solve_part2(input: &str) -> Signal {
    let graph = &mut parse_circuit_graph(input);
    let new_b = evaluate_circuit(graph)["a"];
    graph.insert("b", Gate::Identity(Input::Immediate(new_b)));

    evaluate_circuit(graph)["a"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            "123 -> x\n\
            456 -> y\n\
            x AND y -> d\n\
            x OR y -> e\n\
            x LSHIFT 2 -> f\n\
            y RSHIFT 2 -> g\n\
            NOT x -> h\n\
            NOT y -> i",
            [
                ("d", 72),
                ("e", 507),
                ("f", 492),
                ("g", 114),
                ("h", 65412),
                ("i", 65079),
                ("x", 123),
                ("y", 456),
            ],
        )] {
            let graph = parse_circuit_graph(input);
            assert_eq!(evaluate_circuit(&graph), HashMap::from(expected));
        }
    }
}
