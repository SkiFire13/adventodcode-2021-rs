#[allow(unused_imports)]
use super::prelude::*;
type Input = [Operation; 14];

#[derive(Copy, Clone, Debug)]
pub enum Operation {
    Push(i64),
    Pop(i64),
}

pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    [(); 14].map(|_| {
        let mod_op = lines.nth(4).unwrap();
        if &mod_op[6..] == "1" {
            let push_param = lines.nth(10).unwrap();
            let _ = lines.nth(1);
            Operation::Push(push_param[6..].parse().unwrap())
        } else {
            let pop_param = lines.nth(0).unwrap();
            let _ = lines.nth(11);
            Operation::Pop(pop_param[6..].parse().unwrap())
        }
    })
}

fn solve(operations: &[Operation; 14], f: impl Fn(i64) -> i64) -> u64 {
    let mut digits = [0; 14];
    let mut stack = ArrayVec::<_, 7>::new();

    for (idx, &operation) in operations.iter().enumerate() {
        match operation {
            Operation::Push(n) => stack.push((idx, n)),
            Operation::Pop(n) => {
                let (prev_idx, prev_n) = stack.pop().unwrap();
                let diff = prev_n + n;
                let d = f(diff);
                digits[prev_idx] = (d - diff) as u64;
                digits[idx] = d as u64;
            }
        }
    }

    digits.into_iter().fold(0, |acc, d| acc * 10 + d)
}

pub fn part1(input: &Input) -> u64 {
    solve(input, |diff| min(9 + diff, 9))
}

pub fn part2(input: &Input) -> u64 {
    solve(input, |diff| max(1 + diff, 1))
}
