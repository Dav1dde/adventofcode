use aoc2021::Input;
use std::io::BufRead;

type Board = Vec<Vec<u8>>;

fn transpose<T: Copy>(inp: &[Vec<T>]) -> Vec<Vec<T>> {
    (0..inp.len())
        .map(|i| inp.iter().map(|x| x[i]).collect())
        .collect()
}

fn parse_one(lines: &mut impl Iterator<Item = String>) -> Option<Board> {
    let mut board = lines
        .skip_while(|line| line.trim().is_empty())
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if board.is_empty() {
        return None;
    }

    board.extend(transpose(&board));
    Some(board)
}

fn score(board: &Board, numbers: &[u8]) -> u32 {
    board[0..board.len() / 2]
        .iter()
        .flatten()
        .filter(|n| !numbers.contains(n))
        .map(|x| *x as u32)
        .sum()
}

pub fn part1(reader: Input) -> anyhow::Result<u32> {
    let mut input = reader.lines().map(|line| line.unwrap());
    let numbers: Vec<_> = input
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect();
    let boards = std::iter::repeat(0)
        .map_while(|_| parse_one(&mut input))
        .collect::<Vec<_>>();

    for i in boards[0][0].len()..numbers.len() {
        let slice = &numbers[0..i];

        for board in boards.iter() {
            for solution in board.iter() {
                if solution.iter().all(|x| slice.contains(x)) {
                    return Ok(score(board, slice) * numbers[i - 1] as u32);
                }
            }
        }
    }

    anyhow::bail!("no solution found")
}

pub fn part2(reader: Input) -> anyhow::Result<u32> {
    let mut input = reader.lines().map(|line| line.unwrap());
    let numbers: Vec<_> = input
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect();
    let mut boards = std::iter::repeat(0)
        .map_while(|_| parse_one(&mut input))
        .map(Some)
        .collect::<Vec<_>>();

    for i in boards[0].as_ref().unwrap()[0].len()..numbers.len() {
        let slice = &numbers[0..i];

        let is_last = boards.len() == 1;
        for board in boards.iter_mut() {
            for solution in board.as_ref().unwrap().iter() {
                if solution.iter().all(|x| slice.contains(x)) {
                    let board = board.take().unwrap();
                    if is_last {
                        return Ok(score(&board, slice) * numbers[i - 1] as u32);
                    }
                    break;
                }
            }
        }
        boards.retain(|b| b.is_some());
    }

    anyhow::bail!("no solution found")
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
