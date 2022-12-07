use advent_util::parsing::re_parser;
use advent_util::parsing::Gather;

const INPUT: &str = include_str!("../input/05.txt");

fn parse(blob: &str) -> Vec<Vec<char>> {
    let m = blob.lines().next().unwrap().len();
    let mut res = vec![vec![]; (m + 3) / 4];
    for line in blob.lines() {
        for (i, w) in line.as_bytes().chunks(4).enumerate() {
            if w[1].is_ascii_uppercase() {
                res[i].push(w[1] as char);
            }
        }
    }
    for v in &mut res {
        v.reverse();
    }
    res
}

fn part1(input: &str) -> String {
    let (init, moves): (&str, &str) = input.split("\n\n").gather();
    let mut state = parse(init);
    let parser = re_parser(r"move (.*) from (.*) to (.*)");
    for line in moves.lines() {
        let (k, i, j): (usize, usize, usize) = parser(line);
        for _ in 0..k {
            let c = state[i-1].pop().unwrap();
            state[j-1].push(c);
        }
    }
    state.into_iter().map(|v| *v.last().unwrap()).collect()
}


fn part2(input: &str) -> String {
    let (init, moves): (&str, &str) = input.split("\n\n").gather();
    let mut state = parse(init);
    let parser = re_parser(r"move (.*) from (.*) to (.*)");
    for line in moves.lines() {
        let (k, i, j): (usize, usize, usize) = parser(line);
        let ix = state[i-1].len() - k;
        let tmp = state[i-1][ix..].to_vec();
        state[i-1].truncate(ix);
        state[j-1].extend(&tmp);
    }
    state.into_iter().map(|v| *v.last().unwrap()).collect()
}


pub fn run() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
}
