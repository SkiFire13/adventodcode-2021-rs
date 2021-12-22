#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(bool, [[i32; 2]; 3])>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (onoff, rest) = line.split_once(' ').unwrap();
            let (x, rest) = rest[2..].split_once(",y=").unwrap();
            let (y, z) = rest.split_once(",z=").unwrap();
            let (x0, x1) = x.split_once("..").unwrap();
            let (y0, y1) = y.split_once("..").unwrap();
            let (z0, z1) = z.split_once("..").unwrap();
            let coords = [
                [x0.parse().unwrap(), x1.parse().unwrap()],
                [y0.parse().unwrap(), y1.parse().unwrap()],
                [z0.parse().unwrap(), z1.parse().unwrap()],
            ];
            (onoff == "on", coords)
        })
        .collect()
}

fn find_on_area(input: impl Iterator<Item = (bool, [[i32; 2]; 3])>) -> usize {
    let mut cubes = Vec::<[[i32; 2]; 3]>::new();
    for (onoff, cube) in input {
        let mut i = 0;
        'cubes: while i < cubes.len() {
            let cubei = cubes[i];

            if cubei == cube {
                cubes.swap_remove(i);
                break 'cubes;
            }

            let clip = [
                [max(cube[0][0], cubei[0][0]), min(cube[0][1], cubei[0][1])],
                [max(cube[1][0], cubei[1][0]), min(cube[1][1], cubei[1][1])],
                [max(cube[2][0], cubei[2][0]), min(cube[2][1], cubei[2][1])],
            ];

            for axis in clip {
                if axis[0] > axis[1] {
                    i += 1;
                    continue 'cubes;
                }
            }

            if cubei[0][0] != clip[0][0] {
                cubes.push([[cubei[0][0], clip[0][0] - 1], cubei[1], cubei[2]]);
            }
            if cubei[0][1] != clip[0][1] {
                cubes.push([[clip[0][1] + 1, cubei[0][1]], cubei[1], cubei[2]]);
            }
            if cubei[1][0] != clip[1][0] {
                cubes.push([clip[0], [cubei[1][0], clip[1][0] - 1], cubei[2]]);
            }
            if cubei[1][1] != clip[1][1] {
                cubes.push([clip[0], [clip[1][1] + 1, cubei[1][1]], cubei[2]]);
            }
            if cubei[2][0] != clip[2][0] {
                cubes.push([clip[0], clip[1], [cubei[2][0], clip[2][0] - 1]]);
            }
            if cubei[2][1] != clip[2][1] {
                cubes.push([clip[0], clip[1], [clip[2][1] + 1, cubei[2][1]]]);
            }

            cubes.swap_remove(i);
        }
        if onoff {
            cubes.push(cube);
        }
    }

    cubes
        .iter()
        .map(|&cube| {
            ((cube[0][1] - cube[0][0] + 1) as usize)
                * ((cube[1][1] - cube[1][0] + 1) as usize)
                * ((cube[2][1] - cube[2][0] + 1) as usize)
        })
        .sum()
}

pub fn part1(input: &Input) -> usize {
    let clip = |cube: [[i32; 2]; 3]| {
        [
            [max(cube[0][0], -50), min(cube[0][1], 50)],
            [max(cube[1][0], -50), min(cube[1][1], 50)],
            [max(cube[2][0], -50), min(cube[2][1], 50)],
        ]
    };
    let clipped_input = input
        .iter()
        .map(|&(onoff, cube)| (onoff, clip(cube)))
        .filter(|(_, cube)| cube[0][0] <= cube[0][1])
        .filter(|(_, cube)| cube[1][0] <= cube[1][1])
        .filter(|(_, cube)| cube[2][0] <= cube[2][1]);

    find_on_area(clipped_input)
}

pub fn part2(input: &Input) -> usize {
    find_on_area(input.iter().copied())
}
