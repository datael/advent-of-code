use std::{collections::HashMap, ops::Range};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 19_114);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 167_409_079_868_000);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let (workflows, machine_parts) = parse_input(input);

    let workflows = workflows
        .into_iter()
        .map(|workflow| (workflow.key, workflow))
        .collect::<HashMap<_, _>>();

    machine_parts
        .iter()
        .filter(|machine_part| {
            let mut current_workflow = "in";

            let final_decision = loop {
                let workflow = workflows.get(current_workflow).unwrap();
                let result = workflow.apply(machine_part);

                match result {
                    WorkflowStepResult::HasSubsequent(key) => {
                        current_workflow = key;
                    }
                    WorkflowStepResult::WorkflowFinished(final_decision) => {
                        break final_decision;
                    }
                }
            };

            match final_decision {
                FinalWorkflowDecision::Accept => true,
                FinalWorkflowDecision::Reject => false,
            }
        })
        .map(|machine_part| machine_part.get_rating())
        .sum()
}

fn parse_input(input: &str) -> (Vec<Workflow>, Vec<MachinePart>) {
    let mut lines = input.lines();

    let mut workflows = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        workflows.push(Workflow::from(line));
    }

    let mut machine_parts = Vec::new();
    for line in lines.by_ref() {
        machine_parts.push(MachinePart::from(line));
    }

    (workflows, machine_parts)
}

struct Workflow<'a> {
    key: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn apply(&self, machine_part: &MachinePart) -> &WorkflowStepResult<'a> {
        for rule in &self.rules {
            if let Some(result) = rule.apply(machine_part) {
                return result;
            }
        }

        unreachable!()
    }
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(input: &'a str) -> Self {
        let (key, rules) = input.split_once('{').unwrap();
        let rules = rules.strip_suffix('}').unwrap();

        Self {
            key,
            rules: rules.split(',').map(Rule::from).collect(),
        }
    }
}

enum Rule<'a> {
    Unconditional(WorkflowStepResult<'a>),
    Conditional(Condition<'a>, WorkflowStepResult<'a>),
}

impl<'a> Rule<'a> {
    fn apply(&self, machine_part: &MachinePart) -> Option<&WorkflowStepResult<'a>> {
        match self {
            Rule::Unconditional(target) => Some(target),
            Rule::Conditional(condition, target) => {
                if condition.compare(machine_part) {
                    Some(target)
                } else {
                    None
                }
            }
        }
    }
}

enum Condition<'a> {
    Property(&'a str, Comparison, usize),
}

impl<'a> From<&'a str> for Condition<'a> {
    fn from(input: &'a str) -> Self {
        let comparison = input.matches(['<', '>']).nth(0).unwrap();
        let (property, compared_value) = input.split_once(comparison).unwrap();

        Self::Property(property, comparison.into(), compared_value.parse().unwrap())
    }
}

impl<'a> Condition<'a> {
    fn compare(&self, machine_part: &MachinePart) -> bool {
        match self {
            Self::Property(property, comparison, value) => match comparison {
                Comparison::LessThan => machine_part.properties[property] < *value,
                Comparison::GreaterThan => machine_part.properties[property] > *value,
            },
        }
    }
}

enum Comparison {
    LessThan,
    GreaterThan,
}

impl From<&str> for Comparison {
    fn from(input: &str) -> Self {
        match input {
            "<" => Comparison::LessThan,
            ">" => Comparison::GreaterThan,
            _ => unreachable!(),
        }
    }
}

enum WorkflowStepResult<'a> {
    HasSubsequent(&'a str),
    WorkflowFinished(FinalWorkflowDecision),
}

impl<'a> From<&'a str> for WorkflowStepResult<'a> {
    fn from(input: &'a str) -> Self {
        match input {
            "A" => Self::WorkflowFinished(FinalWorkflowDecision::Accept),
            "R" => Self::WorkflowFinished(FinalWorkflowDecision::Reject),
            _ => Self::HasSubsequent(input),
        }
    }
}

enum FinalWorkflowDecision {
    Accept,
    Reject,
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(input: &'a str) -> Self {
        if let Some((condition, target)) = input.split_once(':') {
            Self::Conditional(condition.into(), target.into())
        } else {
            Self::Unconditional(input.into())
        }
    }
}

