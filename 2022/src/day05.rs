use aoc2022::Input;
use std::{collections::VecDeque, io::BufRead};

#[derive(Default, Debug)]
struct Board([VecDeque<u8>; 10]);

impl Board {
    fn push_below(&mut self, column: usize, value: u8) {
        self.0[column].push_front(value);
    }

    fn r#move(&mut self, amount: usize, from: usize, to: usize) {
        for _ in 0..amount {
            let value = self.0[from].pop_back().unwrap();
            self.0[to].push_back(value);
        }
    }

    fn move_efficient(&mut self, amount: usize, from: usize, to: usize) {
        debug_assert!(from != to);
        let (from, to) = if from < to {
            let (a, b) = self.0.split_at_mut(from + 1);
            (&mut a[from], &mut b[to - from - 1])
        } else {
            let (a, b) = self.0.split_at_mut(to + 1);
            (&mut b[from - to - 1], &mut a[to])
        };

        let from_slice = &*from.make_contiguous();
        let new_from_len = from_slice.len() - amount;
        to.extend(&from_slice[new_from_len..]);
        from.truncate(new_from_len);
    }

    fn tops(&self) -> String {
        self.0
            .iter()
            .filter_map(|stack| stack.back())
            .map(|&b| b as char)
            .collect()
    }
}

fn parse_board(lines: &mut impl Iterator<Item = String>) -> Board {
    let mut board = Board::default();

    for line in lines {
        let line = line.into_bytes();

        let Some(start) = line.iter().position(|&b| b == b'[') else { return board };

        for index in (start + 1..line.len()).step_by(4) {
            let column = index / 4;
            let value = line[index];
            if value.is_ascii_uppercase() {
                board.push_below(column, value);
            }
        }
    }

    board
}

pub fn part1(reader: Input) -> anyhow::Result<String> {
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut board = parse_board(&mut lines);

    for instruction in lines.skip(1) {
        let s = instruction.as_str();
        let (amount, from, to): (usize, usize, usize) =
            serde_scan::scan!("move {} from {} to {}" <- s).unwrap();
        board.r#move(amount, from - 1, to - 1);
    }

    Ok(board.tops())
}

pub fn part2(reader: Input) -> anyhow::Result<String> {
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut board = parse_board(&mut lines);

    for instruction in lines.skip(1) {
        let s = instruction.as_str();
        let (amount, from, to): (usize, usize, usize) =
            serde_scan::scan!("move {} from {} to {}" <- s).unwrap();
        board.move_efficient(amount, from - 1, to - 1);
    }

    Ok(board.tops())
}

pub fn main() {
    aoc2022::cli::run(part1, part2).unwrap();
}
