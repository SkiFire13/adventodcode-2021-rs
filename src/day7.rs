#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u32>;

pub fn input_generator(input: &str) -> Input {
    input
        .split(',')
        .map(|l| l.parse().expect("Invalid input"))
        .sorted()
        .collect()
}

fn best_fuel_cost(positions: &[u32], fuel_cost: impl Fn(u32) -> u32) -> u32 {
    positions
        .iter()
        .map(|&d| {
            positions
                .iter()
                .map(|&p| fuel_cost(u32::max(d, p) - u32::min(d, p)))
                .sum::<u32>()
        })
        .tuple_windows()
        .min_by_key(|&(c1, c2)| c2 > c1)
        .map(|(c1, _)| c1)
        .unwrap()
}

pub fn part1(input: &Input) -> u32 {
    best_fuel_cost(input, |dist| dist)
}

pub fn part2(input: &Input) -> u32 {
    best_fuel_cost(input, |dist| dist * (dist + 1) / 2)
}
