#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<(usize, usize)>, Vec<Fold>);

#[derive(Clone, Copy)]
pub enum Fold {
    X(usize),
    Y(usize),
}

pub fn input_generator(input: &str) -> Input {
    let (points, folds) = input.split_once("\n\n").unwrap();
    let points = points
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let folds = folds
        .lines()
        .map(|fold| {
            let n = fold[13..].parse().unwrap();
            match fold.as_bytes()[11] {
                b'x' => Fold::X(n),
                b'y' => Fold::Y(n),
                _ => panic!(),
            }
        })
        .collect();

    (points, folds)
}

pub fn fold_paper(points: &mut [(usize, usize)], fold: Fold) {
    match fold {
        Fold::X(n) => {
            for p in points.iter_mut() {
                if p.0 > n {
                    p.0 = 2 * n - p.0;
                }
            }
        }
        Fold::Y(n) => {
            for p in points.iter_mut() {
                if p.1 > n {
                    p.1 = 2 * n - p.1;
                }
            }
        }
    }
}

pub fn part1(input: &Input) -> usize {
    let (points, folds) = input;
    let mut points = points.clone();

    fold_paper(&mut points, folds[0]);

    points.into_iter().unique().count()
}

pub fn part2(input: &Input) -> String {
    let (points, folds) = input;
    let mut points = points.clone();

    folds.iter().for_each(|&fold| fold_paper(&mut points, fold));

    let maxx = points.iter().map(|&(x, _)| x).max().unwrap();
    let maxy = points.iter().map(|&(_, y)| y).max().unwrap();

    let mut grid = vec![b'.'; (maxx + 2) * (maxy + 1)];
    for p in points {
        grid[1 + p.0 + p.1 * (maxx + 2)] = b'#';
    }
    for y in 0..=maxy {
        grid[y * (maxx + 2)] = b'\n';
    }
    String::from_utf8(grid).unwrap()
}
