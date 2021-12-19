#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<HashSet<[i32; 3]>>;

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

type Rot = [[i32; 3]; 3];
type Vet = [i32; 3];

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

fn resolve_positions(input: &Input) -> FxHashMap<usize, ([i32; 3], HashSet<[i32; 3]>)> {
    let mut found: FxHashMap<usize, ([i32; 3], HashSet<[i32; 3]>)> =
        FxHashMap::from_iter([(0, ([0; 3], input[0].clone()))]);
    let mut visited: FxHashSet<(usize, usize)> = FxHashSet::default();

    while found.len() < input.len() {
        'i: for i in 0..input.len() {
            if found.contains_key(&i) {
                continue 'i;
            }
            'j: for j in found.keys().copied() {
                if !visited.insert((i, j)) {
                    continue 'j;
                }

                for (rot_x, rot_axis) in itertools::iproduct!(ROTS_X, ROTS_AXIS) {
                    let rot = mat_x_mat(rot_x, rot_axis);
                    let points = input[i]
                        .iter()
                        .map(|&p| mat_x_vet(rot, p))
                        .collect::<Vec<_>>();
                    for point in &points {
                        for &candidate in &found[&j].1 {
                            let root = [
                                candidate[0] - point[0],
                                candidate[1] - point[1],
                                candidate[2] - point[2],
                            ];
                            let num_intersections = points
                                .iter()
                                .map(|&p| [p[0] + root[0], p[1] + root[1], p[2] + root[2]])
                                .filter(|p| found[&j].1.contains(p))
                                .count();
                            if num_intersections >= 12 {
                                let points = points
                                    .iter()
                                    .map(|&p| [p[0] + root[0], p[1] + root[1], p[2] + root[2]])
                                    .collect();
                                found.insert(i, (root, points));
                                continue 'i;
                            }
                        }
                    }
                }
            }
        }
    }

    found
}

pub fn part1(input: &Input) -> usize {
    resolve_positions(input)
        .values()
        .flat_map(|(_, beacons)| beacons.iter().copied())
        .unique()
        .count()
}

pub fn part2(input: &Input) -> i32 {
    let found = resolve_positions(input);
    (0..input.len())
        .flat_map(|i| (i + 1..input.len()).map(move |j| (i, j)))
        .map(|(i, j)| (found[&i].0, found[&j].0))
        .map(|(p1, p2)| (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs() + (p1[2] - p2[2]).abs())
        .max()
        .unwrap()
}
