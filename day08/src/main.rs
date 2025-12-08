use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    time::Instant,
};

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
        .flat_map(Position::from_comma_separated_string)
        .collect::<HashSet<_>>();

    let mut heap: BinaryHeap<(Reverse<usize>, Position, Position)> =
        BinaryHeap::with_capacity(junction_boxes.len() * junction_boxes.len());

    for (i, a) in junction_boxes.iter().enumerate() {
        for b in junction_boxes.iter().skip(i + 1) {
            heap.push((Reverse(a.euclidean_distance_sq(b)), *a, *b));
        }
    }

    let mut junction_boxes = junction_boxes;
    let mut circuits = Vec::<HashSet<Position>>::new();

    let mut n = 0;

    while let Some((_, a, b)) = heap.pop() {
        let circuit_a = circuits.iter().position(|circuit| circuit.contains(&a));
        let circuit_b = circuits.iter().position(|circuit| circuit.contains(&b));

        match (circuit_a, circuit_b) {
            (None, None) => {
                // println!("inserting {:?} and {:?} into NEW", a, b);
                circuits.push(HashSet::from_iter([a, b]));
            }
            (None, Some(circuit_b_position)) => {
                // println!("inserting {:?} into {:?}'s circuit", a, b);
                circuits[circuit_b_position].insert(a);
            }
            (Some(circuit_a_position), None) => {
                // println!("inserting {:?} into {:?}'s circuit", b, a);
                circuits[circuit_a_position].insert(b);
            }
            (Some(circuit_a_position), Some(circuit_b_position))
                if circuit_a_position != circuit_b_position =>
            {
                // println!(
                //     "merging two circuits at indexes {:?} and {:?}",
                //     circuit_a_position, circuit_b_position
                // );

                let mut circuit_b = HashSet::new();
                std::mem::swap(&mut circuit_b, &mut circuits[circuit_b_position]);

                for b in circuit_b.drain() {
                    circuits[circuit_a_position].insert(b);
                }

                circuits.swap_remove(circuit_b_position);
            }
            (Some(_), Some(_)) => {

                // println!("{:?} and {:?} already in the same circuit", b, a);
            }
        }

        junction_boxes.remove(&a);
        junction_boxes.remove(&b);

        if junction_boxes.is_empty() {
            break;
        }

        n += 1;
        if n >= N {
            break;
        }
    }

    circuits.sort_by_key(|a| Reverse(a.len()));

    circuits.iter().take(3).map(HashSet::len).product()
}

#[inline(never)]
fn solve_part2(input: &str) -> usize {
    type Position = Offset<isize, 3>;

    let junction_boxes = input
        .lines()
        .flat_map(Position::from_comma_separated_string)
        .collect::<HashSet<_>>();

    let mut heap: BinaryHeap<(Reverse<usize>, Position, Position)> =
        BinaryHeap::with_capacity(junction_boxes.len() * junction_boxes.len());

    for (i, a) in junction_boxes.iter().enumerate() {
        for b in junction_boxes.iter().skip(i + 1) {
            heap.push((Reverse(a.euclidean_distance_sq(b)), *a, *b));
        }
    }

    let mut junction_boxes = junction_boxes;
    let mut circuits = Vec::<HashSet<Position>>::new();

    while let Some((_, a, b)) = heap.pop() {
        let circuit_a = circuits.iter().position(|circuit| circuit.contains(&a));
        let circuit_b = circuits.iter().position(|circuit| circuit.contains(&b));

        match (circuit_a, circuit_b) {
            (None, None) => {
                circuits.push(HashSet::from_iter([a, b]));
            }
            (None, Some(circuit_b_position)) => {
                circuits[circuit_b_position].insert(a);
            }
            (Some(circuit_a_position), None) => {
                circuits[circuit_a_position].insert(b);
            }
            (Some(circuit_a_position), Some(circuit_b_position))
                if circuit_a_position != circuit_b_position =>
            {
                let mut circuit_b = HashSet::new();
                std::mem::swap(&mut circuit_b, &mut circuits[circuit_b_position]);

                for b in circuit_b.drain() {
                    circuits[circuit_a_position].insert(b);
                }

                circuits.swap_remove(circuit_b_position);
            }
            (Some(_), Some(_)) => {}
        }

        junction_boxes.remove(&a);
        junction_boxes.remove(&b);

        if circuits.len() == 1 && junction_boxes.is_empty() {
            return a.x().cast_unsigned() * b.x().cast_unsigned();
        }

        if junction_boxes.is_empty() {
            break;
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
