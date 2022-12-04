# Advent of Code 2022 in Rust

## Overview

My solutions to [Advent of Code 2022](https://adventofcode.com/2022) in Rust.

## Primary Aims

* Further my understanding of Rust.
* Use the same domain language of the puzzles: a `Rucksack` will appear in code as a `Rucksack` struct.

## Other notes

### Non-Aims

I am not aiming to make this code short, or as clever as possible: the focus is on learning how to apply Rust, and not now to use Rust for code golf.

### SAFETY

Unless the puzzles specify that it may not be, all input is assumed to be well-formed. Therefore, there will definitely be some suspicious uses of `.unwrap()` and assuming the length of inputs, etc.
