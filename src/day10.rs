#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<u8>>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|line| {
            let mut stack = Vec::new();
            for &c in line {
                match c {
                    c @ (b'(' | b'[' | b'{' | b'<') => stack.push(c),
                    b')' if stack.pop() != Some(b'(') => return 3,
                    b']' if stack.pop() != Some(b'[') => return 57,
                    b'}' if stack.pop() != Some(b'{') => return 1197,
                    b'>' if stack.pop() != Some(b'<') => return 25137,
                    _ => {}
                }
            }
            0
        })
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let mut scores = input
        .iter()
        .filter_map(|line| {
            let mut stack = Vec::new();
            for &c in line {
                match c {
                    c @ (b'(' | b'[' | b'{' | b'<') => stack.push(c),
                    b')' if stack.last() != Some(&b'(') => return None,
                    b']' if stack.last() != Some(&b'[') => return None,
                    b'}' if stack.last() != Some(&b'{') => return None,
                    b'>' if stack.last() != Some(&b'<') => return None,
                    _ => drop(stack.pop()),
                }
            }
            Some(stack.iter().rev().fold(0, |acc, b| {
                acc * 5
                    + [0, b'(', b'[', b'{', b'<']
                        .iter()
                        .position(|c| c == b)
                        .unwrap()
            }))
        })
        .filter(|&s| s != 0)
        .collect::<Vec<_>>();
    let middle = scores.len() / 2;
    *scores.select_nth_unstable(middle).1
}
