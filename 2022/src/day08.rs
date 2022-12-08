use aoc2022::{grid::Grid, Input};

const TOP: usize = 0;
const VISIBLE_TOP: u8 = 1 << TOP;
const RIGHT: usize = 1;
const VISIBLE_RIGHT: u8 = 1 << RIGHT;
const BOTTOM: usize = 2;
const VISIBLE_BOTTOM: u8 = 1 << BOTTOM;
const LEFT: usize = 3;
const VISIBLE_LEFT: u8 = 1 << LEFT;

#[derive(Debug, Copy, Clone)]
struct Tree {
    height: u8,
    visible: u8,
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.visible > 0 {
            write!(f, "{}", self.height)
        } else {
            write!(f, " ")
        }
    }
}

impl From<u8> for Tree {
    fn from(val: u8) -> Self {
        Self {
            height: val - b'0',
            visible: 0,
        }
    }
}

fn check_visibility<'a>(mut iter: impl Iterator<Item = &'a mut Tree>, mask: u8) {
    iter.try_fold(None, |max, mut a| {
        let is_visible = max.is_none() || matches!(max, Some(max) if max < a.height);
        if is_visible {
            a.visible |= mask;
        }
        Some(max.unwrap_or(0).max(a.height))
            .filter(|&m| m < 9)
            .map(Some)
    });
}

#[derive(Debug, Copy, Clone)]
struct Tree2 {
    height: u8,
    range: [u8; 4],
}

impl From<u8> for Tree2 {
    fn from(val: u8) -> Self {
        Self {
            height: val - b'0',
            range: Default::default(),
        }
    }
}

fn check_scenic<'a>(iter: impl Iterator<Item = &'a mut Tree2>, direction: usize) {
    type Scratch = [u8; 10];
    iter.enumerate()
        .fold(Scratch::default(), |mut scratch, (index, a)| {
            let x = scratch[a.height as usize..]
                .iter()
                .max()
                .copied()
                .unwrap_or(0);
            a.range[direction] = index as u8 - x;
            scratch[a.height as usize] = index as u8;
            scratch
        });
}

pub fn part1(mut reader: Input) -> anyhow::Result<usize> {
    let mut grid = Grid::<Tree>::read(&mut reader);

    for row in 0..grid.height() {
        check_visibility(grid.row_mut(row), VISIBLE_LEFT);
        check_visibility(grid.row_rev_mut(row), VISIBLE_RIGHT);
    }

    for col in 0..grid.width() {
        check_visibility(grid.columns_mut(col), VISIBLE_TOP);
        check_visibility(grid.columns_rev_mut(col), VISIBLE_BOTTOM);
    }

    Ok(grid.data().iter().filter(|t| t.visible > 0).count())
}

pub fn part2(mut reader: Input) -> anyhow::Result<usize> {
    let mut grid = Grid::<Tree2>::read(&mut reader);

    for row in 0..grid.height() {
        check_scenic(grid.row_mut(row), LEFT);
        check_scenic(grid.row_rev_mut(row), RIGHT);
    }

    for col in 0..grid.width() {
        check_scenic(grid.columns_mut(col), TOP);
        check_scenic(grid.columns_rev_mut(col), BOTTOM);
    }

    let r = grid
        .data()
        .iter()
        .map(|t| t.range.iter().fold(1usize, |acc, &i| acc * i as usize))
        .max()
        .unwrap();

    Ok(r)
}

pub fn main() {
    aoc2022::cli::run(part1, part2).unwrap();
}
