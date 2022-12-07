use advent_util::parsing::re_parser;

const INPUT: &str = include_str!("../input/04.txt");

type Interval = (i64, i64);
fn contains((a, b): Interval, (c, d): Interval) -> bool {
    a <= c && d <= b
}

fn overlaps((a, b): Interval, (c, d): Interval) -> bool {
    !(b < c || d < a)
}

fn parse(input: &str) -> Vec<(Interval, Interval)> {
    input
        .lines()
        .map(re_parser(r"(.*)-(.*),(.*)-(.*)"))
        .collect()
}

fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|&(x, y)| contains(x, y) || contains(y, x))
        .count()
}

fn part2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|&(x, y)| overlaps(x, y))
        .count()
}

pub fn run() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
}
