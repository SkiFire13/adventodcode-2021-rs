#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Instruction>;

pub enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

pub fn input_generator(input: &str) -> Input {
	input.lines()
        .map(|line| {
            if line.starts_with("forward") {
                Instruction::Forward(line[8..].parse().unwrap())
            } else if line.starts_with("down ") {
                Instruction::Down(line[5..].parse().unwrap())
            } else {
                Instruction::Up(line[3..].parse().unwrap())
            }
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
