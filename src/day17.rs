#[allow(unused_imports)]
use super::prelude::*;
type Input = (i32, i32, i32, i32);

pub fn input_generator(input: &str) -> Input {
    let (x1, rest) = input[15..].split_once("..").unwrap();
    let (x2, rest) = rest.split_once(", y=").unwrap();
    let (y1, y2) = rest.split_once("..").unwrap();
    (x1.parse().unwrap(), x2.parse().unwrap(), y1.parse().unwrap(), y2.parse().unwrap())
}

pub fn part1(input: &Input) -> i32 {
    let &(_x1, _x2, y1, _y2) = input;
    -y1 * (-y1-1) / 2
}

pub fn part2(input: &Input) -> usize {
    let &(x1, x2, y1, y2) = input;
    let mut count  = 0;
    for vy in y1..=-y1 {
        for vx in 0..=x2 {
            let (mut vx, mut vy) = (vx, vy);
            let (mut x, mut y) = (0, 0);
            while x + vx <= x2 && y + vy >= y1 {
                x += vx; y += vy;
                vx -= num::signum(vx); vy -= 1;
            }
            if x >= x1 && y <= y2 {
                count += 1;
            }
        }
    }
    count
}
