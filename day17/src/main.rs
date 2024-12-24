#![allow(clippy::upper_case_acronyms)]

use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> String {
    let (cpu_state, program) = input.split_once("\n\n").unwrap();

    let mut cpu = CPU::from(cpu_state);
    let (_, program) = program.trim().split_once(": ").unwrap();

    let mut context = Context::default();
    cpu.apply_program_str(program, &mut context);

    context.get_outputs_as_comma_separated_string()
}

#[derive(Default)]
struct Context {
    outputs: Vec<u8>,
}

impl Context {
    fn get_outputs_as_comma_separated_string(&self) -> String {
        if self.outputs.is_empty() {
            String::new()
        } else if self.outputs.len() == 1 {
            let mut output = String::with_capacity(1);
            output.push((b'0' + self.outputs[0]) as char);
            output
        } else {
            let mut init = String::with_capacity(self.outputs.len() * 2 - 1);
            init.push((b'0' + self.outputs[0]) as char);

            self.outputs.iter().skip(1).fold(init, |mut acc, v| {
                acc.push(',');
                acc.push((b'0' + v) as char);
                acc
            })
        }
    }
}

#[derive(Default, Clone)]
struct CPU {
    instruction_ptr: usize,
    registers: RegisterState,
    jumped_last_instruction: bool,
}

impl CPU {
    fn apply_program(&mut self, program: &RawProgram, context: &mut Context) {
        while let Some((instruction, size)) =
            Instructions::parse_next(self.instruction_ptr, program)
        {
            instruction.apply_to(self, context);

            if !self.jumped_last_instruction {
                self.instruction_ptr += size;
            }

            self.jumped_last_instruction = false;
        }
    }

    fn apply_program_str(&mut self, program: &str, context: &mut Context) {
        let program = RawProgram::from(program);
        self.apply_program(&program, context);
    }

    fn jump_to(&mut self, position: usize) {
        self.instruction_ptr = position;
        self.jumped_last_instruction = true;
    }
}

impl<S> From<S> for CPU
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let mut register_state = RegisterState([0; 3]);
        for (index, line) in value.as_ref().lines().enumerate() {
            let (_, register_value) = line.split_once(": ").expect("Input was invalid");
            register_state[index] = register_value.parse().expect("Input was invalid");
        }

        Self {
            instruction_ptr: 0,
            registers: register_state,
            jumped_last_instruction: false,
        }
    }
}

struct ComboOperand(u8);

impl ComboOperand {
    fn get_value(&self, cpu: &CPU) -> usize {
        match self.0 {
            lit @ 0..=3 => lit as usize,
            4 => cpu.registers.get_register(Register::A),
            5 => cpu.registers.get_register(Register::B),
            6 => cpu.registers.get_register(Register::C),
            _ => panic!("Invalid operand"),
        }
    }
}

enum Register {
    A,
    B,
    C,
}

#[derive(Default, Clone)]
struct RegisterState([usize; 3]);

impl RegisterState {
    fn set_register(&mut self, register: Register, value: usize) {
        self.0[match register {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
        }] = value;
    }

    fn get_register(&self, register: Register) -> usize {
        self.0[match register {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
        }]
    }

    fn get_register_mut(&mut self, register: Register) -> &mut usize {
        &mut self.0[match register {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
        }]
    }
}

