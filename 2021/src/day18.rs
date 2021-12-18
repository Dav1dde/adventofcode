use aoc2021::Input;
use itertools::Itertools;
use std::io::BufRead;

#[derive(Debug, Clone)]
struct Pair {
    lhs: Number,
    rhs: Number,
}

#[derive(Debug, Clone)]
enum Number {
    Pair(Box<Pair>),
    Literal(u8),
}

impl Number {
    fn pair(lhs: Number, rhs: Number) -> Self {
        Self::Pair(Box::new(Pair { lhs, rhs }))
    }

    fn add_left(&mut self, value: u8) {
        match self {
            Self::Literal(v) => *v += value,
            Self::Pair(pair) => pair.lhs.add_left(value),
        }
    }

    fn add_right(&mut self, value: u8) {
        match self {
            Self::Literal(v) => *v += value,
            Self::Pair(pair) => pair.rhs.add_right(value),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Self::Literal(value) => *value as u32,
            Self::Pair(pair) => pair.lhs.magnitude() * 3 + pair.rhs.magnitude() * 2,
        }
    }

    fn unwrap_value(&self) -> u8 {
        match self {
            Self::Literal(num) => *num,
            _ => panic!("unwrap called on number which is not a literal: {}", self),
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pair(pair) => write!(f, "[{},{}]", pair.lhs, pair.rhs),
            Self::Literal(value) => write!(f, "{}", value),
        }
    }
}

impl std::ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        Self::pair(self, rhs)
    }
}

fn reduce(mut number: Number) -> Number {
    loop {
        while !matches!(explode(&mut number, 0), Fuse::Cold) {}

        if !split(&mut number) {
            break number;
        }
    }
}

#[derive(Debug)]
enum Fuse {
    Cold,
    AddLeft(u8),
    AddRight(u8),
    AddLeftRight(u8, u8),
    Done,
}

fn explode(number: &mut Number, depth: usize) -> Fuse {
    let (lhs, rhs) = match number {
        Number::Literal(_) => return Fuse::Cold,
        Number::Pair(pair) => {
            if depth == 4 {
                let r = Fuse::AddLeftRight(pair.lhs.unwrap_value(), pair.rhs.unwrap_value());
                *number = Number::Literal(0);
                return r;
            }
            (&mut pair.lhs, &mut pair.rhs)
        }
    };

    match explode(lhs, depth + 1) {
        Fuse::AddRight(rv) => {
            rhs.add_left(rv);
            Fuse::Done
        }
        Fuse::AddLeftRight(lv, rv) => {
            rhs.add_left(rv);
            Fuse::AddLeft(lv)
        }
        Fuse::Cold => match explode(rhs, depth + 1) {
            Fuse::AddLeft(lv) => {
                lhs.add_right(lv);
                Fuse::Done
            }
            Fuse::AddLeftRight(lv, rv) => {
                lhs.add_right(lv);
                Fuse::AddRight(rv)
            }
            fuse => fuse,
        },
        fuse => fuse,
    }
}

fn split(number: &mut Number) -> bool {
    match number {
        Number::Literal(value) if *value > 9 => {
            let lv = *value / 2;
            *number = Number::pair(Number::Literal(lv), Number::Literal(*value - lv));
            true
        }
        Number::Literal(_) => false,
        Number::Pair(pair) => split(&mut pair.lhs) || split(&mut pair.rhs),
    }
}

fn parse(input: &mut impl Iterator<Item = u8>) -> Number {
    match input.next() {
        Some(b'[') => {
            let number1 = parse(input);
            assert!(matches!(input.next(), Some(b',')));
            let number2 = parse(input);
            assert!(matches!(input.next(), Some(b']')));
            Number::pair(number1, number2)
        }
        Some(v @ b'0'..=b'9') => Number::Literal(v - b'0'),
        _ => unreachable!(),
    }
}

pub fn part1(reader: Input) -> anyhow::Result<u32> {
    let r = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse(&mut line.as_bytes().iter().copied()))
        .reduce(|a, b| reduce(a + b))
        .unwrap();

    Ok(r.magnitude())
}

pub fn part2(reader: Input) -> anyhow::Result<u32> {
    let r = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse(&mut line.as_bytes().iter().copied()))
        .collect_vec()
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| {
            reduce(a.clone() + b.clone())
                .magnitude()
                .max(reduce(b + a).magnitude())
        })
        .max()
        .unwrap();

    Ok(r)
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
