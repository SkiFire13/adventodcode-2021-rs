#[allow(unused_imports)]
use super::prelude::*;
type Input = (u32, u32);

pub fn input_generator(input: &str) -> Input {
    let (p1, p2) = input.split_once('\n').unwrap();
    (p1[28..].parse().unwrap(), p2[28..].parse().unwrap())
}

pub fn part1(input: &Input) -> u32 {
    let (mut p1pos, mut p2pos) = *input;
    let (mut p1score, mut p2score) = (0, 0);
    let mut next_roll = 1;
    let mut rolls = 0;

    loop {
        p1pos = (p1pos + (3 * next_roll + 3) - 1) % 10 + 1;
        p1score += p1pos;
        next_roll = (next_roll + 3) % 10;
        rolls += 3;
        if p1score >= 1000 {
            return p2score * rolls;
        }

        p2pos = (p2pos + (3 * next_roll + 3) - 1) % 10 + 1;
        p2score += p2pos;
        next_roll = (next_roll + 3) % 10;
        rolls += 3;
        if p2score >= 1000 {
            return p1score * rolls;
        }
    }
}

pub fn part2(input: &Input) -> u64 {
    let moves = itertools::iproduct!(1..=3, 1..=3, 1..=3)
        .map(|(m1, m2, m3)| m1 + m2 + m3)
        .counts()
        .into_iter()
        .map(|(m, n)| (m as u8, n as u8))
        .collect::<Vec<_>>();
    let mut states = FxHashMap::from_iter([((input.0 as u8, input.1 as u8, 0u8, 0u8, 1u8), 1)]);
    let mut new_states = FxHashMap::default();
    let (mut p1win, mut p2win) = (0, 0);

    while !states.is_empty() {
        'inner: for ((p1pos, p2pos, p1score, p2score, turn), n) in states.drain() {
            if p1score >= 21 {
                p1win += n;
                continue 'inner;
            }
            if p2score >= 21 {
                p2win += n;
                continue 'inner;
            }

            if turn == 1 {
                for &(mov, movn) in &moves {
                    let p1pos = (p1pos + mov - 1) % 10 + 1;
                    let p1score = p1score + p1pos;
                    *new_states
                        .entry((p1pos, p2pos, p1score, p2score, 2))
                        .or_default() += n * movn as u64;
                }
            } else {
                for &(mov, movn) in &moves {
                    let p2pos = (p2pos + mov - 1) % 10 + 1;
                    let p2score = p2score + p2pos;
                    *new_states
                        .entry((p1pos, p2pos, p1score, p2score, 1))
                        .or_default() += n * movn as u64;
                }
            }
        }
        swap(&mut states, &mut new_states)
    }

    max(p1win, p2win)
}
