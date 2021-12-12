#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = HashMap<&'a str, Vec<&'a str>>;

pub fn input_generator(input: &str) -> Input {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (first, second) = line.split_once('-').unwrap();
        map.entry(first).or_insert(Vec::new()).push(second);
        map.entry(second).or_insert(Vec::new()).push(first);
    }
    map
}

pub fn paths_num<'a>(
    start: &'a str,
    input: &Input<'a>,
    visited: &mut HashSet<&'a str>,
    allow_twice: &mut bool,
) -> usize {
    if start == "end" {
        return 1;
    }
    let mut num = 0;
    for &dest in &input[start] {
        if visited.insert(dest) || dest.as_bytes()[0].is_ascii_uppercase() {
            num += paths_num(dest, input, visited, allow_twice);
            visited.remove(dest);
        } else if *allow_twice == true && dest != "start" {
            *allow_twice = false;
            num += paths_num(dest, input, visited, allow_twice);
            *allow_twice = true;
        }
    }
    num
}

pub fn part1(input: &Input) -> usize {
    paths_num("start", input, &mut HashSet::from(["start"]), &mut false)
}

pub fn part2(input: &Input) -> usize {
    paths_num("start", input, &mut HashSet::from(["start"]), &mut true)
}
