#[allow(unused_imports)]
use super::prelude::*;
type Input = State<2>;

pub fn input_generator(input: &str) -> Input {
    let mut out = vec![];
    for (y, line) in input.lines().skip(1).enumerate() {
        for (x, c) in line.chars().skip(1).enumerate() {
            if matches!(c, 'A' | 'B' | 'C' | 'D') {
                out.push((c as u8 - b'A', x as u8, y as u8))
            }
        }
    }
    State::from_vec(out)
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct State<const DEPTH: usize> {
    inner: Box<[(u8, u8, u8)]>
}

impl<const DEPTH: usize> State<DEPTH> {
    fn from_vec(mut v: Vec<(u8, u8, u8)>) -> Self {
        v.sort_unstable();
        Self { inner: v.into_boxed_slice() }
    }
    fn to_vec(&self) -> Vec<(u8, u8, u8)> {
        self.inner.to_vec()
    }
    fn with(&self, idx: usize, x: u8, y: u8) -> Self {
        let mut state = self.clone();
        state.inner[idx] = (self.inner[idx].0, x, y);
        state.inner.sort_unstable();
        state
    }
    fn enumerate(&self) -> impl Iterator<Item = (usize, u8, u8, u8)> + '_ {
        self.inner.iter().enumerate().map(|(idx, &(c, x, y))| (idx, c, x, y))
    }
    fn any(&self, mut f: impl FnMut(u8, u8, u8) -> bool) -> bool {
        self.inner.iter().any(|&(c, x, y)| f(c, x, y))
    }
    fn count(&self, mut f: impl FnMut(u8, u8, u8) -> bool) -> u8 {
        self.inner.iter().filter(|&&(c, x, y)| f(c, x, y)).count() as u8
    }
}

fn smallest_cost<const DEPTH: usize>(input: State<DEPTH>) -> u32 {
    let depth = DEPTH as u8;

    #[ord_by_key(|this| Reverse(this.cost))]
    struct Entry<const DEPTH: usize> {
        cost: u32,
        state: State<DEPTH>,
    }

    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    queue.push(Entry { cost: 0, state: input });
    while let Some(Entry { cost, state }) = queue.pop() {
        if !seen.insert(state.clone()) {
            continue;
        }

        if !state.any(|c, x, _| x != 2*(c+1)) {
            return cost;
        }

        'c: for (idx, c, x, y) in state.enumerate() {
            if x == 2*(c+1) {
                if state.count(|fc, fx, fy| fc == c && fx == x && fy > y) == depth - y {
                    continue 'c;
                }
            }

            if y > 0 && state.any(|_, ax, ay| ax == x && ay < y) {
                continue 'c;
            }

            use std::ops::ControlFlow;
            let mut move_to_x = |xtarg| {
                let dx = if x < xtarg { xtarg - x } else { x - xtarg };
                if xtarg == 2*(c+1) {
                    if !state.any(|ac, ax, _| ac != c && ax == xtarg) {
                        let count = state.count(|_, fx, _| fx == xtarg);
                        let cost = cost + (y + dx + depth - count) as u32 * 10u32.pow(c as u32);
                        let state = state.with(idx, xtarg, depth - count);
                        queue.push(Entry { cost, state });
                    }
                    if y == 0 {
                        return ControlFlow::Break(());
                    }
                }
                if matches!(xtarg, 2 | 4 | 6 | 8) {
                    return ControlFlow::Continue(());
                }
                if state.any(|_, ax, _| ax == xtarg) {
                    return ControlFlow::Break(());
                }
                if y != 0 {
                    let cost = cost + (y + dx) as u32 * 10u32.pow(c as u32);
                    let state = state.with(idx, xtarg, 0);
                    queue.push(Entry { cost, state });
                }
                ControlFlow::Continue(())
            };

            (0..x).rev().try_for_each(&mut move_to_x);
            (x+1..11).try_for_each(&mut move_to_x);
        }
    }
    panic!();
}

pub fn part1(input: &Input) -> u32 {
    smallest_cost::<2>(input.clone())
}

pub fn part2(input: &Input) -> u32 {
    let mut input = input.to_vec();
    input.iter_mut().filter(|(_, _, y)| *y == 2).for_each(|(_, _, y)| *y += 2);
    for (y, l) in [['D', 'C', 'B', 'A'], ['D', 'B', 'A', 'C']].into_iter().enumerate() {
        for (x, c) in l.into_iter().enumerate() {
            input.push((c as u8 - b'A', 2 * (x + 1) as u8, 2 + y as u8))
        }
    }
    smallest_cost::<4>(State::from_vec(input))
}
