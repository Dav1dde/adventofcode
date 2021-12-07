use aoc2021::Input;
use itertools::Itertools;
use std::io::BufRead;

fn abs_diff(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn search_minimum<F: Fn(u32) -> u32>(mut min: u32, mut max: u32, mapper: F) -> u32 {
    let mut min_val = mapper(min);
    let mut max_val = mapper(max);

    loop {
        if (max - min) <= 1 {
            break min_val.min(max_val);
        }

        let current = min + ((max - min) / 2);
        let current_val = mapper(current);

        if current_val < min_val {
            if current_val < max_val {
                // current value is somewhere between min and max, but it could still be on either
                // side, find the right side by calculating one border
                // 20 ... 10 ... 20
                // 20 .5. 10 ... 20
                // 20 ... 10 .5. 20
                let next = mapper(current + 1);
                if next < current_val {
                    min = current;
                    min_val = current_val;
                } else {
                    max = current;
                    max_val = current_val;
                }
            } else {
                min = current;
                min_val = current_val;
            }
        } else {
            max = current;
            max_val = current_val;
        }
    }
}

fn fuel_cost(target: u32, input: &[u32]) -> u32 {
    input.iter().map(|x| abs_diff(target, *x)).sum()
}

enum BinSearch<T> {
    Left(T),
    Right(T),
}

impl<T> BinSearch<T> {
    fn get_value(self) -> T {
        match self {
            Self::Left(result) => result,
            Self::Right(result) => result,
        }
    }
}

fn search_minimum2<F: Fn(u32) -> BinSearch<T>, T>(mut min: u32, mut max: u32, mapper: F) -> T {
    loop {
        let current = min + ((max - min) / 2);
        let is_last = min == current;
        match mapper(current) {
            BinSearch::Left(value) => {
                max = current;
                if is_last {
                    break value;
                }
            }
            BinSearch::Right(_) => {
                min = current;
                if is_last {
                    break mapper(max).get_value();
                }
            }
        }
    }
}

fn fuel_cost2(target: u32, input: &[u32]) -> BinSearch<u32> {
    let mut value = 0;
    let mut right = 0;

    for &x in input {
        let v_diff = abs_diff(target, x);
        let r_diff = abs_diff(target + 1, x);

        value += v_diff * (v_diff + 1) / 2;
        right += r_diff * (r_diff + 1) / 2;
    }

    if value < right {
        BinSearch::Left(value)
    } else {
        BinSearch::Right(value)
    }
}

pub fn part1(reader: Input) -> anyhow::Result<u32> {
    let input: Vec<u32> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let (min, max) = input.iter().minmax().into_option().unwrap();
    Ok(search_minimum(*min, *max, |x| fuel_cost(x, &input)))
}

pub fn part2(reader: Input) -> anyhow::Result<u32> {
    let input: Vec<u32> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let (min, max) = input.iter().minmax().into_option().unwrap();
    Ok(search_minimum2(*min, *max, |x| fuel_cost2(x, &input)))
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
