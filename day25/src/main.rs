#![deny(clippy::pedantic)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    iter,
};

use rand::distributions::{Distribution, Uniform};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 54);

    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    // Turns out there is no part 2 of day 25
}

fn solve_part1(input: &str) -> usize {
    let edges = build_edges(input);

    // https://en.wikipedia.org/wiki/Karger%27s_algorithm
    //
    // This algorithm or some variant of it is probably the way to do this,
    // but it didn't turn out to be fruitful for me.
    // What DOES work most of the tinme has been to do the following:
    //
    //   The graph has two groups: Group 1 and Group 2. We don't know which nodes are in which group.
    //   When picking two nodes at random, we can either pick two nodes from the same group, or one node from each group.
    //   Assuming the same number of nodes in each group, the probability of picking two nodes from different groups is 1/2. (I guess? I'm not a statistician)
    //   So, if we pick two nodes at random, and they're from different groups, we have traversed an edge that connects the two groups.
    //   Since there's only 3 edges that connect the two groups, eventually we'll have traversed all 3 of them more often than other nodes.
    //   Use that as a priority order for which nodes to try removing.
    //
    //   So our algorithm is therefore:
    //     1. Pick two random nodes
    //     2. Find the shortest path between them
    //     3. Take note of the edges on that path
    //     4. Repeat 1-3 a whole bunch of times
    //     5. Order the edges by most-traversed first
    //     6. Remove 3 edges at a time, and see if the graph is now split into 2 groups
    //     7. If we failed to find the right combination, go back to 1 and try again:
    //          a couple of iterations is usually enough to randomly land at the correct answer

    let full_graph = gather_graph_from_edges(&edges);
    let all_nodes = full_graph.keys().collect::<Vec<_>>();

    let mut rng = rand::thread_rng();
    let mut edge_use_count = HashMap::new();

    let uniform_rng = Uniform::new(0, all_nodes.len());

    loop {
        for _ in 0..1000 {
            let start_index = uniform_rng.sample(&mut rng);
            let goal_index = uniform_rng.sample(&mut rng);

            // We're taking tonnes of samples so it's not a big deal
            // if we get the same one twice and throw the entire sample away
            if start_index == goal_index {
                continue;
            }

            let visited = get_visited_nodes_on_shortest_path_between(
                &full_graph,
                all_nodes[start_index],
                all_nodes[goal_index],
            );

            for i in 0..visited.len() - 1 {
                let edge = Edge::new((visited[i].clone(), visited[i + 1].clone()));
                *edge_use_count.entry(edge).or_insert(0) += 1;
            }
        }

        let cut_priorities =
            edge_use_count
                .drain()
                .fold(BinaryHeap::new(), |mut max_heap, (edge, use_count)| {
                    max_heap.push((use_count, edge));
                    max_heap
                });

        let cut_priority = cut_priorities
            .iter()
            .map(|(_, edge)| edge)
            .take(10)
            .collect::<Vec<_>>();

        for i in 0..cut_priority.len() {
            for j in i + 1..cut_priority.len() {
                for k in j + 1..cut_priority.len() {
                    let mut edges_scratch = edges.clone();

                    edges_scratch.retain(|edge| {
                        edge != cut_priority[i]
                            && edge != cut_priority[j]
                            && edge != cut_priority[k]
                    });

                    let graph = gather_graph_from_edges(&edges_scratch);
                    let groups = gather_groups(&graph);

                    if groups.len() == 2 {
                        return groups.iter().map(HashSet::len).product();
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct NodeLabel<'a>(&'a str);

impl<'a> From<&'a str> for NodeLabel<'a> {
    fn from(s: &'a str) -> Self {
        Self(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Edge<'a>(NodeLabel<'a>, NodeLabel<'a>);

impl<'a> Edge<'a> {
    fn new<T: Into<NodeLabel<'a>>>(nodes: (T, T)) -> Self {
        let nodes = (nodes.0.into(), nodes.1.into());

        if nodes.0 < nodes.1 {
            Self(nodes.0, nodes.1)
        } else {
            Self(nodes.1, nodes.0)
        }
    }
}

fn build_edges(input: &str) -> Vec<Edge> {
    input
        .lines()
        .flat_map(|line| {
            let (left, rights) = line.split_once(':').unwrap();

            let left = left.trim();
            let rights = rights.trim().split(' ');

            iter::repeat(left).zip(rights)
        })
        .map(Edge::new)
        .collect::<HashSet<_>>()
        .iter()
        .cloned()
        .collect()
}

fn gather_graph_from_edges<'graph, 'edge: 'graph>(
    edges: &'graph [Edge<'edge>],
) -> HashMap<NodeLabel<'edge>, Vec<NodeLabel<'edge>>> {
    edges
        .iter()
        .fold(HashMap::new(), |mut acc, Edge(left, right)| {
            acc.entry(left.clone()).or_default().push(right.clone());
            acc.entry(right.clone()).or_default().push(left.clone());
            acc
        })
}

fn get_visited_nodes_on_shortest_path_between<'graph, 'edge: 'graph>(
    graph: &'graph HashMap<NodeLabel<'edge>, Vec<NodeLabel<'edge>>>,
    start: &'graph NodeLabel<'edge>,
    goal: &'graph NodeLabel<'edge>,
) -> Vec<&'graph NodeLabel<'edge>> {
    let route = vec![start];
    let mut seen = HashMap::from([(start, 0)]);
    let mut nexts = BinaryHeap::from([Reverse((0, route, start))]);

    while let Some(Reverse((distance, route, next))) = nexts.pop() {
        // If we are at the goal, we're done:
        // We don't care about this being the shortest
        // path; we just want *some* path
        if next == goal {
            return route;
        }

        let distance = distance + 1;

        for next_node in graph.get(next).unwrap() {
            // Only traverse if we've not already been here on a shorter route
            if let Some(prev_distance) = seen.get_mut(next_node) {
                if *prev_distance <= distance {
                    continue;
                }
                *prev_distance = distance;
            } else {
                seen.insert(next_node, distance);
            }

            let mut route = route.clone();
            route.push(next_node);
            nexts.push(Reverse((distance, route, next_node)));
        }
    }

    unreachable!()
}

fn gather_groups<'graph, 'edge: 'graph>(
    graph: &'graph HashMap<NodeLabel<'edge>, Vec<NodeLabel<'edge>>>,
) -> Vec<HashSet<NodeLabel<'edge>>> {
    let mut remaining_nodes = graph.keys().collect::<Vec<_>>();
    let mut groups = Vec::<HashSet<NodeLabel<'edge>>>::new();

    while let Some(left) = remaining_nodes.pop() {
        let target = if let Some(i) = groups.iter().position(|group| group.contains(left)) {
            groups.get_mut(i).unwrap()
        } else {
            let next = HashSet::new();
            groups.push(next);
            groups.last_mut().unwrap()
        };

        graph.get(left).unwrap().iter().for_each(|node| {
            if target.insert(node.clone()) {
                remaining_nodes.push(node);
            }
        });
    }

    groups
}
