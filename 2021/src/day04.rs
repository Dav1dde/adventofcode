use aoc2021::Input;
use std::io::BufRead;

enum Field {
    Unmarked(u8),
    Marked(u8),
}

impl Field {
    fn is_marked(&self) -> bool {
        matches!(self, Field::Marked(_))
    }

    fn mark_if(&mut self, expected: u8) {
        let value = self.get_value();
        if value == expected {
            *self = Field::Marked(value);
        }
    }

    fn get_value(&self) -> u8 {
        match self {
            Field::Unmarked(v) => *v,
            Field::Marked(v) => *v,
        }
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Unmarked(v) => write!(f, "{:^5}", v),
            Field::Marked(v) => write!(f, "{:^5}", format!("*{}*", v)),
        }
    }
}

struct Board(Vec<Vec<Field>>);

impl Board {
    fn parse_one(lines: &mut impl Iterator<Item = String>) -> Option<Self> {
        let field = lines
            .skip_while(|line| line.trim().is_empty())
            .take_while(|line| !line.trim().is_empty())
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<u8>().unwrap())
                    .map(|num| Field::Unmarked(num))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        (!field.is_empty()).then(|| Board(field))
    }

    fn mark(&mut self, number: u8) -> bool {
        let mut winning = false;
        let mut cols_winning = vec![true; self.0[0].len()];

        for row in self.0.iter_mut() {
            let mut row_winning = true;
            for (col_i, col) in row.iter_mut().enumerate() {
                col.mark_if(number);
                if !col.is_marked() {
                    row_winning = false;
                    cols_winning[col_i] = false;
                }
            }
            if row_winning {
                winning = true;
            }
        }
        winning || cols_winning.into_iter().any(|x| x)
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        for row in self.0.iter() {
            for col in row.iter() {
                if !col.is_marked() {
                    score = score + col.get_value() as u32;
                }
            }
        }
        score
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for col in row {
                write!(f, "{:?} ", col)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn part1(reader: Input) -> anyhow::Result<u32> {
    let mut input = reader.lines().map(|line| line.unwrap());
    let numbers = input.next().unwrap();

    let mut boards = std::iter::repeat(0)
        .map_while(|_| Board::parse_one(&mut input))
        .collect::<Vec<_>>();

    for number in numbers.split(',').map(|num| num.parse::<u8>().unwrap()) {
        for board in boards.iter_mut() {
            if board.mark(number) {
                return Ok(board.score() * number as u32);
            }
        }
    }

    Ok(0)
}

pub fn part2(reader: Input) -> anyhow::Result<u32> {
    let mut input = reader.lines().map(|line| line.unwrap());
    let numbers = input.next().unwrap();

    let mut boards = std::iter::repeat(0)
        .map_while(|_| Board::parse_one(&mut input))
        .map(Some)
        .collect::<Vec<_>>();

    for number in numbers.split(',').map(|num| num.parse::<u8>().unwrap()) {
        let is_last = boards.len() == 1;
        for board in boards.iter_mut() {
            if board.as_mut().unwrap().mark(number) {
                let board = board.take().unwrap();
                if is_last {
                    return Ok(board.score() * number as u32);
                }
            }
        }
        boards.retain(|b| b.is_some());
    }

    Ok(0)
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
