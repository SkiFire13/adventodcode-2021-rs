#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Line>;

#[derive(Clone, Copy)]
pub struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once(" -> ").expect("Invalid input");
            let (x1, y1) = p1.split_once(',').expect("Invalid input");
            let (x2, y2) = p2.split_once(',').expect("Invalid input");
            Line {
                x1: x1.parse().expect("Invalid input"),
                y1: y1.parse().expect("Invalid input"),
                x2: x2.parse().expect("Invalid input"),
                y2: y2.parse().expect("Invalid input"),
            }
        })
        .collect()
}

fn find_intersections(input: &[Line], filter: impl Fn(isize, isize) -> bool) -> usize {
    let maxx = input.iter().flat_map(|l| [l.x1, l.x2]).max().unwrap() as usize + 1;
    let maxy = input.iter().flat_map(|l| [l.y1, l.y2]).max().unwrap() as usize + 1;
    let mut grid = vec![0u8; maxx * maxy];

    for &l in input {
        let dx = isize::signum(l.x2 as isize - l.x1 as isize);
        let dy = isize::signum(l.y2 as isize - l.y1 as isize);

        if !filter(dx, dy) {
            continue;
        }

        let mut x = l.x1 as isize;
        let mut y = l.y1 as isize;
        while x != l.x2 as isize || y != l.y2 as isize {
            grid[y as usize * maxx + x as usize] += 1;
            x += dx;
            y += dy;
        }
        grid[y as usize * maxx + x as usize] += 1;
    }

    grid.iter().filter(|&&c| c > 1).count()
}

pub fn part1(input: &Input) -> usize {
    find_intersections(input, |dx, dy| dx == 0 || dy == 0)
}

pub fn part2(input: &Input) -> usize {
    find_intersections(input, |_, _| true)
}
