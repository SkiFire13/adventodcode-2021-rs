#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u16>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u16 - b'0' as u16)
}

fn best_path(input: &Input) -> u16 {
    let mut queue = BinaryHeap::from([Reverse((0, input.width as u16 - 1, input.height() as u16 - 1))]);
    let mut seen = Grid {
        vec: vec![false; input.vec.len()],
        width: input.width,
    };

    while let Some(Reverse((risk, x, y))) = queue.pop() {
        let (x, y) = (x as usize, y as usize);
        if (x, y) == (0, 0) {
            return risk;
        }
        if !std::mem::replace(&mut seen[(x, y)], true) {
            let risk = risk + input[(x, y)];
            queue.extend(
                input
                    .plus_neighbours((x, y))
                    .filter(|&(nx, ny)| !seen[(nx, ny)])
                    .map(|(nx, ny)| Reverse((risk, nx as u16, ny as u16))),
            );
        }
    }

    panic!("No path found");
}

pub fn part1(input: &Input) -> u16 {
    best_path(input)
}

pub fn part2(input: &Input) -> u16 {
    let mut vec = Vec::with_capacity(input.vec.len() * 25);
    for my in 0..5 {
        for y in 0..input.height() {
            for mx in 0..5 {
                vec.extend(
                    input.vec[input.width * y..][..input.width]
                        .iter()
                        .map(|&r| (r + mx + my - 1) % 9 + 1),
                )
            }
        }
    }
    let input = Grid {
        vec,
        width: input.width * 5,
    };

    best_path(&input)
}
