#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| match c {
        '.' => 0,
        '>' => 1,
        'v' => 2,
        _ => panic!(),
    })
}

pub fn part1(input: &Input) -> usize {
    let mut lookup = [0; 1 << 16];
    for idx in 0..(1 << 16) {
        let d = (idx >> 14) & 0b11;
        let g = (idx >> 12) & 0b11;
        let b = (idx >> 10) & 0b11;
        let e = (idx >> 8) & 0b11;
        let h = (idx >> 6) & 0b11;
        let f = (idx >> 2) & 0b11;
        let i = (idx >> 0) & 0b11;

        lookup[idx] = match e {
            0 if d == 1 => 1,
            0 if b == 2 => 2,
            0 => 0,
            1 if f != 0 => 1,
            1 if b == 2 => 2,
            1 => 0,
            2 if h == 0 && g != 1 => 0,
            2 if h == 1 && i == 0 => 0,
            2 => 2,
            _ => 0,
        };
    }

    let mut curr = input.vec.clone();
    let mut next = vec![0; curr.len()];

    let (xlen, ylen) = (input.w(), input.h());
    let mut i = 0;
    let mut moved = true;
    while moved {
        moved = next
            .par_chunks_exact_mut(xlen)
            .enumerate()
            .map(|(y, line)| {
                let mut moved = false;

                let py = if y > 0 { y - 1 } else { ylen - 1 };
                let ny = if y + 1 < ylen { y + 1 } else { 0 };

                let b = curr[0 + py * xlen] as u16;
                let c = curr[1 + py * xlen] as u16;
                let d = curr[xlen - 1 + y * xlen] as u16;
                let e = curr[0 + y * xlen] as u16;
                let f = curr[1 + y * xlen] as u16;
                let g = curr[xlen - 1 + ny * xlen] as u16;
                let h = curr[0 + ny * xlen] as u16;
                let i = curr[1 + ny * xlen] as u16;

                let mut lookup_idx = (d << 14)
                    | (g << 12)
                    | (b << 10)
                    | (e << 8)
                    | (h << 6)
                    | (c << 4)
                    | (f << 2)
                    | (i << 0);

                for x in 0..xlen {
                    line[x] = lookup[lookup_idx as usize];
                    moved |= line[x] != curr[x + y * xlen];

                    let nx = if x + 2 < xlen { x + 2 } else { x + 2 - xlen };
                    let c = curr[nx + py * xlen] as u16;
                    let f = curr[nx + y * xlen] as u16;
                    let i = curr[nx + ny * xlen] as u16;
                    lookup_idx = (lookup_idx << 6) | (c << 4) | (f << 2) | (i << 0);
                }

                moved
            })
            .reduce(|| false, |a, b| a | b);

        swap(&mut curr, &mut next);
        i += 1;
    }
    i
}
