#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<bool>, Grid<bool>);

pub fn input_generator(input: &str) -> Input {
    let (pattern, grid) = input.split_once("\n\n").unwrap();
    let pattern = pattern.chars().map(|c| c == '#').collect();
    let grid = Grid::from_input_chars(grid, |c, _, _| c == '#');
    (pattern, grid)
}

fn enhance(pattern: &[bool], image: &mut Grid<bool>, ext: bool) {
    let mut new_image = Grid {
        vec: vec![false; image.vec.len()],
        width: image.width,
    };
    for (y, x) in itertools::iproduct!(0..image.h() as isize, 0..image.w() as isize) {
        let arr = [
            image.iget((x - 1, y - 1,)),
            image.iget((x, y - 1)),
            image.iget((x + 1, y - 1)),
            image.iget((x - 1, y)),
            image.iget((x, y)),
            image.iget((x + 1, y)),
            image.iget((x - 1, y + 1)),
            image.iget((x, y + 1)),
            image.iget((x + 1, y + 1)),
        ];
        let pos = arr
            .into_iter()
            .map(|b| b.copied().unwrap_or(ext))
            .fold(0, |acc, b| (acc << 1) | (b as usize));
        new_image[(x as usize, y as usize)] = pattern[pos];
    }
    *image = new_image;
}

fn enhance_n(pattern: &[bool], image: &Grid<bool>, n: usize) -> usize {
    let mut ext_image = Grid {
        vec: iter::repeat(false)
            .take(n * (image.width + 2*n))
            .chain(image.vec.chunks(image.width).flat_map(|chunk| {
                iter::repeat(false)
                    .take(n)
                    .chain(chunk.iter().copied())
                    .chain(iter::repeat(false).take(n))
            }))
            .chain(iter::repeat(false).take(n * (image.width + 2*n)))
            .collect(),
        width: image.width + 2*n,
    };

    enhance(pattern, &mut ext_image, false);
    for i in 0..n-1 {
        let pat_pos = if i % 2 == 0 { 0 } else { (1 << 9) - 1 };
        enhance(pattern, &mut ext_image, pattern[pat_pos]);
    }
    ext_image.vec.iter().filter(|&&b| b).count()
}

pub fn part1(input: &Input) -> usize {
    let (pattern, image) = input;
    enhance_n(pattern, image, 2)
}

pub fn part2(input: &Input) -> usize {
    let (pattern, image) = input;
    enhance_n(pattern, image, 50)
}
