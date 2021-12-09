#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8 - b'0')
}

pub fn part1(input: &Input) -> u32 {
    let mut risk = 0;
    for x in 0..input.width {
        for y in 0..input.height() {
            let p = input[(x, y)];
            if [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(dx, dy)| input.iget((x as isize + dx, y as isize + dy)))
                .all(|&n| n > p)
            {
                risk += 1 + p as u32;
            }
        }
    }
    risk
}

pub fn part2(input: &Input) -> usize {
    let mut basins: Vec<HashSet<(usize, usize)>> = Vec::new();
    for x in 0..input.width {
        for y in 0..input.height() {
            if input[(x,y)] != 9 {
                let mut connected_basins: Vec<usize> = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .into_iter()
                    .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
                    .filter(|&(x, y)| input.iget((x, y)).map_or(false, |&b| b != 9))
                    .map(|(x, y)| (x as usize, y as usize))
                    .filter_map(|(x, y)| basins.iter().position(|basin| basin.contains(&(x, y))))
                    .sorted()
                    .collect();
                connected_basins.dedup();            

                match connected_basins.len() {
                    0 => basins.push(HashSet::from([(x, y)])),
                    1 => {
                        basins[connected_basins[0]].insert((x, y));
                    }
                    _ => {
                        let new_basin = connected_basins
                            .iter()
                            .rev()
                            .flat_map(|&idx| basins.swap_remove(idx))
                            .chain([(x, y)])
                            .collect();
                        basins.push(new_basin);
                    }
                }
            }
        }
    }

    basins.sort_by_key(|b| -(b.len() as isize));
    basins[..3].iter().map(|b| b.len()).product()
}
