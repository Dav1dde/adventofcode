use aoc2021::Input;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;

fn calculate(start: u8, days: usize) -> u32 {
    let mut fishes = vec![0];
    fishes.reserve(days * days);
    for _ in 0..(days - start as usize) {
        let mut new_fish = vec![];
        for fish in fishes.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fish.push(8);
            } else {
                *fish -= 1;
            }
        }
        fishes.extend(new_fish);
    }
    fishes.len() as u32
}

fn calculate2(fish: u8, days: i64, cache: &mut HashMap<i64, u64>) -> u64 {
    1 + calculate2internal(days - fish as i64, cache)
}

fn calculate2internal(days: i64, cache: &mut HashMap<i64, u64>) -> u64 {
    if days <= 0 {
        return 0;
    }

    let fish1 = days - 7;
    let fish2 = days - 9;
    let a = if let Some(value) = cache.get(&fish1) {
        *value
    } else {
        let x = calculate2internal(fish1, cache);
        cache.insert(fish1, x);
        x
    };
    let b = if let Some(value) = cache.get(&fish2) {
        *value
    } else {
        let x = calculate2internal(fish2, cache);
        cache.insert(fish2, x);
        x
    };
    a + b + 1
}

pub fn part1(reader: Input) -> anyhow::Result<u64> {
    let input: Vec<_> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    let calculated: HashMap<_, _> = input
        .iter()
        .dedup()
        .map(|x| (*x, calculate(*x, 80)))
        .collect();

    let result = input
        .iter()
        .map(|x| calculated.get(x).unwrap())
        .copied()
        .sum::<u32>();

    Ok(result as u64)
}

pub fn part2(reader: Input) -> anyhow::Result<u64> {
    let mut cache = HashMap::new();
    let result = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .map(move |x| calculate2(x, 256, &mut cache))
        .sum();

    Ok(result)
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
