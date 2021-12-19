#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<HashSet<[i16; 3]>>;

pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|beacons| {
            beacons
                .lines()
                .skip(1)
                .map(|line| {
                    let (x, rest) = line.split_once(',').unwrap();
                    let (y, z) = rest.split_once(',').unwrap();
                    [x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()]
                })
                .collect()
        })
        .collect()
}

type Rot = [[i16; 3]; 3];
type Vet = [i16; 3];

// m2 is applied first
const fn mat_x_mat(m1: Rot, m2: Rot) -> Rot {
    let mut output = [[0; 3]; 3];
    let mut i = 0;
    while i < 3 {
        let mut j = 0;
        while j < 3 {
            let mut k = 0;
            while k < 3 {
                output[i][j] += m1[i][k] * m2[k][j];
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    output
}

const fn mat_x_vet(m: Rot, v: Vet) -> Vet {
    let mut output = [0; 3];
    let mut i = 0;
    while i < 3 {
        let mut j = 0;
        while j < 3 {
            output[i] += m[i][j] * v[j];
            j += 1;
        }
        i += 1;
    }
    output
}

const ROT_NOP: Rot = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

const ROT_X: Rot = [[1, 0, 0], [0, 0, 1], [0, -1, 0]];
const ROT_XX: Rot = mat_x_mat(ROT_X, ROT_X);
const ROT_XXX: Rot = mat_x_mat(ROT_XX, ROT_X);

const ROT_UP: Rot = [[0, 0, 1], [0, 1, 0], [-1, 0, 0]];
const ROT_BEHIND: Rot = mat_x_mat(ROT_UP, ROT_UP);
const ROT_DOWN: Rot = mat_x_mat(ROT_BEHIND, ROT_UP);
const ROT_RIGHT: Rot = [[0, 1, 0], [-1, 0, 0], [0, 0, 1]];
const ROT_LEFT: Rot = mat_x_mat(ROT_BEHIND, ROT_RIGHT);

const ROTS_X: [Rot; 4] = [ROT_NOP, ROT_X, ROT_XX, ROT_XXX];
const ROTS_AXIS: [Rot; 6] = [ROT_NOP, ROT_UP, ROT_DOWN, ROT_BEHIND, ROT_LEFT, ROT_RIGHT];

const ROTS: [Rot; 24] = {
    let mut rots = [ROT_NOP; 24];
    let mut i = 0;
    while i < 6 {
        let mut j = 0;
        while j < 4 {
            rots[i * 4 + j] = mat_x_mat(ROTS_X[j], ROTS_AXIS[i]);
            j += 1;
        }
        i += 1;
    }
    rots
};

fn resolve_positions(input: &Input) -> Vec<([i16; 3], HashSet<[i16; 3]>)> {
    let mut num_found = 1;
    let mut found = vec![None; input.len()];
    found[0] = Some(([0; 3], input[0].clone()));

    let mut visited = Grid {
        vec: vec![false; input.len() * input.len()],
        width: input.len(),
    };

    let rotated_points = input
        .iter()
        .map(|points| {
            ROTS.map(|rot| {
                points
                    .iter()
                    .map(|&p| mat_x_vet(rot, p))
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();

    while num_found < input.len() {
        'i: for i in 0..input.len() {
            if found[i].is_some() {
                continue 'i;
            }
            'j: for j in 0..input.len() {
                if found[j].is_none() || replace(&mut visited[(i, j)], true) {
                    continue 'j;
                }
                for points in &rotated_points[i] {
                    for point in &points[11..] {
                        let candidates = &found[j].as_ref().unwrap().1;
                        for &candidate in candidates.iter().take(candidates.len() - 11) {
                            let root = [
                                candidate[0] - point[0],
                                candidate[1] - point[1],
                                candidate[2] - point[2],
                            ];
                            let num_intersections = points
                                .iter()
                                .map(|&p| [p[0] + root[0], p[1] + root[1], p[2] + root[2]])
                                .filter(|p| candidates.contains(p))
                                .count();
                            if num_intersections >= 12 {
                                let points = points
                                    .iter()
                                    .map(|&p| [p[0] + root[0], p[1] + root[1], p[2] + root[2]])
                                    .collect();
                                found[i] = Some((root, points));
                                num_found += 1;
                                continue 'i;
                            }
                        }
                    }
                }
            }
        }
    }

    found.into_iter().map(Option::unwrap).collect()
}

pub fn part1(input: &Input) -> usize {
    resolve_positions(input)
        .into_iter()
        .flat_map(|(_, beacons)| beacons.into_iter())
        .unique()
        .count()
}

pub fn part2(input: &Input) -> i16 {
    let found = resolve_positions(input);
    (0..input.len())
        .flat_map(|i| (i + 1..input.len()).map(move |j| (i, j)))
        .map(|(i, j)| (found[i].0, found[j].0))
        .map(|(p1, p2)| (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs() + (p1[2] - p2[2]).abs())
        .max()
        .unwrap()
}
