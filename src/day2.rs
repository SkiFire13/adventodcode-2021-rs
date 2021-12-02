#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Instruction>;

pub enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| match line.split_once(' ') {
            Some(("forward", rest)) => Instruction::Forward(rest.parse().expect("Invalid input")),
            Some(("down", rest)) => Instruction::Down(rest.parse().expect("Invalid input")),
            Some(("up", rest)) => Instruction::Up(rest.parse().expect("Invalid input")),
            _ => panic!("Invalid input"),
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    let mut hor = 0;
    let mut depth = 0;
    for instr in input {
        match instr {
            Instruction::Forward(f) => hor += f,
            Instruction::Up(u) => depth -= u,
            Instruction::Down(d) => depth += d,
        }
    }
    hor * depth
}

pub fn part2(input: &Input) -> u32 {
    let mut hor = 0;
    let mut aim = 0;
    let mut depth = 0;
    for instr in input {
        match instr {
            Instruction::Forward(f) => {
                hor += f;
                depth += f * aim;
            }
            Instruction::Up(u) => aim -= u,
            Instruction::Down(d) => aim += d,
        }
    }
    hor * depth
}
