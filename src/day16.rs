#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u8>;

pub fn input_generator(input: &str) -> Input {
    input
        .bytes()
        .flat_map(|b| match b {
            b'0' => [0, 0, 0, 0],
            b'1' => [0, 0, 0, 1],
            b'2' => [0, 0, 1, 0],
            b'3' => [0, 0, 1, 1],
            b'4' => [0, 1, 0, 0],
            b'5' => [0, 1, 0, 1],
            b'6' => [0, 1, 1, 0],
            b'7' => [0, 1, 1, 1],
            b'8' => [1, 0, 0, 0],
            b'9' => [1, 0, 0, 1],
            b'A' => [1, 0, 1, 0],
            b'B' => [1, 0, 1, 1],
            b'C' => [1, 1, 0, 0],
            b'D' => [1, 1, 0, 1],
            b'E' => [1, 1, 1, 0],
            b'F' => [1, 1, 1, 1],
            _ => panic!(),
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    fn parse_packet(data: &mut &[u8]) -> usize {
        if data.len() < 10 {
            return 0;
        }
        let version = ((data[0] << 2) | (data[1] << 1) | data[2]) as usize;
        let ty = (data[3] << 2) | (data[4] << 1) | data[5];
        *data = &data[6..];
        match ty {
            4 => {
                while data[0] == 1 {
                    *data = &data[5..];
                }
                *data = &data[5..];
                version
            }
            _ if data[0] == 0 => {
                let length = data[1..16]
                    .iter()
                    .fold(0, |acc, &b| (acc << 1) | b as usize);
                *data = &data[16..];
                let mut sum = version;
                let mut inner_data = &data[..length];
                while !inner_data.is_empty() {
                    sum += parse_packet(&mut inner_data);
                }
                *data = &data[length..];
                sum
            }
            _ => {
                let subpackets = data[1..12]
                    .iter()
                    .fold(0, |acc, &b| (acc << 1) | b as usize);
                *data = &data[12..];
                let mut sum = version;
                for _ in 0..subpackets {
                    sum += parse_packet(data);
                }
                sum
            }
        }
    }
    let mut data = &**input;
    parse_packet(&mut data)
}

pub fn part2(input: &Input) -> usize {
    fn parse_packet(data: &mut &[u8]) -> usize {
        if data.len() < 10 {
            return 0;
        }
        //let version = ((data[0] << 2) | (data[1] << 1) | data[2]) as usize;
        let ty = (data[3] << 2) | (data[4] << 1) | data[5];
        *data = &data[6..];

        if ty == 4 {
            let mut sum = 0;
            loop {
                sum <<= 4;
                sum |= data[1..5].iter().fold(0, |acc, &b| (acc << 1) | b as usize);
                let exit = data[0] == 0;
                *data = &data[5..];
                if exit {
                    break;
                }
            }
            return sum;
        }

        let packets = if data[0] == 0 {
            let length = data[1..16]
                .iter()
                .fold(0, |acc, &b| (acc << 1) | b as usize);
            *data = &data[16..];
            let mut packets = Vec::new();
            let mut inner_data = &data[..length];
            while !inner_data.is_empty() {
                packets.push(parse_packet(&mut inner_data));
            }
            *data = &data[length..];
            packets
        } else {
            let subpackets = data[1..12]
                .iter()
                .fold(0, |acc, &b| (acc << 1) | b as usize);
            let mut packets = Vec::new();
            *data = &data[12..];
            for _ in 0..subpackets {
                packets.push(parse_packet(data));
            }
            packets
        };

        match ty {
            0 => packets.iter().sum(),
            1 => packets.iter().product(),
            2 => packets.iter().copied().min().unwrap(),
            3 => packets.iter().copied().max().unwrap(),
            5 => {
                if packets[0] > packets[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if packets[0] < packets[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if packets[0] == packets[1] {
                    1
                } else {
                    0
                }
            }
            _ => panic!(),
        }
    }
    let mut data = &**input;
    parse_packet(&mut data)
}
