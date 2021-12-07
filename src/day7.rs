#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u32>;

pub fn input_generator(input: &str) -> Input {
    input
        .split(',')
        .map(|l| l.parse().expect("Invalid input"))
        .collect()
}

fn calculate_cost(positions: &[u32], dest: u32, fuel_cost: fn(u32) -> u32) -> u32 {
    positions
        .iter()
        .map(|&pos| u32::max(dest, pos) - u32::min(dest, pos))
        .map(fuel_cost)
        .sum()
}

pub fn part1(input: &mut Input) -> u32 {
    let len = input.len();
    let (_, &mut median, _) = input.select_nth_unstable(len / 2);
    calculate_cost(input, median, |dist| dist)
}

pub fn part2(input: &Input) -> u32 {
    let mean = input.iter().copied().sum::<u32>() / input.len() as u32;
    (mean..=mean+2)
        .map(|dest| calculate_cost(input, dest, |dest| dest * (dest + 1) / 2))
        .min()
        .unwrap()
}
