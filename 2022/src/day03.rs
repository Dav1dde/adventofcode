use aoc2022::Input;
use itertools::Itertools;
use std::io::BufRead;

#[derive(Copy, Clone)]
enum State {
    Empty,
    Seen,
    Read,
}

impl State {
    pub fn seen(&mut self) {
        *self = Self::Seen;
    }

    pub fn read(&mut self) -> bool {
        let v = matches!(self, Self::Seen);
        *self = Self::Read;
        v
    }
}

fn find_duplicates(inp: &[u8]) -> impl Iterator<Item = u8> + '_ {
    debug_assert!(inp.len() % 2 == 0);

    let half = inp.len() / 2;

    let mut duplicates = [State::Empty; 26 * 2];

    for a in &inp[0..half] {
        // we know this is safe because to_index returns in that range
        unsafe { duplicates.get_unchecked_mut(to_index(*a)).seen() };
    }

    inp[half..]
        .iter()
        .copied()
        .filter(move |&b| unsafe { duplicates.get_unchecked_mut(to_index(b)).read() })
}

fn find_group(a: &[u8], b: &[u8], c: &[u8]) -> u8 {
    let mut duplicates = [0u8; 26 * 2];

    for &a in a {
        unsafe { *duplicates.get_unchecked_mut(to_index(a)) |= 1 }
    }
    for &b in b {
        unsafe { *duplicates.get_unchecked_mut(to_index(b)) |= 1 << 1 }
    }
    for &c in c {
        if unsafe { *duplicates.get_unchecked(to_index(c)) } == 0b11 {
            return c;
        }
    }

    panic!("invalid input");
}

#[inline]
fn to_index(item: u8) -> usize {
    let r = match item {
        b'a'..=b'z' => item - b'a',
        b'A'..=b'Z' => item - b'A' + 26,
        _ => unreachable!(),
    };
    r as usize
}

pub fn part1(reader: Input) -> anyhow::Result<usize> {
    let r = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            find_duplicates(line.as_bytes())
                .map(|dup| to_index(dup) + 1)
                .sum::<usize>()
        })
        .sum();

    Ok(r)
}

pub fn part2(reader: Input) -> anyhow::Result<usize> {
    let r = reader
        .lines()
        .map(|line| line.unwrap())
        .tuples()
        .map(|(a, b, c)| find_group(a.as_bytes(), b.as_bytes(), c.as_bytes()))
        .map(|r| to_index(r) + 1)
        .sum();

    Ok(r)
}

pub fn main() {
    aoc2022::cli::run(part1, part2).unwrap();
}
