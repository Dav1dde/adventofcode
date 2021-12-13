use aoc2021::Input;
use std::collections::HashSet;
use std::fmt::Display;
use std::io::BufRead;

#[derive(Debug)]
struct Paper {
    dots: HashSet<(u64, u64)>,
    max_x: u64,
    max_y: u64,
}

impl Paper {
    fn read(reader: &mut Input) -> Self {
        let mut dots = HashSet::new();
        let lines = reader.lines().map(|line| line.unwrap());

        let mut max_x = 0;
        let mut max_y = 0;
        for line in lines.take_while(|x| !x.is_empty()) {
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse::<u64>().unwrap();
            let y = y.parse::<u64>().unwrap();
            dots.insert((x, y));
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        Self { dots, max_x, max_y }
    }

    fn fold_y(&mut self, fold_y: u64) {
        self.dots = self
            .dots
            .iter()
            .map(|&(x, y)| {
                if y > fold_y {
                    (x, 2 * fold_y - y)
                } else {
                    (x, y)
                }
            })
            .collect();
        self.max_y = fold_y;
    }

    fn fold_x(&mut self, fold_x: u64) {
        self.dots = self
            .dots
            .iter()
            .map(|&(x, y)| {
                if x > fold_x {
                    (2 * fold_x - x, y)
                } else {
                    (x, y)
                }
            })
            .collect();
        self.max_x = fold_x;
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                if self.dots.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part1(mut reader: Input) -> anyhow::Result<String> {
    let mut paper = Paper::read(&mut reader);

    if let Some(line) = reader.lines().map(|line| line.unwrap()).next() {
        match line.trim_start_matches("fold along ").split_once("=") {
            Some(("x", value)) => paper.fold_x(value.parse().unwrap()),
            Some(("y", value)) => paper.fold_y(value.parse().unwrap()),
            _ => unreachable!(),
        }
    }

    Ok(paper.dots.len().to_string())
}

pub fn part2(mut reader: Input) -> anyhow::Result<String> {
    let mut paper = Paper::read(&mut reader);

    for line in reader.lines().map(|line| line.unwrap()) {
        match line.trim_start_matches("fold along ").split_once("=") {
            Some(("x", value)) => paper.fold_x(value.parse().unwrap()),
            Some(("y", value)) => paper.fold_y(value.parse().unwrap()),
            _ => unreachable!(),
        }
    }

    Ok(paper.to_string())
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
