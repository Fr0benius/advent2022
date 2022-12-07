use std::collections::HashSet;

const INPUT: &str = include_str!("../input/03.txt");
const SAMPLE: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

fn priority(c: u8) -> i64 {
    (if c.is_ascii_lowercase() {
        c - b'a' + 1
    } else {
        c - b'A' + 27
    }) as i64
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let n = line.len();
            let a: HashSet<_> = line.bytes().take(n / 2).collect();
            let b: HashSet<_> = line.bytes().skip(n / 2).collect();
            a.intersection(&b).copied().map(priority).sum::<i64>()
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let lines: Vec<_> = input.lines().collect();
    lines
        .chunks(3)
        .map(|w| {
            w[0].bytes()
                .collect::<HashSet<_>>()
                .into_iter()
                .filter(|&c| w.iter().all(|&s| s.contains([c as char])))
                .map(priority)
                .sum::<i64>()
        })
        .sum()
}

pub fn run() {
    dbg!(part1(SAMPLE));
    dbg!(part1(INPUT));
    dbg!(part2(SAMPLE));
    dbg!(part2(INPUT));
}
