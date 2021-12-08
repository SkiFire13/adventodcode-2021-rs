#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Entry>;

pub struct Entry {
    digits: Vec<Digit>,
}

impl Entry {
    fn find(&self, f: impl Fn(Digit) -> bool) -> Digit {
        self.digits[..10].iter().copied().find(|&d| f(d)).unwrap()
    }
    fn out(&self) -> impl Iterator<Item = Digit> + '_ {
        self.digits[10..].iter().copied()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Digit(u8);

impl Digit {
    fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes.iter().fold(0, |acc, &b| acc | (1 << (b - b'a'))))
    }
    fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
    fn contains(&self, s: u8) -> bool {
        self.0 & s == s
    }
    fn find(&self, f: impl Fn(u8) -> bool) -> u8 {
        (0..7)
            .map(|shift| self.0 & (1 << shift))
            .find(|&s| s != 0 && f(s))
            .unwrap()
    }
    fn contains_all(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let digits = line
                .split_ascii_whitespace()
                .filter(|&s| s != "|")
                .map(|d| Digit::from_bytes(d.as_bytes()))
                .collect();
            Entry { digits }
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .flat_map(|e| e.out())
        .filter(|d| matches!(d.len(), 2 | 4 | 3 | 7))
        .count()
}

pub fn part2(input: &Input) -> usize {
    let all = Digit::from_bytes(b"abcdefg");

    input
        .iter()
        .map(|entry| {
            let d1 = entry.find(|d| d.len() == 2);
            let d4 = entry.find(|d| d.len() == 4);
            let d7 = entry.find(|d| d.len() == 3);
            let d6 = entry.find(|d| d.len() == 6 && !d.contains_all(d1));

            let sc = all.find(|s| !d6.contains(s));

            let d5 = entry.find(|d| d.len() == 5 && !d.contains(sc));

            let se = all.find(|s| !d5.contains(s) && s != sc);

            let d3 = entry.find(|d| d.len() == 5 && !d.contains(se) && d != d5);

            let sb = all.find(|s| !d3.contains(s) && s != se);
            let sd = d4.find(|s| !d1.contains(s) && s != sb);
            let sf = d1.find(|s| s != sc);

            let d0 = entry.find(|d| d.len() == 6 && !d.contains(sd));
            let d2 = entry.find(|d| d.len() == 5 && !d.contains(sb) && !d.contains(sf));
            let d8 = entry.find(|d| d.len() == 7);
            let d9 = entry.find(|d| d.len() == 6 && !d.contains(se));

            let ds = [d0, d1, d2, d3, d4, d5, d6, d7, d8, d9];

            entry
                .out()
                .map(|d| ds.iter().position(|&c| c == d).unwrap())
                .fold(0, |acc, d| acc * 10 + d)
        })
        .sum()
}
