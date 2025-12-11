use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let before = Instant::now();
    let part_1_result = solve_part1(INPUT);
    let after = Instant::now();
    println!("Part 1: {part_1_result}");
    println!("Part 1 took: {:?}", after - before);

    let before = Instant::now();
    let part_2_result = solve_part2(INPUT);
    let after = Instant::now();
    println!("Part 2: {part_2_result}");
    println!("Part 2 took: {:?}", after - before);
}

#[inline(never)]
fn solve_part1(input: &str) -> usize {
    let graph: Graph = input.into();

    let memo = &mut HashMap::new();
    graph.count_paths("you", "out", memo)
}

struct Graph<'a>(HashMap<&'a str, HashSet<&'a str>>);

impl<'s> From<&'s str> for Graph<'s> {
    fn from(value: &'s str) -> Self {
        Self(
            value
                .lines()
                .map(|line| {
                    let (name, rest) = line.split_once(": ").expect("correctly formatted data");
                    let connections = rest.split_ascii_whitespace().collect();

                    (name, connections)
                })
                .collect(),
        )
    }
}

impl<'s> Graph<'s> {
    fn count_paths(&self, from: &'s str, to: &'s str, memo: &mut HashMap<&'s str, usize>) -> usize {
        if from == to {
            return 1;
        }

        if let Some(memoized) = memo.get(from) {
            return *memoized;
        }

        let mut num_paths = 0;
        for connection in self.0.get(from).unwrap_or(&HashSet::new()).iter() {
            num_paths += self.count_paths(connection, to, memo);
        }

        memo.insert(from, num_paths);
        num_paths
    }
}

#[inline(never)]
fn solve_part2(input: &str) -> usize {
    let graph: Graph = input.into();

    let memo = &mut HashMap::new();
    let svr_dac = graph.count_paths("svr", "dac", memo);
    memo.clear();
    let dac_fft = graph.count_paths("dac", "fft", memo);
    memo.clear();
    let fft_out = graph.count_paths("fft", "out", memo);

    memo.clear();
    let svr_fft = graph.count_paths("svr", "fft", memo);
    memo.clear();
    let fft_dac = graph.count_paths("fft", "dac", memo);
    memo.clear();
    let dac_out = graph.count_paths("dac", "out", memo);

    svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let input = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

        assert_eq!(solve_part1(input), 5);
    }

    #[test]
    fn test_input_part2() {
        let input = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

        assert_eq!(solve_part2(input), 2);
    }
}
