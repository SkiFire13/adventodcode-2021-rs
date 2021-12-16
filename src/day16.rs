#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u8>;

pub fn input_generator(input: &str) -> Input {
    input
        .bytes()
        .map(|c| match c {
            b'0'..=b'9' => c - b'0',
            b'A'..=b'F' => c - b'A' + 10,
            _ => panic!(),
        })
        .flat_map(|b| [b >> 3, b >> 2, b >> 1, b])
        .map(|b| b & 1)
        .collect()
}

fn read_n(data: &mut &[u8], n: usize) -> u64 {
    let value = data[..n].iter().fold(0, |acc, &b| (acc << 1) | b as u64);
    *data = &data[n..];
    value
}

fn read_literal(data: &mut &[u8]) -> u64 {
    let mut lit = 0;
    let mut last_read = false;
    while !last_read {
        last_read = read_n(data, 1) == 0;
        lit = (lit << 4) | read_n(data, 4);
    }
    lit
}

// Workaround for an `impl Trait` bug
trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

fn read_n_packets<'m, 'd, T>(
    data: &'m mut &'d [u8],
    read: impl Fn(&mut &[u8]) -> T + 'm,
) -> impl Iterator<Item = T> + Captures<'m> + Captures<'d> {
    if read_n(data, 1) == 0 {
        let length = read_n(data, 15) as usize;
        let initial_len = data.len();
        Either::Left(iter::from_fn(move || {
            (initial_len - data.len() < length).then(|| read(data))
        }))
    } else {
        let num_packets = read_n(data, 11);
        Either::Right((0..num_packets).map(move |_| read(data)))
    }
}

pub fn part1(input: &Input) -> u64 {
    fn read_packet(data: &mut &[u8]) -> u64 {
        if data.len() < 10 {
            return 0;
        }
        let version = read_n(data, 3);
        let ty = read_n(data, 3);
        if ty == 4 {
            let _literal = read_literal(data);
            version
        } else {
            read_n_packets(data, read_packet).sum::<u64>()
        }
    }
    read_packet(&mut &**input)
}

pub fn part2(input: &Input) -> u64 {
    fn read_packet(data: &mut &[u8]) -> u64 {
        if data.len() < 10 {
            return 0;
        }
        let _version = read_n(data, 3);
        let ty = read_n(data, 3);

        if ty == 4 {
            read_literal(data)
        } else {
            let mut iter = read_n_packets(data, read_packet);
            match ty {
                0 => iter.sum(),
                1 => iter.product(),
                2 => iter.min().unwrap(),
                3 => iter.max().unwrap(),
                5 => (iter.next().unwrap() > iter.next().unwrap()) as u64,
                6 => (iter.next().unwrap() < iter.next().unwrap()) as u64,
                7 => (iter.next().unwrap() == iter.next().unwrap()) as u64,
                _ => panic!(),
            }
        }
    }
    read_packet(&mut &**input)
}
