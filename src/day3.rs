#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u16>;

const NUM_LEN: usize = 12;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .fold(0, |acc, c| (acc << 1) + (c == '1') as u16)
        })
        .collect_vec()
}

fn get_bit(n: u16, pos: usize) -> bool {
    (n & (1 << (NUM_LEN - 1 - pos))) != 0
}

pub fn part1(input: &Input) -> u32 {
    let mut freqs = vec![[0; 2]; NUM_LEN];
    for &w in input {
        for (i, freq) in freqs.iter_mut().enumerate() {
            freq[get_bit(w, i) as usize] += 1;
        }
    }
    let gamma = freqs
        .iter()
        .fold(0, |acc, [z, u]| (acc << 1) + (u > z) as u32);
    let eps = (1 << NUM_LEN) - 1 - gamma;
    gamma * eps
}

pub fn part2(input: &Input) -> u32 {
    fn find_rating(input: &Input, select: impl Fn(u16, u16) -> bool) -> u32 {
        let mut nums = input.clone();
        let mut pos = 0;
        while nums.len() > 1 {
            let mut freq = [0; 2];
            for &c in &nums {
                freq[get_bit(c, pos) as usize] += 1;
            }
            nums.retain(|&n| get_bit(n, pos) == select(freq[0], freq[1]));
            pos += 1;
        }
        nums[0] as u32
    }

    let oxy = find_rating(input, |z, u| u >= z);
    let co2 = find_rating(input, |z, u| u < z);

    oxy * co2
}
