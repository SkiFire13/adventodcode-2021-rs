#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Entry>;

pub struct Entry {
    digits: [Digit; 10],
    output: [Digit; 4],
}

impl Entry {
    fn find(&self, f: impl Fn(Digit) -> bool) -> Digit {
        self.digits.into_iter().find(|&d| f(d)).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Digit(u8);

impl Digit {
    fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
    fn diff(&self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (digits, output) = line.split_once(" | ").expect("Invalid input");

            fn parse(line: &str) -> impl Iterator<Item = Digit> + '_ {
                line.split_ascii_whitespace()
                    .filter(|&s| s != "|")
                    .map(|d| Digit(d.bytes().fold(0, |acc, b| acc | (1 << (b - b'a')))))
            }

            Entry {
                digits: <[_; 10]>::from_iter(parse(digits)),
                output: <[_; 4]>::from_iter(parse(output)),
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .flat_map(|e| e.output)
        .filter(|d| matches!(d.len(), 2 | 4 | 3 | 7))
        .count()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|entry| {
            let d1 = entry.find(|d| d.len() == 2);
            let d4 = entry.find(|d| d.len() == 4);
            let d7 = entry.find(|d| d.len() == 3);
            let d8 = entry.find(|d| d.len() == 7);

            let d3 = entry.find(|d| d.diff(d1).len() == 3);
            let d6 = entry.find(|d| d.diff(d1).len() == 5 && d != d8);

            let d0 = entry.find(|d| d.len() == 6 && d.diff(d4).len() == 3 && d != d6);
            let d2 = entry.find(|d| d.len() == 5 && d.diff(d4).len() == 3);
            let d5 = entry.find(|d| d.len() == 5 && d.diff(d4).len() == 2 && d != d3);
            let d9 = entry.find(|d| d.len() == 6 && d.diff(d4).len() == 2);

            let ds = [d0, d1, d2, d3, d4, d5, d6, d7, d8, d9];

            entry
                .output
                .iter()
                .map(|d| ds.iter().position(|c| c == d).unwrap())
                .fold(0, |acc, d| acc * 10 + d)
        })
        .sum()
}
