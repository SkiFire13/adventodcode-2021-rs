#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = Vec<u16>;

pub fn input_generator(input: &str) -> Input {
    let mut map = HashMap::from([("start", 0), ("end", 1)]);
    let mut edges = vec![1 << 15; 2];
    for line in input.lines() {
        let (n1, n2) = line.split_once('-').unwrap();
        let mut idx_entry = |n| *map.entry(n).or_insert_with(|| {
            edges.push((n.as_bytes()[0].is_ascii_lowercase() as u16) << 15);
            edges.len() - 1
        });
        let idx1 = idx_entry(n1);
        let idx2 = idx_entry(n2);
        edges[idx1] |= 1 << idx2;
        edges[idx2] |= 1 << idx1;
    }
    edges
}

pub fn paths_num<'a>(
    start: usize,
    input: &Input<'a>,
    visited: u16,
    allow_twice: bool,
) -> usize {
    if start == 1 {
        return 1;
    }
    let mut num = 0;
    let neighbours = input[start];
    for dest in (0..15).filter(|&idx| neighbours & (1 << idx) != 0) {
        if visited & (1 << dest) == 0 || input[dest] & (1 << 15) == 0 {
            num += paths_num(dest, input, visited | (1 << dest), allow_twice);
        } else if allow_twice == true && dest != 0 {
            num += paths_num(dest, input, visited, false);
        }
    }
    num
}

pub fn part1(input: &Input) -> usize {
    paths_num(0, input, 1, false)
}

pub fn part2(input: &Input) -> usize {
    paths_num(0, input, 1, true)
}
