use aoc2021::Input;
use either::{Left, Right};
use itertools::Itertools;
use std::cmp;
use std::io::BufRead;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn parse(inp: &str) -> Self {
        let (x, y) = inp.split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn iter(&self) -> impl Iterator<Item = Point> {
        if self.from.x == self.to.x {
            let x = self.from.x;
            return Left(
                (cmp::min(self.from.y, self.to.y)..cmp::max(self.from.y, self.to.y) + 1)
                    .map(move |y| Point { x, y }),
            );
        }
        let m = (self.from.y - self.to.y) / (self.from.x - self.to.x);
        let t = self.from.y - m * self.from.x;

        Right(
            (cmp::min(self.from.x, self.to.x)..cmp::max(self.from.x, self.to.x) + 1)
                .map(move |x| Point { x, y: m * x + t }),
        )
    }

    fn parse(inp: &str) -> Self {
        let (left, right) = inp.split_once(" -> ").unwrap();
        Self {
            from: Point::parse(left),
            to: Point::parse(right),
        }
    }

    fn is_horz_or_vert(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }
}

pub fn part1(reader: Input) -> anyhow::Result<usize> {
    let r = reader
        .lines()
        .map(|line| Line::parse(&line.unwrap()))
        .filter(|line| line.is_horz_or_vert())
        .flat_map(|line| line.iter())
        .into_grouping_map_by(|point| *point)
        .fold(0, |count, _key, _value| count + 1)
        .values()
        .filter(|count| **count > 1)
        .count();

    Ok(r)
}

pub fn part2(reader: Input) -> anyhow::Result<usize> {
    let r = reader
        .lines()
        .map(|line| Line::parse(&line.unwrap()))
        .flat_map(|line| line.iter())
        .into_grouping_map_by(|point| *point)
        .fold(0, |count, _key, _value| count + 1)
        .values()
        .filter(|count| **count > 1)
        .count();

    Ok(r)
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
