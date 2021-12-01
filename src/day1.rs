#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u32>;

pub fn input_generator(input: &str) -> Input {
	input.lines().map(|line| line.parse().expect("Invalid input")).collect()
}

pub fn part1(input: &Input) -> usize {
	input.windows(2).filter(|s| s[1] > s[0]).count()
}

pub fn part2(input: &Input) -> usize {
	input.windows(4).filter(|s| s[3] > s[0]).count()
}