struct MachinePart<'a> {
    properties: HashMap<&'a str, usize>,
}

impl<'a> From<&'a str> for MachinePart<'a> {
    fn from(input: &'a str) -> Self {
        let input = input.trim_matches(['{', '}']);
        let properties = input
            .split(',')
            .map(|part| {
                let (key, value) = part.split_once('=').unwrap();
                (key, value.parse().unwrap())
            })
            .collect();

        Self { properties }
    }
}

impl<'a> MachinePart<'a> {
    fn get_rating(&self) -> usize {
        self.properties.values().sum()
    }
}

fn solve_part2(input: &str) -> usize {
    let (workflows, _) = parse_input(input);

    let workflows = workflows
        .into_iter()
        .map(|workflow| (workflow.key, workflow))
        .collect::<HashMap<_, _>>();

    let mut unprocessed = vec![(
        "in",
        InterimResult {
            applicable_property_ranges: ["x", "m", "a", "s"]
                .iter()
                .map(|property| (*property, (1..4001)))
                .collect::<HashMap<_, _>>(),
        },
    )];

    let mut accepted = Vec::new();

    while let Some((workflow, interim_result)) = unprocessed.pop() {
        let workflow = workflows.get(workflow).unwrap();

        for (interim_result, workflow_step_result) in workflow.filter_applicable(interim_result) {
            match workflow_step_result {
                WorkflowStepResult::HasSubsequent(key) => {
                    unprocessed.push((key, interim_result));
                }
                WorkflowStepResult::WorkflowFinished(FinalWorkflowDecision::Accept) => {
                    accepted.push(interim_result)
                }
                WorkflowStepResult::WorkflowFinished(FinalWorkflowDecision::Reject) => {}
            }
        }
    }

    accepted
        .iter()
        .map(|interim_result| {
            interim_result
                .applicable_property_ranges
                .values()
                .map(|range| range.len())
                .product::<usize>()
        })
        .sum()
}

impl<'a> Workflow<'a> {
    fn filter_applicable(
        &self,
        interim_result: InterimResult<'a>,
    ) -> Vec<(InterimResult<'a>, &WorkflowStepResult<'a>)> {
        let mut results = Vec::new();

        let mut maybe_interim_result = Some(interim_result);

        for rule in &self.rules {
            let ((applicable, target), maybe_not_applicable) =
                rule.filter_applicable(maybe_interim_result.unwrap());

            results.push((applicable, target));

            maybe_interim_result = maybe_not_applicable;

            if maybe_interim_result.is_none() {
                break;
            }
        }

        results
    }
}

impl<'a> Rule<'a> {
    fn filter_applicable(
        &self,
        interim_result: InterimResult<'a>,
    ) -> (
        (InterimResult<'a>, &WorkflowStepResult<'a>),
        Option<InterimResult<'a>>,
    ) {
        match self {
            Rule::Unconditional(target) => ((interim_result, target), None),
            Rule::Conditional(condition, target) => {
                let (applicable, not_applicable) = condition.filter_applicable(interim_result);

                ((applicable, target), Some(not_applicable))
            }
        }
    }
}

impl<'a> Condition<'a> {
    fn filter_applicable(
        &self,
        interim_result: InterimResult<'a>,
    ) -> (InterimResult<'a>, InterimResult<'a>) {
        match self {
            Self::Property(property, comparison, value) => {
                let mut applicable = interim_result.clone();
                let applicable_range = applicable
                    .applicable_property_ranges
                    .get_mut(property)
                    .unwrap();

                let mut not_applicable = interim_result.clone();
                let not_applicable_range = not_applicable
                    .applicable_property_ranges
                    .get_mut(property)
                    .unwrap();

                match comparison {
                    Comparison::LessThan => {
                        applicable_range.end = applicable_range.end.min(*value);
                        not_applicable_range.start = not_applicable_range.start.max(*value);
                    }
                    Comparison::GreaterThan => {
                        not_applicable_range.end = not_applicable_range.end.min(*value + 1);
                        applicable_range.start = applicable_range.start.max(*value + 1);
                    }
                }

                (applicable, not_applicable)
            }
        }
    }
}

#[derive(Clone)]
struct InterimResult<'a> {
    applicable_property_ranges: HashMap<&'a str, Range<usize>>,
}
