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
        moved = false;
        for y in 0..h {
            let first_occupied = grid[(0, y)] != b'.';
            let mut x = 0;
            while x < w - 1 {
                if grid[(x, y)] == b'>' && grid[(x + 1, y)] == b'.' {
                    grid[(x, y)] = b'.';
                    grid[(x + 1, y)] = b'>';
                    x += 2;
                    moved = true;
                } else {
                    x += 1;
                }
            }
            if x == w - 1 && grid[(w-1, y)] == b'>' && !first_occupied {
                grid[(w-1, y)] = b'.';
                grid[(0, y)] = b'>';
                moved = true;
            }
        }
        for x in 0..w {
            let first_occupied = grid[(x, 0)] != b'.';
            let mut y = 0;
            while y < h - 1 {
                if grid[(x, y)] == b'v' && grid[(x, y + 1)] == b'.' {
                    grid[(x, y)] = b'.';
                    grid[(x, y + 1)] = b'v';
                    y += 2;
                    moved = true;
                } else {
                    y += 1;
                }
            }
            if y == h - 1 && grid[(x, h-1)] == b'v' && !first_occupied {
                grid[(x, h-1)] = b'.';
                grid[(x, 0)] = b'v';
                moved = true;
            }
        }
        i += 1;
    }
    i
}
