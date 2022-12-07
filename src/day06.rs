use std::collections::HashSet;

const INPUT: &str = include_str!("../input/06.txt");

fn find_marker(s: &str, k: usize) -> usize {
    s.as_bytes()
        .windows(k)
        .position(|w| w.iter().collect::<HashSet<_>>().len() == k)
        .unwrap()
        + k
}

fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

fn part2(input: &str) -> usize {
    find_marker(input, 14)
}

pub fn run() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
}
