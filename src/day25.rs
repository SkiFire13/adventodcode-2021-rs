#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

pub fn part1(input: &Input) -> usize {
    let mut grid = input.clone();
    let (w, h) = (grid.w(), grid.h());
    let mut i = 0;
    let mut moved = true;
    while moved {
        let mut next = grid.clone();

        moved = false;
        for (y, x) in itertools::iproduct!(0..grid.h(), 0..grid.w()) {
            if grid[(x, y)] == b'>' {
                if grid[((x + 1) % w, y)] == b'.' {
                    next[(x, y)] = b'.';
                    next[((x + 1) % w, y)] = b'>';
                    moved = true;
                }
            }
        }
        grid = next;
        let mut next = grid.clone();

        for (x, y) in itertools::iproduct!(0..grid.w(), 0..grid.h()) {
            if grid[(x, y)] == b'v' {
                if grid[(x, (y + 1) % h)] == b'.' {
                    next[(x, y)] = b'.';
                    next[(x, (y + 1) % h)] = b'v';
                    moved = true;
                }
            }
        }
        grid = next;
        i += 1;
    }
    i
}
