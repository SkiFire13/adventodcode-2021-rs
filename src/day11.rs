#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8 - b'0')
}

fn step(grid: &mut Grid<u8>, queue: &mut Vec<(usize, usize)>) -> usize {
    for (y, x) in itertools::iproduct!(0..grid.height(), 0..grid.width) {
        queue.push((x, y));
        while let Some((x, y)) = queue.pop() {
            (grid[(x, y)] == 9).then(|| queue.extend(grid.square_neighbours((x, y))));
            grid[(x, y)] += 1;
        }
    }

    grid.vec
        .iter_mut()
        .filter(|cell| **cell > 9)
        .map(|cell| *cell = 0)
        .count()
}

pub fn part1(input: &Input) -> usize {
    let mut grid = input.clone();
    let mut queue = Vec::new();

    (0..100).map(|_| step(&mut grid, &mut queue)).sum()
}

pub fn part2(input: &Input) -> usize {
    let mut grid = input.clone();
    let mut queue = Vec::new();

    (1..)
        .find(|_| step(&mut grid, &mut queue) == grid.vec.len())
        .unwrap()
}
