use std::collections::{HashMap, VecDeque};

const INPUT_TEST_1: &str = include_str!("../input_test_1.txt");
const INPUT_TEST_2: &str = include_str!("../input_test_2.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST_1);
    println!("Test Part 1,1: {}", test_result);
    assert!(test_result == 32_000_000);

    let test_result = solve_part1(INPUT_TEST_2);
    println!("Test Part 1,2: {}", test_result);
    assert!(test_result == 11_687_500);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    // No test input for part 2

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let mut network = CommunicationNetwork::from(input);

    let (num_high, num_low) = (0..1000)
        .map(|_| {
            network.broadcast(Pulse::Low).iter().fold(
                (0, 0),
                |aggregate, (_, pulse, _)| match pulse {
                    Pulse::High => (aggregate.0 + 1, aggregate.1),
                    Pulse::Low => (aggregate.0, aggregate.1 + 1),
                },
            )
        })
        .fold((0, 0), |aggregate, result| {
            (aggregate.0 + result.0, aggregate.1 + result.1)
        });

    num_high * num_low
}

fn solve_part2(input: &str) -> usize {
    let mut network = CommunicationNetwork::from(input);

    let mut known_cycle_times = HashMap::<String, Option<usize>>::new();

    // jz is the only input to rx
    for key in &network.modules["jz"].inputs {
        known_cycle_times.insert(key.to_string(), None);
    }

    // jz is a conjunction, so we need to find the cycle time of its inputs
    // and then take the lcm of those
    for pushes in 1.. {
        let broadcasted_pulses = network.broadcast(Pulse::Low);

        for (sender, pulse, receiver) in broadcasted_pulses {
            if pulse == Pulse::High && receiver == "jz" {
                if let Some(None) = known_cycle_times.get(&sender.to_string()) {
                    known_cycle_times.insert(sender.to_string(), Some(pushes));
                }
            }
        }

        if known_cycle_times.values().all(Option::is_some) {
            break;
        }
    }

    let known_inputs = known_cycle_times
        .values()
        .flatten()
        .copied()
        .collect::<Vec<_>>();

    lcm(known_inputs[0], &known_inputs[1..])
}

fn lcm(a: usize, b: &[usize]) -> usize {
    if b.is_empty() {
        return a;
    }
    let b = lcm(b[0], &b[1..]);

    a * (b / gcd(a, b))
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

struct CommunicationNetwork<'a> {
    modules: HashMap<&'a str, ConnectedModule<'a>>,
}

impl CommunicationNetwork<'_> {
    fn broadcast(&mut self, pulse: Pulse) -> Vec<(&str, Pulse, &str)> {
        let mut stack = VecDeque::from([("button", pulse, Module::<'_>::KEY_BROADCASTER)]);

        let mut sent_signals = Vec::new();

        while let Some((source_key, pulse, target_key)) = stack.pop_front() {
            sent_signals.push((source_key, pulse, target_key));

            let connected_module = self.modules.get_mut(target_key).unwrap();

            if let Some(output) = connected_module
                .module
                .get_output_for_input(source_key, pulse)
            {
                for output_key in connected_module.outputs.iter() {
                    stack.push_back((target_key, output, *output_key));
                }
            }
        }

        sent_signals
    }
}

impl<'a> From<&'a str> for CommunicationNetwork<'a> {
    fn from(value: &'a str) -> Self {
        let mut modules: HashMap<&'a str, ConnectedModule<'a>> = value
            .lines()
            .map(ConnectedModule::from)
            .map(|connected_module| (connected_module.key, connected_module))
            .collect();

        let mut inputs_by_module_key = modules.values().fold(
            HashMap::<&str, Vec<&str>>::new(),
            |mut connections, connected_module| {
                for output in connected_module.outputs.iter() {
                    connections
                        .entry(output)
                        .or_default()
                        .push(connected_module.key);
                }

                connections
            },
        );

        for (key, inputs) in inputs_by_module_key.drain() {
            let connected_module = modules
                .entry(key)
                .or_insert_with_key(|key| ConnectedModule {
                    key,
                    ..Default::default()
                });

            connected_module.module.assign_inputs(&inputs);
            connected_module.inputs = inputs;
        }

        Self { modules }
    }
}

#[derive(Default)]
struct ConnectedModule<'a> {
    key: &'a str,
    module: Module<'a>,
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
}

impl<'a> From<&'a str> for ConnectedModule<'a> {
    fn from(value: &'a str) -> Self {
        let (module, outputs) = value.split_once(" -> ").unwrap();

        let (key, module) = Module::parse_definition(module);
        let outputs = outputs.trim().split(", ").collect();

        Self {
            key,
            module,
            outputs,
            ..Default::default()
        }
    }
}

#[derive(Default)]
enum Module<'a> {
    #[default]
    OutputSink,
    Broadcaster,
    FlipFlop(FlipFlopState),
    Conjunction(HashMap<&'a str, Pulse>),
}

#[derive(Default)]
enum FlipFlopState {
    #[default]
    Off,
    On,
}

impl FlipFlopState {
    fn opposite(&self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        }
    }
}

impl<'a> Module<'a> {
    const KEY_BROADCASTER: &'static str = "broadcaster";

    fn parse_definition(value: &'_ str) -> (&'_ str, Self) {
        match value {
            "broadcaster" => (Self::KEY_BROADCASTER, Module::Broadcaster),
            module => match module.split_at(1) {
                ("%", label) => (label, Module::FlipFlop(FlipFlopState::default())),
                ("&", label) => (label, Module::Conjunction(HashMap::new())),
                _ => unreachable!(),
            },
        }
    }

    fn assign_inputs(&mut self, inputs: &[&'a str]) {
        // Only conjunctions directly care about their inputs
        if let Self::Conjunction(module_inputs) = self {
            for input in inputs.to_owned().drain(..) {
                module_inputs.insert(input, Pulse::Low);
            }
        }
    }

    fn get_output_for_input(&mut self, source: &'_ str, input: Pulse) -> Option<Pulse> {
        match self {
            Self::OutputSink => None,
            Self::Broadcaster => Some(input),
            Self::FlipFlop(ref mut state) => match input {
                Pulse::High => None,
                Pulse::Low => {
                    *state = state.opposite();

                    match state {
                        FlipFlopState::On => Some(Pulse::High),
                        FlipFlopState::Off => Some(Pulse::Low),
                    }
                }
            },
            Self::Conjunction(ref mut module_inputs) => {
                *module_inputs.get_mut(source).unwrap() = input;

                if module_inputs.values().all(|&input| input == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}
