use aoc2022::Input;
use std::{io::BufRead, ops::RangeInclusive};

type Sections = RangeInclusive<u32>;

fn parse_section(s: &str) -> Sections {
    let (start, end) = s.split_once('-').unwrap();
    Sections::new(start.parse().unwrap(), end.parse().unwrap())
}

fn parse(reader: Input) -> impl Iterator<Item = (Sections, Sections)> {
    reader.lines().map(|line| line.unwrap()).map(|line| {
        let (a, b) = line.split_once(',').unwrap();
        (parse_section(a), parse_section(b))
    })
}

pub fn part1(reader: Input) -> anyhow::Result<u32> {
    let r = parse(reader)
        .filter(|(a, b)| {
            let b_in_a = a.contains(b.start()) && a.contains(b.end());
            let a_in_b = b.contains(a.start()) && b.contains(a.end());
            b_in_a || a_in_b
        })
        .count();

    Ok(r as u32)
}

pub fn part2(reader: Input) -> anyhow::Result<u32> {
    let r = parse(reader)
        .filter(|(a, b)| {
            let b_in_a = a.contains(b.start()) || a.contains(b.end());
            let a_in_b = b.contains(a.start()) || b.contains(a.end());
            b_in_a || a_in_b
        })
        .count();

    Ok(r as u32)
}

pub fn main() {
    aoc2022::cli::run(part1, part2).unwrap();
}
