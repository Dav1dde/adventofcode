use aoc2021::Input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

#[derive(Debug)]
enum Segment {
    Fixed(char),
    Maybe(HashSet<char>),
}

impl Segment {
    fn learn(&mut self, inp: &str, matches: bool) -> bool {
        if let Self::Maybe(current) = self {
            if matches {
                current.retain(|&c| inp.chars().any(|tc| tc == c));
            } else {
                current.retain(|&c| !inp.chars().any(|tc| tc == c));
            }
            if current.len() == 1 {
                *self = Self::Fixed(*current.iter().next().unwrap());
                return true;
            }
        }
        false
    }

    fn fixed(&self) -> Option<char> {
        match self {
            Self::Fixed(c) => Some(*c),
            _ => None,
        }
    }
}

static A: usize = 0;
static B: usize = 1;
static C: usize = 2;
static D: usize = 3;
static E: usize = 4;
static F: usize = 5;
static G: usize = 6;

impl Default for Segment {
    fn default() -> Self {
        let mut maybe = HashSet::new();
        maybe.insert('a');
        maybe.insert('b');
        maybe.insert('c');
        maybe.insert('d');
        maybe.insert('e');
        maybe.insert('f');
        maybe.insert('g');
        Self::Maybe(maybe)
    }
}

#[derive(Default, Debug)]
struct SeventSegment {
    display: [Segment; 7],
}

impl SeventSegment {
    fn learn(&mut self, inp: &str) {
        let len = inp.len();
        let mut more_info = false;
        match len {
            2 => {
                more_info = more_info || self.display[A].learn(inp, false);
                more_info = more_info || self.display[B].learn(inp, false);
                more_info = more_info || self.display[C].learn(inp, true);
                more_info = more_info || self.display[D].learn(inp, false);
                more_info = more_info || self.display[E].learn(inp, false);
                more_info = more_info || self.display[F].learn(inp, true);
                more_info = more_info || self.display[G].learn(inp, false);
            }
            3 => {
                more_info = more_info || self.display[A].learn(inp, true);
                more_info = more_info || self.display[B].learn(inp, false);
                more_info = more_info || self.display[C].learn(inp, true);
                more_info = more_info || self.display[D].learn(inp, false);
                more_info = more_info || self.display[E].learn(inp, false);
                more_info = more_info || self.display[F].learn(inp, true);
                more_info = more_info || self.display[G].learn(inp, false);
            }
            4 => {
                more_info = more_info || self.display[A].learn(inp, false);
                more_info = more_info || self.display[B].learn(inp, true);
                more_info = more_info || self.display[C].learn(inp, true);
                more_info = more_info || self.display[D].learn(inp, true);
                more_info = more_info || self.display[E].learn(inp, false);
                more_info = more_info || self.display[F].learn(inp, true);
                more_info = more_info || self.display[G].learn(inp, false);
            }
            _ => (),
        };

        while more_info {
            more_info = false;
            let fixed = self.display.iter().filter_map(|s| s.fixed()).join("");
            for segment in self.display.iter_mut() {
                more_info = more_info || segment.learn(&fixed, false);
            }
        }
    }

    fn solve(&mut self, input: HashMap<usize, Vec<&str>>) {
        input.values().flatten().for_each(|&val| self.learn(val));

        let mut count = HashMap::new();
        let x = input
            .get(&5)
            .unwrap()
            .iter()
            .flat_map(|x| x.chars())
            .sorted_unstable()
            .group_by(|x| *x);
        for (key, group) in &x {
            count
                .entry(group.count())
                .or_insert_with(HashSet::new)
                .insert(key);
        }

        let inp = count.get(&3).unwrap().iter().join("");
        self.display[A].learn(&inp, true);
        self.display[B].learn(&inp, false);
        self.display[C].learn(&inp, false);
        self.display[D].learn(&inp, true);
        self.display[E].learn(&inp, false);
        self.display[F].learn(&inp, false);
        self.display[G].learn(&inp, true);

        let mut count = HashMap::new();
        let x = input
            .get(&6)
            .unwrap()
            .iter()
            .flat_map(|x| x.chars())
            .sorted_unstable()
            .group_by(|x| *x);
        for (key, group) in &x {
            count
                .entry(group.count())
                .or_insert_with(HashSet::new)
                .insert(key);
        }

        let inp = count.get(&3).unwrap().iter().join("");
        self.display[A].learn(&inp, true);
        self.display[B].learn(&inp, true);
        self.display[C].learn(&inp, false);
        self.display[D].learn(&inp, false);
        self.display[E].learn(&inp, false);
        self.display[F].learn(&inp, true);
        self.display[G].learn(&inp, true);

        let mut more_info = true;
        while more_info {
            more_info = false;
            let fixed = self.display.iter().filter_map(|s| s.fixed()).join("");
            for segment in self.display.iter_mut() {
                more_info = more_info || segment.learn(&fixed, false);
            }
        }
    }

    fn read(&self, inp: &str) -> u32 {
        let indices: Vec<_> = self.display.iter().map(|x| x.fixed().unwrap()).collect();

        let f: Vec<_> = inp
            .chars()
            .map(|x| indices.iter().position(|&r| r == x).unwrap())
            .sorted_unstable()
            .collect();
        match f[..] {
            [0, 1, 2, 4, 5, 6] => 0,
            [2, 5] => 1,
            [0, 2, 3, 4, 6] => 2,
            [0, 2, 3, 5, 6] => 3,
            [1, 2, 3, 5] => 4,
            [0, 1, 3, 5, 6] => 5,
            [0, 1, 3, 4, 5, 6] => 6,
            [0, 2, 5] => 7,
            [0, 1, 2, 3, 4, 5, 6] => 8,
            [0, 1, 2, 3, 5, 6] => 9,
            _ => panic!("invalid number"),
        }
    }
}

pub fn part1(reader: Input) -> anyhow::Result<usize> {
    let r = reader
        .lines()
        .map(|line| line.unwrap())
        .flat_map(|line| {
            line.split_once('|')
                .unwrap()
                .1
                .split_whitespace()
                .map(|x| x.len())
                .collect::<Vec<_>>()
        })
        .filter(|len| matches!(len, 2 | 3 | 4 | 7))
        .count();

    Ok(r)
}

pub fn part2(reader: Input) -> anyhow::Result<usize> {
    let r = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (l, r) = line.split_once(" | ").unwrap();
            (l.to_owned(), r.to_owned())
        })
        .map(|(data, value)| {
            let mut ss = SeventSegment::default();
            ss.solve(data.split_whitespace().into_group_map_by(|n| n.len()));
            value
                .split_whitespace()
                .map(|inp| ss.read(inp))
                .join("")
                .parse::<usize>()
                .unwrap()
        })
        .sum();
    Ok(r)
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
