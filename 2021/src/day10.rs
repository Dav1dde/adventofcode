use aoc2021::Input;
use itertools::Itertools;
use std::io::BufRead;

fn parse(data: &[u8]) -> Result<Vec<u8>, (usize, u8)> {
    let mut stack = vec![];

    for (i, byte) in data.iter().enumerate() {
        match (stack.last(), byte) {
            (_, b'(' | b'[' | b'{' | b'<') => stack.push(*byte),
            (Some(b'('), b')') => {
                stack.pop();
            }
            (Some(b'['), b']') => {
                stack.pop();
            }
            (Some(b'{'), b'}') => {
                stack.pop();
            }
            (Some(b'<'), b'>') => {
                stack.pop();
            }
            (_, _) => return Err((i, *byte)),
        }
    }

    Ok(stack)
}

fn to_score(c: u8) -> usize {
    match c {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => panic!("invalid char: {}", c),
    }
}

fn autocomplete_score(mut stack: Vec<u8>) -> usize {
    let mut score = 0;
    while let Some(open) = stack.pop() {
        let s = match open {
            b'(' => 1,
            b'[' => 2,
            b'{' => 3,
            b'<' => 4,
            _ => panic!("invalid char: {}", open),
        };

        score = (score * 5) + s;
    }

    score
}

pub fn part1(reader: Input) -> anyhow::Result<usize> {
    let r = reader
        .lines()
        .map(|line| line.unwrap())
        .filter_map(|line| parse(line.as_bytes()).err())
        .map(|(_, c)| c)
        .map(to_score)
        .sum();

    Ok(r)
}

pub fn part2(reader: Input) -> anyhow::Result<usize> {
    let r = reader
        .lines()
        .map(|line| line.unwrap())
        .filter_map(|line| parse(line.as_bytes()).ok())
        .map(autocomplete_score)
        .sorted_unstable()
        .collect_vec();

    Ok(r[r.len() / 2])
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
