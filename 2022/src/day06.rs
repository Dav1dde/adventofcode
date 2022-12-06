use aoc2022::Input;
use itertools::Itertools;
use std::io::Read;

fn has_no_duplicates(inp: &[u8]) -> bool {
    let s = inp
        .iter()
        .fold(0u32, |state, item| state | 1 << (item - b'a'));
    s.count_ones() == inp.len() as u32
}

pub fn part1(reader: Input) -> anyhow::Result<usize> {
    let r = reader
        .bytes()
        .map(|b| b.unwrap())
        .tuple_windows()
        .enumerate()
        .find_map(|(i, (a, b, c, d))| {
            let different = a != b && a != c && a != d && b != c && b != d && c != d;
            different.then_some(i + 4)
        })
        .unwrap();

    Ok(r)
}

pub fn part2(mut reader: Input) -> anyhow::Result<usize> {
    let mut input = Vec::new();
    reader.read_to_end(&mut input).unwrap();

    let r = input
        .windows(14)
        .enumerate()
        .find_map(|(i, arr)| has_no_duplicates(arr).then_some(i + arr.len()))
        .unwrap();

    Ok(r)
}

pub fn main() {
    aoc2022::cli::run(part1, part2).unwrap();
}
