#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<bool>>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '1').collect_vec())
        .collect_vec()
}

pub fn part1(input: &Input) -> u32 {
    let mut freq = vec![(0, 0); input[0].len()];
    for w in input {
        for (i, &c) in w.iter().enumerate() {
            if c {
                freq[i].1 += 1;
            } else {
                freq[i].0 += 1;
            }
        }
    }
    let gamma = freq.iter().fold(0, |acc, (z, u)| {
        (acc << 1) + (u > z) as u32
    });
    let eps = freq
        .iter()
        .fold(0, |acc, (z, u)| (acc << 1) + (u < z) as u32);
    gamma * eps
}

pub fn part2(input: &Input) -> u32 {
    let mut nums = input.clone();
    let mut pos = 0;
    while nums.len() > 1 {
        let mut freq = (0, 0);
        for c in &nums {
            if c[pos] {
                freq.1 += 1;
            } else {
                freq.0 += 1;
            }
        }

        nums.retain(|n| n[pos] == (freq.1 >= freq.0));
        pos += 1;
    }
    let oxy = nums[0].iter().fold(0, |acc, d| (acc << 1) + *d as u32);

    let mut nums = input.clone();
    let mut pos = 0;
    while nums.len() > 1 {
        let mut freq = (0, 0);
        for c in &nums {
            if c[pos] {
                freq.1 += 1;
            } else {
                freq.0 += 1;
            }
        }

        nums.retain(|n| n[pos] == (freq.1 < freq.0));
        pos += 1;
    }
    let co2 = nums[0].iter().fold(0, |acc, d| (acc << 1) + *d as u32);

    oxy * co2
}
