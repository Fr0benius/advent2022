const INPUT: &str = include_str!("../input/01.txt");

fn parse(s: &str) -> Vec<i64> {
    s.split("\n\n")
        .map(|blob| blob.lines().map(|s| s.parse::<i64>().unwrap()).sum::<i64>())
        .collect()
}
pub fn part1(input: &str) -> i64 {
    parse(input).into_iter().max().unwrap()
}

pub fn part2(input: &str) -> i64 {
    let mut nums = parse(input);
    nums.sort_unstable_by_key(|&k| -k);
    nums[0] + nums[1] + nums[2]
}

pub fn run() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
}
