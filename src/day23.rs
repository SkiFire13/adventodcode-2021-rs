#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(u8, u8, u8)>;

pub fn input_generator(input: &str) -> Input {
    let mut out = vec![];
    for (y, line) in input.lines().skip(1).enumerate() {
        for (x, c) in line.chars().skip(1).enumerate() {
            if matches!(c, 'A' | 'B' | 'C' | 'D') {
                out.push((c as u8 - b'A', x as u8, y as u8))
            }
        }
    }
    out.sort_unstable();
    out
}

fn smallest_cost<const DEPTH: usize>(input: Input) -> u32 {
    let depth = DEPTH as u8;

    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    queue.push((Reverse(0), input.into_boxed_slice()));
    while let Some((Reverse(cost), state)) = queue.pop() {
        if seen.insert(state.clone()) {
            // Final check
            if state.iter().all(|&(c, x, _)| x == 2*(c+1)) {
                return cost;
            }

            // Possible moves for each character
            'c: for (idx, &(c, x, y)) in state.iter().enumerate() {
                // Check if already in final position
                if x == 2*(c+1) {
                    if state.iter().filter(|&&(fc, fx, fy)| fc == c && fx == x && fy > y).count() as u8 == depth - y {
                        continue 'c;
                    }
                }

                // Check if moveable up
                if y > 0 && state.iter().any(|&(_, ax, ay)| ax == x && ay < y) {
                    continue 'c;
                }

                // Going to left
                'xtarg1: for xtarg in (0..x).rev() {
                    // Move to destination
                    if xtarg == 2*(c+1) {
                        // No amphipod from other rooms
                        if !state.iter().any(|&(ac, ax, _)| ac != c && ax == xtarg) {
                            let count = state.iter().filter(|&&(_, fx, _)| fx == xtarg).count() as u8;
                            let mut state = state.clone();
                            state[idx] = (c, xtarg, depth - count);
                            state.sort_unstable();
                            queue.push((Reverse(cost + (y + x - xtarg + depth - count) as u32 * 10u32.pow(c as u32)), state));
                        }
                        // Can only move to destination if in the hallway
                        if y == 0 {
                            break 'xtarg1;
                        }
                    }
                    // Can't move outside doors
                    if matches!(xtarg, 2 | 4 | 6 | 8) {
                        continue 'xtarg1;
                    }
                    // Can't jump characters
                    if state.iter().any(|&(_, ax, _)| ax == xtarg) {
                        break 'xtarg1;
                    }
                    if y == 0 {
                        continue 'xtarg1;
                    }
                    // Move
                    let mut state = state.clone();
                    state[idx] = (c, xtarg, 0);
                    state.sort_unstable();
                    queue.push((Reverse(cost + (y + x - xtarg) as u32 * 10u32.pow(c as u32)), state));
                }

                // Going to right
                'xtarg2: for xtarg in x+1..=10 {
                    // Move to destination
                    if xtarg == 2*(c+1) {
                        // No amphipod from other rooms
                        if !state.iter().any(|&(ac, ax, _)| ac != c && ax == xtarg) {
                            let count = state.iter().filter(|&&(_, fx, _)| fx == xtarg).count() as u8;
                            let mut state = state.clone();
                            state[idx] = (c, xtarg, depth - count);
                            state.sort_unstable();
                            queue.push((Reverse(cost + (y + xtarg - x + depth - count) as u32 * 10u32.pow(c as u32)), state));
                        }
                        // Can only move to destination if in the hallway
                        if y == 0 {
                            break 'xtarg2;
                        }
                    }
                    // Can't move outside doors
                    if matches!(xtarg, 2 | 4 | 6 | 8) {
                        continue 'xtarg2;
                    }
                    // Can't jump characters
                    if state.iter().any(|&(_, ax, _)| ax == xtarg) {
                        break 'xtarg2;
                    }
                    if y == 0 {
                        continue 'xtarg2;
                    }
                    // Move
                    let mut state = state.clone();
                    state[idx] = (c, xtarg, 0);
                    state.sort_unstable();
                    queue.push((Reverse(cost + (y + xtarg - x) as u32 * 10u32.pow(c as u32)), state));
                }
            }
        }
    }
    panic!();
}

pub fn part1(input: &Input) -> u32 {
    smallest_cost::<2>(input.clone())
}

pub fn part2(input: &Input) -> u32 {
    let mut input = input.clone();
    input.iter_mut().filter(|(_, _, y)| *y == 2).for_each(|(_, _, y)| *y += 2);
    for (y, l) in [['D', 'C', 'B', 'A'], ['D', 'B', 'A', 'C']].into_iter().enumerate() {
        for (x, c) in l.into_iter().enumerate() {
            input.push((c as u8 - b'A', 2 * (x + 1) as u8, 2 + y as u8))
        }
    }
    input.sort_unstable();

    
    smallest_cost::<4>(input)
}
