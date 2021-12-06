#[allow(unused_imports)]
use super::prelude::*;
type Input = [u32; 9];

pub fn input_generator(input: &str) -> Input {
    let mut fishes = [0; 9];
    for n in input.split(',') {
        fishes[n.parse::<usize>().expect("Invalid input")] += 1;
    }
    fishes
}

fn simulate_fishes<T>(fishes: [u32; 9], days: usize) -> T
where
    T: From<u32> + Copy + std::ops::AddAssign + std::iter::Sum,
{
    let mut fishes = <[T; 9]>::from_iter(fishes.into_iter().map(T::from));
    for _ in 0..days {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }
    fishes.into_iter().sum()
}

pub fn part1(input: &Input) -> u32 {
    simulate_fishes(*input, 80)
}

pub fn part2(input: &Input) -> u64 {
    simulate_fishes(*input, 256)
}
