use std::{cmp::Reverse, collections::BinaryHeap, time::Instant};

use advent_of_code_2025_lib::Offset;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let before = Instant::now();
    let part_1_result = solve_part1::<1000>(INPUT);
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
fn solve_part1<const N: usize>(input: &str) -> usize {
    type Position = Offset<isize, 3>;

    let junction_boxes = input
        .lines()
        .flat_map(Position::from_comma_separated_str)
        .collect::<Vec<_>>();

    let mut heap: BinaryHeap<(Reverse<usize>, usize, usize)> =
        BinaryHeap::with_capacity(junction_boxes.len() * junction_boxes.len());

    for (i, a) in junction_boxes.iter().enumerate() {
        for (j, b) in junction_boxes.iter().enumerate().skip(i + 1) {
            heap.push((Reverse(a.euclidean_distance_sq(b)), i, j));
        }
    }

    let mut circuits = vec![const { None::<usize> }; junction_boxes.len()];
    let mut next_circuit_idx = 0;

    let mut n = 0;

    while let Some((_, a_idx, b_idx)) = heap.pop() {
        let a_circuit_idx = circuits[a_idx];
        let b_circuit_idx = circuits[b_idx];

        match (a_circuit_idx, b_circuit_idx) {
            (None, None) => {
                circuits[a_idx] = Some(next_circuit_idx);
                circuits[b_idx] = Some(next_circuit_idx);
                next_circuit_idx += 1;
            }
            (None, Some(circuit_idx)) => {
                circuits[a_idx] = Some(circuit_idx);
            }
            (Some(circuit_idx), None) => {
                circuits[b_idx] = Some(circuit_idx);
            }
            (Some(a_circuit_idx), Some(b_circuit_idx)) => {
                if !a_circuit_idx.eq(&b_circuit_idx) {
                    for circuit in circuits.iter_mut() {
                        if let Some(idx) = circuit
                            && *idx == b_circuit_idx
                        {
                            *idx = a_circuit_idx;
                        }
                    }
                }
            }
        }

        n += 1;
        if n >= N {
            break;
        }
    }

    let mut circuits = circuits
        .iter()
        .fold(vec![0; next_circuit_idx], |mut acc, circuit_idx| {
            if let Some(idx) = circuit_idx {
                acc[*idx] += 1;
            }
            acc
        })
        .into_iter()
        .map(Reverse)
        .collect::<Vec<_>>();

    circuits.sort_unstable();

    circuits.iter().take(3).map(|r| r.0).product()
}

#[inline(never)]
fn solve_part2(input: &str) -> usize {
    type Position = Offset<isize, 3>;

    let junction_boxes = input
        .lines()
        .flat_map(Position::from_comma_separated_str)
        .collect::<Vec<_>>();

    let mut heap: BinaryHeap<(Reverse<usize>, usize, usize)> =
        BinaryHeap::with_capacity(junction_boxes.len() * junction_boxes.len());

    for (i, a) in junction_boxes.iter().enumerate() {
        for (j, b) in junction_boxes.iter().enumerate().skip(i + 1) {
            heap.push((Reverse(a.euclidean_distance_sq(b)), i, j));
        }
    }

    let mut circuits = vec![const { None::<usize> }; junction_boxes.len()];
    let mut next_circuit_idx = 0;

    while let Some((_, a_idx, b_idx)) = heap.pop() {
        let a_circuit_idx = circuits[a_idx];
        let b_circuit_idx = circuits[b_idx];

        match (a_circuit_idx, b_circuit_idx) {
            (None, None) => {
                circuits[a_idx] = Some(next_circuit_idx);
                circuits[b_idx] = Some(next_circuit_idx);
                next_circuit_idx += 1;
            }
            (None, Some(circuit_idx)) => {
                circuits[a_idx] = Some(circuit_idx);
            }
            (Some(circuit_idx), None) => {
                circuits[b_idx] = Some(circuit_idx);
            }
            (Some(a_circuit_idx), Some(b_circuit_idx)) => {
                if !a_circuit_idx.eq(&b_circuit_idx) {
                    for circuit in circuits.iter_mut() {
                        if let Some(idx) = circuit
                            && *idx == b_circuit_idx
                        {
                            *idx = a_circuit_idx;
                        }
                    }
                }
            }
        }

        if circuits.iter().all(Option::is_some) {
            let a = junction_boxes[a_idx];
            let b = junction_boxes[b_idx];
            return a.x().cast_unsigned() * b.x().cast_unsigned();
        }
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let input = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

        assert_eq!(solve_part1::<10>(input), 40);
    }

    #[test]
    fn test_input_part2() {
        let input = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

        assert_eq!(solve_part2(input), 25272);
    }
}
