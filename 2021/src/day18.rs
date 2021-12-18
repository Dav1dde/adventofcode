use aoc2021::Input;
use itertools::Itertools;
use std::io::BufRead;

macro_rules! add_left {
    ($num:ident, $value:expr) => {{
        $num.add_left($value);
        $num
    }};
}
macro_rules! add_right {
    ($num:ident, $value:expr) => {{
        $num.add_right($value);
        $num
    }};
}

#[derive(Debug, Clone)]
enum Number {
    Pair(Box<Number>, Box<Number>),
    Literal(u8),
}

impl Number {
    fn pair(n1: Number, n2: Number) -> Self {
        Self::Pair(Box::new(n1), Box::new(n2))
    }

    fn add_left(&mut self, value: u8) {
        match self {
            Self::Literal(v) => *v += value,
            Self::Pair(lhs, _) => lhs.add_left(value),
        }
    }

    fn add_right(&mut self, value: u8) {
        match self {
            Self::Literal(v) => *v += value,
            Self::Pair(_, rhs) => rhs.add_right(value),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Self::Literal(value) => *value as u32,
            Self::Pair(lhs, rhs) => lhs.magnitude() * 3 + rhs.magnitude() * 2,
        }
    }

    fn unwrap(self) -> u8 {
        match self {
            Self::Literal(num) => num,
            _ => panic!("unwrap called on number which is not a literal: {}", self),
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pair(lhs, rhs) => write!(f, "[{},{}]", lhs, rhs),
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

fn reduce(number: Number) -> Number {
    let mut number = Box::new(number);
    loop {
        loop {
            let ex = explode(number, 0);
            let is_done = matches!(ex, Fuse::Cold(_));
            number = ex.unwrap();
            if is_done {
                break;
            }
        }

        if !split(&mut number) {
            break *number;
        }
    }
}

#[derive(Debug)]
enum Fuse {
    Cold(Box<Number>),
    AddLeft(Box<Number>, u8),
    AddRight(Box<Number>, u8),
    AddLeftRight(Box<Number>, u8, u8),
    Done(Box<Number>),
}

impl Fuse {
    fn unwrap(self) -> Box<Number> {
        match self {
            Self::Cold(number) => number,
            Self::AddLeft(number, ..) => number,
            Self::AddRight(number, ..) => number,
            Self::AddLeftRight(number, ..) => number,
            Self::Done(number) => number,
        }
    }
}

fn explode(number: Box<Number>, depth: usize) -> Fuse {
    let (lhs, mut rhs) = match *number {
        Number::Literal(_) => return Fuse::Cold(number),
        Number::Pair(lhs, rhs) => (lhs, rhs),
    };

    if depth == 4 {
        return Fuse::AddLeftRight(Box::new(Number::Literal(0)), lhs.unwrap(), rhs.unwrap());
    }

    match explode(lhs, depth + 1) {
        Fuse::AddLeft(number, lv) => Fuse::AddLeft(Box::new(Number::Pair(number, rhs)), lv),
        Fuse::AddRight(number, rv) => {
            Fuse::Done(Box::new(Number::Pair(number, add_left!(rhs, rv))))
        }
        Fuse::AddLeftRight(number, lv, rv) => {
            Fuse::AddLeft(Box::new(Number::Pair(number, add_left!(rhs, rv))), lv)
        }
        Fuse::Done(number) => Fuse::Done(Box::new(Number::Pair(number, rhs))),
        Fuse::Cold(mut nhls) => match explode(rhs, depth + 1) {
            Fuse::AddLeft(number, rv) => {
                Fuse::Done(Box::new(Number::Pair(add_right!(nhls, rv), number)))
            }
            Fuse::AddRight(number, rv) => Fuse::AddRight(Box::new(Number::Pair(nhls, number)), rv),
            Fuse::AddLeftRight(number, lv, rv) => {
                Fuse::AddRight(Box::new(Number::Pair(add_right!(nhls, lv), number)), rv)
            }
            Fuse::Done(number) => Fuse::Done(Box::new(Number::Pair(nhls, number))),
            Fuse::Cold(nrhs) => Fuse::Cold(Box::new(Number::Pair(nhls, nrhs))),
        },
    }
}

fn split(number: &mut Number) -> bool {
    match number {
        Number::Literal(value) if *value > 9 => {
            let lv = *value / 2;
            *number = Number::pair(
                Number::Literal(lv),
                Number::Literal(*value - lv),
            );
            true
        }
        Number::Literal(_) => false,
        Number::Pair(lhs, rhs) => split(lhs) || split(rhs),
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
