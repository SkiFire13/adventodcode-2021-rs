#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8 - b'0')
}

pub fn part1(input: &Input) -> u16 {
    let mut risk = 0;
    for y in 0..input.height() {
        for x in 0..input.width {
            let p = input[(x, y)];
            if [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(dx, dy)| input.iget((x as isize + dx, y as isize + dy)))
                .all(|&n| n > p)
            {
                risk += 1 + p as u16;
            }
        }
    }
    risk
}

pub fn part2(input: &Input) -> u16 {
    type Point = (usize, usize);

    #[derive(Clone, Copy)]
    enum Basin {
        Root(u16),
        Link((u8, u8)),
    }

    fn root(basins: &Grid<Basin>, p: Point) -> (Point, u16) {
        match basins[p] {
            Basin::Root(c) => (p, c),
            Basin::Link(p) => root(basins, (p.0 as usize, p.1 as usize)),
        }
    }

    fn union(basins: &mut Grid<Basin>, p1: Point, p2: Point) {
        let (root1, count1) = root(&basins, p1);
        let (root2, count2) = root(&basins, p2);
        if root1 != root2 {
            basins[root1] = Basin::Root(count1 + count2);
            basins[root2] = Basin::Link((root1.0 as u8, root1.1 as u8));
        }
    }

    let mut basins = Grid {
        vec: Vec::with_capacity(input.vec.len()),
        width: input.width,
    };

    for y in 0..input.height() {
        for x in 0..input.width {
            basins.vec.push(Basin::Root(1));
            if input[(x, y)] != 9 {
                (x > 0 && input[(x - 1, y)] != 9).then(|| union(&mut basins, (x - 1, y), (x, y)));
                (y > 0 && input[(x, y - 1)] != 9).then(|| union(&mut basins, (x, y - 1), (x, y)));
            }
        }
    }

    basins
        .vec
        .into_iter()
        .filter_map(|basin| match basin {
            Basin::Root(count) => Some(-(count as i16)),
            Basin::Link(_) => None,
        })
        .k_smallest(3)
        .fold(1, |acc, c| acc * (-c as u16))
}
