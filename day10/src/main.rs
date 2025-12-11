#![feature(portable_simd)]

use std::simd::num::SimdUint;
use std::simd::*;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let before = Instant::now();
    let part_1_result = solve_part1(INPUT);
    let after = Instant::now();
    println!("Part 1: {part_1_result}");
    println!("Part 1 took: {:?}", after - before);

    // let before = Instant::now();
    // let part_2_result = solve_part2(INPUT);
    // let after = Instant::now();
    // println!("Part 2: {part_2_result}");
    // println!("Part 2 took: {:?}", after - before);
}

#[inline(never)]
fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(Machine::from)
        .map(|machine| machine.find_optimal_presses() as usize)
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Machine {
    indicator_lights_target: u16,
    buttons: u16x64,
    num_buttons: u16,
}

impl Machine {
    fn find_optimal_presses(&self) -> u16 {
        const ZERO: u16x64 = u16x64::splat(0);

        let iter_max = 1 << self.num_buttons;

        (0..iter_max / 4)
            .flat_map(|i| {
                let i = i * 4;

                let maskbits = (i << 0) + ((i + 1) << 16) + ((i + 2) << 32) + ((i + 3) << 48);
                let mask = mask16x64::from_bitmask(maskbits);
                let pressed_buttons = mask.select(self.buttons, ZERO);
                let pressed_buttons = pressed_buttons.to_array();

                [(0, 16), (16, 32), (32, 48), (48, 64)]
                    .map(|(from, to)| &pressed_buttons[from..to])
                    .map(u16x16::from_slice)
                    .map(u16x16::reduce_xor)
                    .iter()
                    .enumerate()
                    .filter_map(|(n, &indicator_lights)| {
                        (indicator_lights == self.indicator_lights_target)
                            .then_some((i + (n as u64)).count_ones() as u16)
                    })
                    .min()
            })
            .min()
            .expect("we should have at least one valid combination")
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut split = value.split(' ');

        let indicator_lights_target = split.next().expect("indicator lights");
        let indicator_lights_target =
            &indicator_lights_target[1..indicator_lights_target.len() - 1];

        let indicator_lights_target =
            indicator_lights_target
                .chars()
                .rev()
                .fold(0, |acc, next| match next {
                    '#' => (acc << 1) + 1,
                    _ => acc << 1,
                });

        let mut buttons = const { u16x16::splat(0) };
        let mut num_buttons = 0;

        while let Some(next) = split.next() {
            // "Because none of the machines are running, the joltage requirements are irrelevant and can be safely ignored."
            if next.starts_with('{') {
                break;
            }

            let mut button = 0;
            let bytes = next[1..next.len() - 1].as_bytes();
            for i in (0..bytes.len()).step_by(2) {
                button |= 1 << bytes[i] - b'0';
            }

            buttons = buttons.shift_elements_right::<1>(button);
            num_buttons += 1;
        }

        let buttons = buttons.to_array();
        let buttons = u16x64::from_slice(&[buttons, buttons, buttons, buttons].concat());

        Machine {
            indicator_lights_target,
            buttons,
            num_buttons,
        }
    }
}

#[inline(never)]
fn _solve_part2(_input: &str) -> usize {
    // TODO
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

        assert_eq!(solve_part1(input), 7);
    }

    //     #[test]
    //     fn test_input_part2() {
    //         let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    // [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    //         assert_eq!(solve_part2(input), 33);
    //     }
}
