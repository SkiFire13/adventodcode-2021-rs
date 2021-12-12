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

pub fn part1(input: &Input) -> usize {
    fn paths<'a>(input: &Input<'a>, visited: &mut HashSet<&'a str>, curr_pos: &'a str,) -> usize {
        if curr_pos == "end" {
            return 1;
        }
        let mut paths_num = 0;
        for &dest in &input[curr_pos] {
            if visited.insert(dest) || dest.as_bytes()[0].is_ascii_uppercase() {
                paths_num += paths(input, visited, dest);
                visited.remove(dest);
            }
        }
        paths_num
    }

    paths(input, &mut HashSet::from(["start"]), "start")
}

pub fn part2(input: &Input) -> usize {
    fn paths<'a>(input: &Input<'a>, visited: &mut HashSet<&'a str>, curr_pos: &'a str, twice: &mut bool) -> usize {
        if curr_pos == "end" {
            return 1;
        }
        let mut paths_num = 0;
        for &dest in &input[curr_pos] {
            if visited.insert(dest) || dest.as_bytes()[0].is_ascii_uppercase() {
                paths_num += paths(input, visited, dest, twice);
                visited.remove(dest);
            } else if *twice == false && dest != "start" {
                *twice = true;
                paths_num += paths(input, visited, dest, twice);
                *twice = false;
            }
        }
        paths_num
    }

    paths(input, &mut HashSet::from(["start"]), "start", &mut false)
}
