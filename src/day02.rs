const INPUT: &str = include_str!("../input/02.txt");

fn score1(s: &str) -> i64 {
    let s = s.as_bytes();
    let a = (s[0] - b'A') as i64;
    let b = (s[2] - b'X') as i64;
    let d = (b - a + 4) % 3;
    d * 3 + b + 1
}

pub fn part1(input: &str) -> i64 {
    input.lines().map(score1).sum()
}

fn score2(s: &str) -> i64 {
    let s = s.as_bytes();
    let a = (s[0] - b'A') as i64;
    let d = (s[2] - b'X') as i64;
    let b = (a + d + 2) % 3;
    d * 3 + b + 1
}

pub fn part2(input: &str) -> i64 {
    input.lines().map(score2).sum()
}

pub fn run() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
}
