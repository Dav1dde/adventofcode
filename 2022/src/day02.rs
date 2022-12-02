use aoc2022::Input;
use std::{io::BufRead, str::FromStr};

#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn weight(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err("invalid move"),
        }
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Loss => 0,
            Self::Draw => 3,
        }
    }
}

impl FromStr for Outcome {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err("invalid outcome"),
        }
    }
}

fn parse<T: FromStr>(reader: Input) -> impl Iterator<Item = (Move, T)>
where
    T::Err: std::fmt::Debug,
{
    reader.lines().map(|line| line.unwrap()).map(|line| {
        let (mov, b) = line.split_once(' ').unwrap();
        (mov.parse().unwrap(), b.parse().unwrap())
    })
}

fn play(enemy: Move, player: Move) -> Outcome {
    use Move::*;
    use Outcome::*;

    match (enemy, player) {
        (Rock, Rock) => Draw,
        (Rock, Paper) => Win,
        (Rock, Scissors) => Loss,
        (Paper, Rock) => Loss,
        (Paper, Paper) => Draw,
        (Paper, Scissors) => Win,
        (Scissors, Rock) => Win,
        (Scissors, Paper) => Loss,
        (Scissors, Scissors) => Draw,
    }
}

fn what_to_play(enemy: Move, outcome: Outcome) -> Move {
    use Move::*;
    use Outcome::*;

    match (enemy, outcome) {
        (Rock, Win) => Paper,
        (Rock, Loss) => Scissors,
        (Rock, Draw) => Rock,
        (Paper, Win) => Scissors,
        (Paper, Loss) => Rock,
        (Paper, Draw) => Paper,
        (Scissors, Win) => Rock,
        (Scissors, Loss) => Paper,
        (Scissors, Draw) => Scissors,
    }
}

pub fn part1(reader: Input) -> anyhow::Result<u32> {
    let score = parse(reader)
        .map(|(enemy, player)| play(enemy, player).score() + player.weight())
        .sum();

    Ok(score)
}

pub fn part2(reader: Input) -> anyhow::Result<u32> {
    let score = parse(reader)
        .map(|(enemy, outcome)| what_to_play(enemy, outcome).weight() + outcome.score())
        .sum();

    Ok(score)
}

pub fn main() {
    aoc2022::cli::run(part1, part2).unwrap();
}
