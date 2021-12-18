#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<SnailFish>>;

#[derive(Clone, Copy)]
pub struct SnailFish {
    depth: u8,
    value: u8,
}

fn eat<T: Copy>(input: &mut &[T]) -> T {
    let first = input[0];
    *input = &input[1..];
    first
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let input = &mut line.as_bytes();
            let mut output = Vec::new();
            let mut depth = 0;
            loop {
                match eat(input) {
                    b'[' => depth += 1,
                    b']' if depth == 1 => return output,
                    b']' => depth -= 1,
                    b @ b'0'..=b'9' => output.push(SnailFish {
                        depth,
                        value: b - b'0',
                    }),
                    b',' => {}
                    _ => panic!("Invalid character"),
                }
            }
        })
        .collect()
}

fn sum(n1: Vec<SnailFish>, n2: &[SnailFish]) -> Vec<SnailFish> {
    let mut acc = n1;

    acc.extend(n2.as_ref().iter().copied());
    acc.iter_mut().for_each(|sf| sf.depth += 1);

    loop {
        if let Some(pos) = acc.iter().position(|n| n.depth > 4) {
            (pos != 0).then(|| acc[pos - 1].value += acc[pos].value);
            (pos != acc.len() - 2).then(|| acc[pos + 2].value += acc[pos + 1].value);
            let depth = acc[pos].depth - 1;
            acc[pos] = SnailFish { depth, value: 0 };
            acc.remove(pos + 1);
            continue;
        }

        if let Some(pos) = acc.iter().position(|n| n.value >= 10) {
            let (l, r) = (acc[pos].value / 2, (acc[pos].value + 1) / 2);
            let depth = acc[pos].depth + 1;
            acc[pos] = SnailFish { depth, value: l };
            acc.insert(pos + 1, SnailFish { depth, value: r });
            continue;
        }

        break;
    }

    acc
}

fn magnitude(mut sfs: &[SnailFish]) -> u32 {
    fn helper(sfs: &mut &[SnailFish], depth: u8) -> u32 {
        if sfs[0].depth == depth {
            return eat(sfs).value as u32;
        }
        3 * helper(sfs, depth + 1) + 2 * helper(sfs, depth + 1)
    }
    helper(&mut sfs, 0)
}

pub fn part1(input: &Input) -> u32 {
    let sfs = input
        .iter()
        .skip(1)
        .fold(input[0].clone(), |acc, next| sum(acc, next));

    magnitude(&sfs)
}

pub fn part2(input: &Input) -> u32 {
    input
        .par_iter()
        .flat_map(|n1| input.par_iter().map(move |n2| (n1, n2)))
        .filter(|(n1, n2)| n1 as *const _ != n2 as *const _)
        .map(|(n1, n2)| magnitude(&sum(n1.to_vec(), n2)))
        .max()
        .unwrap()
}
