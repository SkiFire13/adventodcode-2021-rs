#[allow(unused_imports)]
use super::prelude::*;
type Input = Game;

pub struct Game {
    /// num (index in num_turn) -> turn (turn when num is picked)
    num_turn: Vec<u8>,
    grids: Vec<[[u8; 5]; 5]>,
}

pub fn input_generator(input: &str) -> Input {
    let (numbers, grids) = input.split_once("\n\n").expect("Invalid input");

    let numbers = numbers
        .split(',')
        .map(|n| n.parse().expect("Invalid input"))
        .collect::<Vec<u8>>();
    let mut num_turn = vec![0; numbers.len()];
    for (idx, n) in numbers.into_iter().enumerate() {
        num_turn[n as usize] = idx as u8;
    }

    let grids = grids
        .split("\n\n")
        .map(|grid| {
            <[_; 5]>::from_iter(grid.lines().map(|line| {
                <[_; 5]>::from_iter(
                    line.split_ascii_whitespace()
                        .map(|n| n.parse::<u8>().unwrap()),
                )
            }))
        })
        .collect();

    Game { num_turn, grids }
}

fn score_of_best_grid<K: Ord>(input: &Input, key: impl Fn(u8) -> K) -> u32 {
    let Game { num_turn, grids } = input;

    let (best_grid, best_grid_idx) = grids
        .iter()
        .map(|grid| {
            fn best_idx(num_turn: &[u8], f: impl Copy + Fn(usize, usize) -> u8) -> u8 {
                (0..5)
                    .map(|d1| (0..5).map(|d2| num_turn[f(d1, d2) as usize]).max().unwrap())
                    .min()
                    .unwrap()
            }
            let best_row_idx = best_idx(num_turn, |d1, d2| grid[d1][d2]);
            let best_col_idx = best_idx(num_turn, |d1, d2| grid[d2][d1]);
            (grid, min(best_row_idx, best_col_idx))
        })
        .min_by_key(|&(_, idx)| key(idx))
        .unwrap();

    let unmarked_sum = best_grid
        .iter()
        .flatten()
        .filter(|&&n| num_turn[n as usize] > best_grid_idx)
        .map(|&n| n as u32)
        .sum::<u32>();
    let last = num_turn
        .iter()
        .position(|&idx| idx == best_grid_idx)
        .unwrap() as u32;

    unmarked_sum * last
}

pub fn part1(input: &Input) -> u32 {
    score_of_best_grid(input, |idx| idx)
}

pub fn part2(input: &Input) -> u32 {
    score_of_best_grid(input, |idx| -(idx as i8))
}
