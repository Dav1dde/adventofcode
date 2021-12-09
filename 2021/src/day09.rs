use aoc2021::Input;
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

struct Image {
    pixels: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Image {
    fn read(reader: Input) -> Self {
        let pixels: Vec<Vec<_>> = reader
            .lines()
            .map(|line| line.unwrap())
            .map(|line| line.bytes().map(|b| b - b'0').collect())
            .collect();

        let height = pixels.len();
        let width = pixels[0].len();

        Self {
            pixels,
            width,
            height,
        }
    }

    fn kernel(&self, border: usize) -> impl Iterator<Item = (u8, (usize, usize), Vec<&[u8]>)> {
        // I had this idea and it turns out it's not so great but I stuck with it anyways
        let rows = self.height;
        let columns = self.width;
        (0..rows)
            .flat_map(move |row| std::iter::repeat(row).zip(0..columns))
            .map(move |(row, column)| {
                let r = (0.max(row.saturating_sub(border))..rows.min(row + border + 1))
                    .map(|brow| {
                        let bcol =
                            0.max(column.saturating_sub(border))..columns.min(column + border + 1);
                        &self.pixels[brow][bcol]
                    })
                    .collect();
                (self.pixels[row][column], (row, column), r)
            })
    }

    fn basin_size(&self, pos: (usize, usize)) -> usize {
        let mut to_check = VecDeque::new();
        to_check.push_front(pos);
        let mut seen = HashSet::new();

        let mut count = 0;
        while let Some((row, column)) = to_check.pop_front() {
            let value = self.pixels[row][column];
            count += 1;

            let mut check_pos = |current, (row, column)| {
                if self
                    .get((row, column))
                    .filter(|&val| val < 9 && val > current)
                    .is_some()
                    && seen.insert((row, column))
                {
                    to_check.push_back((row as usize, column as usize));
                }
            };

            let row = row as i32;
            let column = column as i32;
            check_pos(value, (row - 1, column));
            check_pos(value, (row + 1, column));
            check_pos(value, (row, column - 1));
            check_pos(value, (row, column + 1));
        }

        count
    }

    fn get(&self, (row, column): (i32, i32)) -> Option<u8> {
        if row < 0 || row >= self.height as i32 || column < 0 || column >= self.width as i32 {
            None
        } else {
            Some(self.pixels[row as usize][column as usize])
        }
    }
}

pub fn part1(reader: Input) -> anyhow::Result<usize> {
    let image = Image::read(reader);

    let r = image
        .kernel(1)
        .filter(|(value, _, k)| value == k.iter().copied().flatten().min().unwrap())
        .map(|(v, _, _)| v as usize + 1)
        .sum();

    Ok(r)
}

pub fn part2(reader: Input) -> anyhow::Result<usize> {
    let image = Image::read(reader);

    let r = image
        .kernel(1)
        .filter(|(value, _, k)| value == k.iter().copied().flatten().min().unwrap())
        .map(|(_, pos, _)| image.basin_size(pos))
        .sorted_unstable_by_key(|x| usize::MAX - x)
        .take(3)
        .reduce(|a, b| a * b)
        .unwrap();

    Ok(r)
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
