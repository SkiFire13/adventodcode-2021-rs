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
    fn to_key(p1pos: u8, p2pos: u8, p1score: u8, p2score: u8, turn: u8) -> u32 {
        p1pos as u32
            + p2pos as u32 * 10 
            + p1score as u32 * 10 * 10
            + p2score as u32 * 10 * 10 * 21
            + (turn as u32 - 1) * 10 * 10 * 21 * 21
    }

    fn helper(
        p1pos: u8,
        p2pos: u8,
        p1score: u8,
        p2score: u8,
        turn: u8,
        moves: &[(u8, u8)],
        cache: &mut [(u64, u64)],
    ) -> (u64, u64) {
        if p1score >= 21 {
            return (1, 0);
        }
        if p2score >= 21 {
            return (0, 1);
        }

        let cached = cache[to_key(p1pos, p2pos, p1score, p2score, turn) as usize];
        if cached != (0, 0) {
            return cached;
        }

        let (mut p1win, mut p2win) = (0, 0);

        if turn == 1 {
            for &(mov, n) in moves {
                let p1pos = (p1pos + mov) % 10;
                let p1score = p1score + p1pos + 1;
                let (newp1win, newp2win) = helper(p1pos, p2pos, p1score, p2score, 2, moves, cache);
                p1win += newp1win * n as u64;
                p2win += newp2win * n as u64;
            }
        } else {
            for &(mov, n) in moves {
                let p2pos = (p2pos + mov) % 10;
                let p2score = p2score + p2pos + 1;
                let (newp1win, newp2win) = helper(p1pos, p2pos, p1score, p2score, 1, moves, cache);
                p1win += newp1win * n as u64;
                p2win += newp2win * n as u64;
            }
        }

        cache[to_key(p1pos, p2pos, p1score, p2score, turn) as usize] = (p1win, p2win);
        (p1win, p2win)
    }

    let moves = itertools::iproduct!(1..=3, 1..=3, 1..=3)
        .map(|(m1, m2, m3)| m1 + m2 + m3)
        .counts()
        .into_iter()
        .map(|(m, n)| (m as u8, n as u8))
        .collect::<Vec<_>>();
    let (p1win, p2win) = helper(
        input.0 as u8 - 1,
        input.1 as u8 - 1,
        0,
        0,
        1,
        &moves,
        &mut vec![(0, 0); 10 * 10 * 21 * 21 * 2],
    );
    max(p1win, p2win)
}
