#[allow(unused_imports)]
use super::prelude::*;
type Input = Game;

pub struct Game {
    numbers: Vec<u8>,
    grids: HashMap<(u8, u8, u8), u8>,
}

pub fn input_generator(input: &str) -> Input {
    let mut lines = input.split("\n\n");
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let grids = lines
        .enumerate()
        .flat_map(|(g, grid)| {
            grid.lines().enumerate().flat_map(move |(y, line)| {
                line.trim().split_ascii_whitespace()
                    .enumerate()
                    .map(move |(x, n)| ((g as u8, y as u8, x as u8), n.parse::<u8>().unwrap()))
            })
        })
        .collect::<HashMap<_, _>>();

    Game {
        numbers,
        grids,
    }
}

pub fn part1(input: &Input) -> u32 {
    let nums_rel = input.numbers.iter().enumerate().map(|(idx, &n)| (n, idx)).collect::<HashMap<_,_>>();
    let mut g = 0;
    let mut min_g = 0;
    let mut min_g_idx = usize::MAX;
    while input.grids.contains_key(&(g, 0, 0)) {
        for y in 0..5 {
            let max_idx_col = (0..5).map(|x| nums_rel[&input.grids[&(g, y, x)]]).max().unwrap();
            if max_idx_col < min_g_idx {
                min_g_idx = max_idx_col;
                min_g = g;
            }
        }
        for x in 0..5 {
            let max_idx_row = (0..5).map(|y| nums_rel[&input.grids[&(g, y, x)]]).max().unwrap();
            if max_idx_row < min_g_idx {
                min_g_idx = max_idx_row;
                min_g = g;
            }
        }
        g += 1;
    }

    let last = input.numbers[min_g_idx];
    input
        .grids
        .iter()
        .filter(|&(pos, n)| pos.0 == min_g && nums_rel[n] > min_g_idx)
        .map(|(_, &n)| n as u32)
        .sum::<u32>()
        * last as u32
}

pub fn part2(input: &Input) -> u32 {
    let nums_rel = input.numbers.iter().enumerate().map(|(idx, &n)| (n, idx)).collect::<HashMap<_,_>>();
    let mut g = 0;
    let mut max_g = 0;
    let mut max_g_idx = 0;
    while input.grids.contains_key(&(g, 0, 0)) {
        let mut min_max_idx = usize::MAX;
        for y in 0..5 {
            let max_idx_col = (0..5).map(|x| nums_rel[&input.grids[&(g, y, x)]]).max().unwrap();
            if max_idx_col < min_max_idx {
                min_max_idx = max_idx_col;
            }
        }
        for x in 0..5 {
            let max_idx_row = (0..5).map(|y| nums_rel[&input.grids[&(g, y, x)]]).max().unwrap();
            if max_idx_row < min_max_idx {
                min_max_idx = max_idx_row;
            }
        }

        if min_max_idx > max_g_idx {
            max_g_idx = min_max_idx;
            max_g = g;
        }

        g += 1;
    }

    let last = input.numbers[max_g_idx];
    input
        .grids
        .iter()
        .filter(|&(pos, n)| pos.0 == max_g && nums_rel[n] > max_g_idx)
        .map(|(_, &n)| n as u32)
        .sum::<u32>()
        * last as u32
}