impl Deref for RegisterState {
    type Target = [usize; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RegisterState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

enum Instructions {
    ADV(cpu_instructions::ADV),
    BXL(cpu_instructions::BXL),
    BST(cpu_instructions::BST),
    JNZ(cpu_instructions::JNZ),
    BXC(cpu_instructions::BXC),
    OUT(cpu_instructions::OUT),
    BDV(cpu_instructions::BDV),
    CDV(cpu_instructions::CDV),
}

impl Instructions {
    fn parse_next(instruction_ptr: usize, bytes: &[u8]) -> Option<(Instructions, usize)> {
        let next = bytes.get(instruction_ptr)?;

        macro_rules! single_operand {
            ($instruction:ident) => {
                Some((
                    Instructions::$instruction(Into::<cpu_instructions::$instruction>::into(
                        *bytes.get(instruction_ptr + 1)?,
                    )),
                    2,
                ))
            };
        }

        match next {
            0 => single_operand!(ADV),
            1 => single_operand!(BXL),
            2 => single_operand!(BST),
            3 => single_operand!(JNZ),
            4 => single_operand!(BXC),
            5 => single_operand!(OUT),
            6 => single_operand!(BDV),
            7 => single_operand!(CDV),
            _ => panic!("Unknown opcode"),
        }
    }

    fn apply_to(&self, cpu: &mut CPU, context: &mut Context) {
        use cpu_instructions::Instruction;

        match self {
            Instructions::ADV(adv) => adv.apply_to(cpu, context),
            Instructions::BXL(bxl) => bxl.apply_to(cpu, context),
            Instructions::BST(bst) => bst.apply_to(cpu, context),
            Instructions::JNZ(jnz) => jnz.apply_to(cpu, context),
            Instructions::BXC(bxc) => bxc.apply_to(cpu, context),
            Instructions::OUT(out) => out.apply_to(cpu, context),
            Instructions::BDV(bdv) => bdv.apply_to(cpu, context),
            Instructions::CDV(cdv) => cdv.apply_to(cpu, context),
        }
    }
}

mod cpu_instructions {
    use super::{CPU, ComboOperand, Context, Register};

    pub trait Instruction {
        fn apply_to(&self, cpu: &mut CPU, context: &mut Context);
    }

    macro_rules! oper_combo {
        ($type:ident) => {
            pub struct $type(ComboOperand);

            impl From<u8> for $type {
                fn from(value: u8) -> Self {
                    Self(ComboOperand(value))
                }
            }
        };
    }

    macro_rules! oper_literal {
        ($type:ident) => {
            pub struct $type(#[allow(unused)] u8);

            impl From<u8> for $type {
                fn from(value: u8) -> Self {
                    Self(value)
                }
            }
        };
    }

    oper_combo!(ADV);
    oper_literal!(BXL);
    oper_combo!(BST);
    oper_literal!(JNZ);
    oper_literal!(BXC);
    oper_combo!(OUT);
    oper_combo!(BDV);
    oper_combo!(CDV);

    impl Instruction for ADV {
        fn apply_to(&self, cpu: &mut CPU, _: &mut Context) {
            let numerator = cpu.registers.get_register(Register::A);
            let denominator = 2usize.pow(self.0.get_value(cpu) as u32);

            cpu.registers
                .set_register(Register::A, numerator / denominator);
        }
    }

    impl Instruction for BXL {
        fn apply_to(&self, cpu: &mut CPU, _: &mut Context) {
            let register = cpu.registers.get_register_mut(Register::B);
            *register ^= self.0 as usize
        }
    }

    impl Instruction for BST {
        fn apply_to(&self, cpu: &mut CPU, _: &mut Context) {
            let op_lower_3 = self.0.get_value(cpu) % 8;
            cpu.registers.set_register(Register::B, op_lower_3);
        }
    }

    impl Instruction for JNZ {
        fn apply_to(&self, cpu: &mut CPU, _: &mut Context) {
            if cpu.registers.get_register(Register::A) == 0 {
                return;
            }

            cpu.jump_to(self.0 as usize);
        }
    }

    impl Instruction for BXC {
        fn apply_to(&self, cpu: &mut CPU, _: &mut Context) {
            let register_c = cpu.registers.get_register(Register::C);
            let register_b = cpu.registers.get_register_mut(Register::B);

            *register_b ^= register_c;
        }
    }

    impl Instruction for OUT {
        fn apply_to(&self, cpu: &mut CPU, context: &mut Context) {
            let op_lower_3 = (self.0.get_value(cpu) % 8) as u8;
            context.outputs.push(op_lower_3);
        }
    }

    impl Instruction for BDV {
        fn apply_to(&self, cpu: &mut CPU, _: &mut Context) {
            let numerator = cpu.registers.get_register(Register::A);
            let denominator = 2usize.pow(self.0.get_value(cpu) as u32);

            cpu.registers
                .set_register(Register::B, numerator / denominator);
        }
    }

    impl Instruction for CDV {
        fn apply_to(&self, cpu: &mut CPU, _: &mut Context) {
            let numerator = cpu.registers.get_register(Register::A);
            let denominator = 2usize.pow(self.0.get_value(cpu) as u32);

            cpu.registers
                .set_register(Register::C, numerator / denominator);
        }
    }
}

struct RawProgram(Vec<u8>);

impl Deref for RawProgram {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> From<S> for RawProgram
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        Self(
            value
                .as_ref()
                .trim()
                .split(',')
                .flat_map(str::parse::<u8>) // flat_map results in us assuming that all input is valid
                .collect::<Vec<_>>(),
        )
    }
}

fn solve_part2(input: &str) -> usize {
    let (_, program) = input.split_once("\n\n").unwrap();
    let (_, program) = program.trim().split_once(": ").unwrap();

    // A simple loop from 0 to however many we need to get is unfeasible, so let's work back step by step.

    // 2,4,1,1,7,5,1,5,4,1,5,5,0,3,3,0 is
    //
    // BST [A]      [B] = [A] % 8
    // BXL 1        [B] ^= 1
    // CDV [B]      [C] = [A] / (2^[B])
    // BXL 5        [B] ^= 5
    // BXC          [B] ^= [C]
    // OUT [B]      << [B]
    // ADV 3        [A] /= 2^3    (2^3 == 8)
    // JNZ 0        if [A] == 0, GOTO 0

    // Rougly, this is:
    //
    // let mut a = 17323786;
    // let mut b = 0;
    // let mut c = 0;
    //
    // while a > 0 {
    //     b = a % 8;
    //     b ^= 1;
    //     c = a / 2^b;   (this is "a >> b")
    //     b ^= 5;
    //     b ^= c;
    //     print!(b);
    //     a /= 8;
    // }

    // Turns out that the initial values of registers B and C do not matter,
    // so we can boil this all down into a single unary function:
    fn calculate_output(a: usize) -> usize {
        let mut b = (a % 8) ^ 1;
        let c = a >> b;
        b ^= 5;
        b ^= c;
        b % 8
    }

    // The multiple XORs; the % 8 at the start and the /= 8 at the end;
    // this all smells of some form of hashing function to me, so reversing this is going to be fun.
    //
    // We know the following:
    // - Each loop outputs one octet of the hash (since it's % 8)
    // - Each loop will roughly(? exactly?) lower the magnitude of the number by one octet (so we're looking for a number that is at least 4 trillion or so?)
    //
    // If we instead step backwards and find possible values for A for each step we should be able to optimize the search space.

    let mut to_try = HashSet::new();
    to_try.insert(0);

    for target_output in RawProgram::from(program)
        .iter()
        .map(|u8| *u8 as usize)
        .rev()
    {
        let mut next_to_try = HashSet::new();
        for number_to_try in to_try {
            let base = number_to_try << 3;
            for a in base..base + 8 {
                if calculate_output(a) == target_output {
                    next_to_try.insert(a);
                }
            }
        }

        to_try = next_to_try
    }

    *to_try
        .iter()
        .min()
        .expect("We should have an answer by now")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adv() {
        let mut cpu = CPU {
            registers: RegisterState([128, 5, 0]),
            ..Default::default()
        };
        let mut context = Context::default();

        cpu.apply_program_str("0,1", &mut context);
        assert_eq!(cpu.registers[0], 64);

        cpu.instruction_ptr = 0;
        cpu.apply_program_str("0,5", &mut context);
        assert_eq!(cpu.registers[0], 2);
    }

    #[test]
    fn test_bxl() {
        let mut cpu = CPU {
            registers: RegisterState([0, 29, 0]),
            ..Default::default()
        };
        let mut context = Context::default();

        cpu.apply_program_str("1,7", &mut context);

        assert_eq!(cpu.registers[1], 26);
    }

    #[test]
    fn test_bst() {
        let mut cpu = CPU {
            registers: RegisterState([0, 0, 9]),
            ..Default::default()
        };
        let mut context = Context::default();

        cpu.apply_program_str("2,6", &mut context);

        assert_eq!(cpu.registers[1], 1);
    }

    #[test]
    fn test_jnz() {
        let mut cpu = CPU {
            registers: RegisterState([2024, 0, 0]),
            ..Default::default()
        };
        let mut context = Context::default();

        cpu.apply_program_str("0,1,5,4,3,0", &mut context);

        assert_eq!(
            context.get_outputs_as_comma_separated_string(),
            "4,2,5,6,7,7,7,7,3,1,0"
        );
        assert_eq!(cpu.registers[0], 0);
    }

    #[test]
    fn test_bxc() {
        let mut cpu = CPU {
            registers: RegisterState([0, 2024, 43690]),
            ..Default::default()
        };
        let mut context = Context::default();

        cpu.apply_program_str("4,0", &mut context);

        assert_eq!(cpu.registers[1], 44354);
    }

    #[test]
    fn test_out() {
        let mut cpu = CPU {
            registers: RegisterState([10, 0, 0]),
            ..Default::default()
        };
        let mut context = Context::default();

        cpu.apply_program_str("5,0,5,1,5,4", &mut context);

        assert_eq!(context.get_outputs_as_comma_separated_string(), "0,1,2");
    }

    #[test]
    fn test_bdv() {
        let mut cpu = CPU {
            registers: RegisterState([128, 0, 5]),
            ..Default::default()
        };
        let mut context = Context::default();

        cpu.apply_program_str("6,1", &mut context);
        assert_eq!(cpu.registers[1], 64);

        cpu.instruction_ptr = 0;
        cpu.apply_program_str("6,6", &mut context);
        assert_eq!(cpu.registers[1], 4);
    }

    #[test]
    fn test_cdv() {
        let mut cpu = CPU {
            registers: RegisterState([128, 5, 0]),
            ..Default::default()
        };
        let mut context = Context::default();

        cpu.apply_program_str("7,1", &mut context);
        assert_eq!(cpu.registers[2], 64);

        cpu.instruction_ptr = 0;
        cpu.apply_program_str("7,5", &mut context);
        assert_eq!(cpu.registers[2], 4);
    }

    #[test]
    fn test_part1() {
        for (input, expected) in [(
            r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
            "4,6,3,5,6,3,5,2,1,0",
        )] {
            assert_eq!(solve_part1(input), expected);
        }
    }
}
