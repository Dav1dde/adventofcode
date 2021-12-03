use aoc2021::Input;
use itertools::Itertools;
use std::io::BufRead;

pub fn part1(reader: Input) -> anyhow::Result<u32> {
    let gamma = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    '0' => -1,
                    '1' => 1,
                    _ => panic!("invalid input"),
                })
                .collect::<Vec<_>>()
        })
        .fold1(|a, b| {
            a.iter()
                .zip(b.iter())
                .map(|(x, y)| x + y)
                .collect::<Vec<_>>()
        })
        .unwrap()
        .iter()
        .map(|x| match *x {
            v if v > 0 => "1",
            v if v < 0 => "0",
            _ => panic!("same amount of 0's and 1's"),
        })
        .join("");

    let len = gamma.len();
    let gamma = u32::from_str_radix(&gamma, 2)?;
    let epsilon = !gamma & ((1 << len) - 1);

    Ok(gamma * epsilon)
}

pub fn part2(reader: Input) -> anyhow::Result<u32> {
    let values = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|x| match x {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("invalid input"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // I guess I really wanted it to be recursive and use .partition() ...
    // There probably is a smart way of sorting the input and partioning by slicing
    let (ones, zeros) = values.iter().partition::<Vec<_>, _>(|value| value[0] == 1);
    let (oxygene, co2) = select(ones, zeros, true);

    let oxygene = u32::from_str_radix(&reduce(&oxygene, 0, true).iter().join(""), 2)?;
    let co2 = u32::from_str_radix(&reduce(&co2, 1, false).iter().join(""), 2)?;

    Ok(oxygene * co2)
}

fn reduce<'a>(values: &[&'a Vec<u8>], index: usize, keep_majority: bool) -> &'a Vec<u8> {
    let (ones, zeros) = values
        .iter()
        .partition::<Vec<_>, _>(|value| value[index] == 1);

    let (winner, _) = select(ones, zeros, keep_majority);

    if winner.len() == 1 {
        winner[0]
    } else {
        reduce(&winner, index + 1, keep_majority)
    }
}

fn select<T>(ones: Vec<T>, zeros: Vec<T>, prefer_majority: bool) -> (Vec<T>, Vec<T>) {
    match (prefer_majority, ones.len() >= zeros.len()) {
        (true, true) => (ones, zeros),
        (false, false) => (ones, zeros),
        _ => (zeros, ones),
    }
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
