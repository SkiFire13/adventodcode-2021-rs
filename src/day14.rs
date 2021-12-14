#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = (Vec<u8>, FxHashMap<(u8, u8), u8>);

pub fn input_generator(input: &str) -> Input {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let template = template.as_bytes().to_vec();
    let rules = rules
        .lines()
        .map(|line| line.as_bytes())
        .map(|line| ((line[0], line[1]), line[6]))
        .collect();
    (template, rules)
}

pub fn simulate_steps(template: &[u8], rules: &FxHashMap<(u8, u8), u8>, steps: usize) -> usize {
    let mut counts = template.iter().copied().counts();
    let mut tuple_counts = FxHashMap::default();
    for t in template.iter().copied().tuple_windows() {
        *tuple_counts.entry(t).or_default() += 1;
    }
    let mut new_tuple_counts = FxHashMap::<(u8, u8), usize>::default();
    for _ in 0..steps {
        for (&(a, b), &count) in tuple_counts.iter() {
            if let Some(&middle) = rules.get(&(a, b)) {
                *new_tuple_counts.entry((a, middle)).or_default() += count;
                *new_tuple_counts.entry((middle, b)).or_default() += count;
                *counts.entry(middle).or_default() += count;
            } else {
                *new_tuple_counts.entry((a, b)).or_default() += count;
            }
        }
        std::mem::swap(&mut tuple_counts, &mut new_tuple_counts);
        new_tuple_counts.iter_mut().for_each(|(_, count)| *count = 0);
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    max - min
}

pub fn part1(input: &Input) -> usize {
    simulate_steps(&input.0, &input.1, 10)
}

pub fn part2(input: &Input) -> usize {
    simulate_steps(&input.0, &input.1, 40)
}
