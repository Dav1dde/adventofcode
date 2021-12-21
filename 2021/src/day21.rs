use aoc2021::Input;
use std::collections::HashMap;
use std::io::BufRead;

trait Dice {
    fn next(&mut self) -> u32;
    fn rolled(&self) -> u32;
}

struct DeterministicDice {
    rolled: u32,
    next: u32,
}

impl DeterministicDice {
    fn new() -> Self {
        Self { rolled: 0, next: 0 }
    }
}

impl Default for DeterministicDice {
    fn default() -> Self {
        Self::new()
    }
}

impl Dice for DeterministicDice {
    fn next(&mut self) -> u32 {
        self.next += 1;
        self.rolled += 1;
        self.next
    }

    fn rolled(&self) -> u32 {
        self.rolled
    }
}

type Player = (u32, u32);

fn quantum_game(
    player1: u32,
    player2: u32,
    player1_score: u32,
    player2_score: u32,
    cache: &mut HashMap<(Player, Player), (u64, u64)>,
) -> (u64, u64) {
    if let Some(&winnings) = cache.get(&((player1, player1_score), (player2, player2_score))) {
        return winnings;
    }

    let mut total_p1_wins = 0;
    let mut total_p2_wins = 0;
    for x in 1..=3 {
        for y in 1..=3 {
            for z in 1..=3 {
                let p1 = (player1 + x + y + z - 1) % 10 + 1;
                let p1_score = player1_score + p1;

                if p1_score >= 21 {
                    total_p1_wins += 1;
                    continue;
                }

                for x2 in 1..=3 {
                    for y2 in 1..=3 {
                        for z2 in 1..=3 {
                            let p2 = (player2 + x2 + y2 + z2 - 1) % 10 + 1;
                            let p2_score = player2_score + p2;

                            if p2_score >= 21 {
                                total_p2_wins += 1;
                                continue;
                            }

                            let (p1w, p2w) = quantum_game(p1, p2, p1_score, p2_score, cache);
                            total_p1_wins += p1w;
                            total_p2_wins += p2w;
                        }
                    }
                }
            }
        }
    }

    cache.insert(
        ((player1, player1_score), (player2, player2_score)),
        (total_p1_wins, total_p2_wins),
    );

    (total_p1_wins, total_p2_wins)
}

fn parse_player(line: &str) -> u32 {
    (line.as_bytes()[28] - b'0') as u32
}

pub fn part1(reader: Input) -> anyhow::Result<u64> {
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut player1 = parse_player(&lines.next().unwrap());
    let mut player1_score = 0;
    let mut player2 = parse_player(&lines.next().unwrap());
    let mut player2_score = 0;

    let mut dice = DeterministicDice::new();
    let (_winner, loser) = loop {
        player1 = (player1 + dice.next() + dice.next() + dice.next() - 1) % 10 + 1;
        player1_score += player1;
        if player1_score >= 1000 {
            break (player1_score, player2_score);
        }
        player2 = (player2 + dice.next() + dice.next() + dice.next() - 1) % 10 + 1;
        player2_score += player2;
        if player2_score >= 1000 {
            break (player2_score, player1_score);
        }
    };

    Ok((loser * dice.rolled()) as u64)
}

pub fn part2(reader: Input) -> anyhow::Result<u64> {
    let mut lines = reader.lines().map(|line| line.unwrap());
    let player1 = parse_player(&lines.next().unwrap());
    let player1_score = 0;
    let player2 = parse_player(&lines.next().unwrap());
    let player2_score = 0;

    let result = quantum_game(
        player1,
        player2,
        player1_score,
        player2_score,
        &mut HashMap::new(),
    );
    Ok(result.0.max(result.1))
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
