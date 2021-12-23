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
    inner: [[(u8, u8); DEPTH]; 4],
}

impl<const DEPTH: usize> State<DEPTH> {
    fn iter(&self) -> impl Iterator<Item = (u8, u8, u8)> + '_ {
        self.inner
            .into_iter()
            .enumerate()
            .flat_map(|(c, inner)| inner.into_iter().map(move |(x, y)| (c as u8, x, y)))
    }
    fn from_vec(v: Vec<(u8, u8, u8)>) -> Self {
        let mut inner = [[(u8::MAX, u8::MAX); DEPTH]; 4];
        for (c, x, y) in v.into_iter() {
            let mut idx = 0;
            while inner[c as usize][idx] != (u8::MAX, u8::MAX) {
                idx += 1;
            }
            inner[c as usize][idx] = (x, y);
        }
        Self { inner }
    }
    fn to_vec(&self) -> Vec<(u8, u8, u8)> {
        self.iter().collect()
    }
    fn with(&self, idx: usize, x: u8, y: u8) -> Self {
        let mut state = self.clone();
        state.inner[idx / DEPTH][idx % DEPTH] = (x, y);
        state
            .inner
            .iter_mut()
            .for_each(|inner| inner.sort_unstable());
        state
    }
    fn enumerate(&self) -> impl Iterator<Item = (usize, u8, u8, u8)> + '_ {
        self.iter()
            .enumerate()
            .map(|(idx, (c, x, y))| (idx, c, x, y))
    }
    fn any(&self, mut f: impl FnMut(u8, u8, u8) -> bool) -> bool {
        self.iter().any(|(c, x, y)| f(c, x, y))
    }
    fn count(&self, mut f: impl FnMut(u8, u8, u8) -> bool) -> u8 {
        self.iter().filter(|&(c, x, y)| f(c, x, y)).count() as u8
    }
}

fn star_add<const DEPTH: usize>(state: &State<DEPTH>) -> u32 {
    let mut star = 0;
    for (_, c, x, y) in state.enumerate() {
        let xtarg = 2 * (c + 1);
        if x != xtarg {
            let dx = if x < xtarg { xtarg - x } else { x - xtarg};
            star += (y + dx + 1) as u32 * 10u32.pow(c as u32);
        }
    }
    star
}

fn smallest_cost<const DEPTH: usize>(input: State<DEPTH>) -> u32 {
    let depth = DEPTH as u8;

    #[ord_by_key(|this| Reverse(this.star))]
    struct Entry<const DEPTH: usize> {
        star: u32,
        cost: u32,
        state: State<DEPTH>,
    }

    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    queue.push(Entry {
        star: star_add(&input),
        cost: 0,
        state: input,
    });
    while let Some(Entry { star: _, cost, state }) = queue.pop() {
        let (mut cost, mut state) = (cost, state);
        if !seen.insert(state.clone()) {
            continue;
        }

        if !state.any(|c, x, _| x != 2 * (c + 1)) {
            return cost;
        }

        let before = state.clone();
        'l: loop {
            'i: for (idx, c, x, y) in state.clone().enumerate() {
                if x == 2 * (c + 1)
                    || state.any(|fc, fx, _| fc != c && fx == 2 * (c + 1))
                    || (x % 2 == 0 && state.any(|_, fx, fy| fx == x && fy < y))
                {
                    continue 'i;
                }
                let xtarg = 2 * (c + 1);
                let mut range = if x < xtarg { x + 1..xtarg } else { xtarg..x };
                if !range.any(|x| state.any(|_, fx, fy| x == fx && fy == 0)) {
                    let dx = if x < xtarg { xtarg - x } else { x - xtarg };
                    let count = state.count(|_, fx, _| fx == xtarg);
                    cost += (y + dx + depth - count) as u32 * 10u32.pow(c as u32);
                    state = state.with(idx, xtarg, depth - count);
                    continue 'l;
                }
            }
            break 'l;
        }
        if state != before && !state.any(|c, x, _| x != 2 * (c + 1)) {
            let star = cost + star_add(&state);
            queue.push(Entry { star, cost, state });
            continue;
        }

        'c: for (idx, c, x, y) in state.enumerate() {
            if y == 0 {
                continue 'c;
            }

            if x == 2 * (c + 1) {
                if state.count(|fc, fx, fy| fc == c && fx == x && fy > y) == depth - y {
                    continue 'c;
                }
            }

            if y > 0 && state.any(|_, ax, ay| ax == x && ay < y) {
                continue 'c;
            }

            let mut move_to_x = |xtarg| {
                let dx = if x < xtarg { xtarg - x } else { x - xtarg };
                if matches!(xtarg, 2 | 4 | 6 | 8) {
                    return ControlFlow::Continue(());
                }
                if state.any(|_, ax, _| ax == xtarg) {
                    return ControlFlow::Break(());
                }
                let cost = cost + (y + dx) as u32 * 10u32.pow(c as u32);
                let state = state.with(idx, xtarg, 0);
                let star = cost + star_add(&state);
                queue.push(Entry { star, cost, state });
                ControlFlow::Continue(())
            };

            (0..x).rev().try_for_each(&mut move_to_x);
            (x + 1..11).try_for_each(&mut move_to_x);
        }
    }
    panic!();
}

pub fn part1(input: &Input) -> u32 {
    smallest_cost::<2>(input.clone())
}

pub fn part2(input: &Input) -> u32 {
    let mut input = input.to_vec();
    input
        .iter_mut()
        .filter(|(_, _, y)| *y == 2)
        .for_each(|(_, _, y)| *y += 2);
    for (y, l) in [['D', 'C', 'B', 'A'], ['D', 'B', 'A', 'C']]
        .into_iter()
        .enumerate()
    {
        for (x, c) in l.into_iter().enumerate() {
            input.push((c as u8 - b'A', 2 * (x + 1) as u8, 2 + y as u8))
        }
    }
    smallest_cost::<4>(State::from_vec(input))
}
