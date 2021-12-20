#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<bool>, Grid<bool>);

pub fn input_generator(input: &str) -> Input {
    let (pattern, grid) = input.split_once("\n\n").unwrap();
    let pattern = pattern.chars().map(|c| c == '#').collect();
    let grid = Grid::from_input_chars(grid, |c, _, _| c == '#');
    (pattern, grid)
}

fn enhance(pattern: &[bool], image: &mut Grid<bool>, new_image: &mut Grid<bool>, ext: bool) {
    new_image
        .vec
        .par_chunks_mut(new_image.width)
        .enumerate()
        .flat_map(|(y, line)| {
            line.par_iter_mut()
                .enumerate()
                .map(move |(x, elm)| (x, y, elm))
        })
        .for_each(|(x, y, elm)| {
            let (x, y) = (x as isize, y as isize);
            let arr = [
                image.iget((x - 1, y - 1)),
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
            *elm = pattern[pos];
        });
    swap(image, new_image);
}

fn enhance_n(pattern: &[bool], image: &Grid<bool>, n: usize) -> usize {
    let mut ext_image = Grid {
        vec: iter::repeat(false)
            .take(n * (image.width + 2 * n))
            .chain(image.vec.chunks(image.width).flat_map(|chunk| {
                iter::repeat(false)
                    .take(n)
                    .chain(chunk.iter().copied())
                    .chain(iter::repeat(false).take(n))
            }))
            .chain(iter::repeat(false).take(n * (image.width + 2 * n)))
            .collect(),
        width: image.width + 2 * n,
    };
    let mut buf_image = Grid {
        vec: vec![false; ext_image.vec.len()],
        width: ext_image.w(),
    };

    enhance(pattern, &mut ext_image, &mut buf_image, false);
    for i in 0..n - 1 {
        let pat_pos = if i % 2 == 0 { 0 } else { (1 << 9) - 1 };
        enhance(pattern, &mut ext_image, &mut buf_image, pattern[pat_pos]);
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
