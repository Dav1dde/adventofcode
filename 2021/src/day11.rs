use aoc2021::Input;
use std::io::Read;

#[derive(Debug)]
struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

#[allow(dead_code)]
impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = x + y * self.width;
        (index < self.data.len()).then(|| self.data[index])
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut u8> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = x + y * self.width;
        (index < self.data.len()).then(|| &mut self.data[index])
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn indices(&self) -> impl Iterator<Item = (usize, usize)> {
        let width = self.width;
        (0..self.height).flat_map(move |y| std::iter::repeat(y).zip(0..width))
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.data.len() {
            if i > 0 && i % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", self.data[i])?;
        }
        Ok(())
    }
}

impl<T: Read> std::convert::From<T> for Grid {
    fn from(reader: T) -> Grid {
        let mut width = 0;
        let data: Vec<_> = reader
            .bytes()
            .enumerate()
            .filter_map(|(i, v)| {
                let v = v.unwrap();
                let relevant = (b'0'..=b'9').contains(&v);
                if width == 0 && !relevant {
                    width = i;
                }
                relevant.then(|| v - b'0')
            })
            .collect();
        let height = data.len() / width;

        Grid {
            data,
            width,
            height,
        }
    }
}

impl std::ops::Index<usize> for Grid {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::Index<(usize, usize)> for Grid {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[x + y * self.width]
    }
}

impl std::ops::IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[x + y * self.width]
    }
}

macro_rules! try_flash {
    ($grid:ident, $x:expr, $y: expr) => {
        if let Some(value) = $grid.get_mut($x, $y) {
            *value += 1;
            if *value == 10 {
                flash(&mut $grid, $x, $y)
            }
        }
    };
}

fn flash(mut grid: &mut Grid, x: usize, y: usize) {
    try_flash!(grid, x - 1, y);
    try_flash!(grid, x + 1, y);
    try_flash!(grid, x, y - 1);
    try_flash!(grid, x, y + 1);
    try_flash!(grid, x - 1, y - 1);
    try_flash!(grid, x - 1, y + 1);
    try_flash!(grid, x + 1, y - 1);
    try_flash!(grid, x + 1, y + 1);
}

pub fn part1(reader: Input) -> anyhow::Result<usize> {
    let mut grid: Grid = reader.into();

    let mut flashes = 0;
    for _ in 0..100 {
        for (x, y) in grid.indices() {
            try_flash!(grid, x, y);
        }

        for (x, y) in grid.indices() {
            if grid[(x, y)] > 9 {
                grid[(x, y)] = 0;
                flashes += 1;
            }
        }
    }

    Ok(flashes)
}

pub fn part2(reader: Input) -> anyhow::Result<usize> {
    let mut grid: Grid = reader.into();

    for step in 1.. {
        for (x, y) in grid.indices() {
            try_flash!(grid, x, y);
        }

        let mut flashes = 0;
        for (x, y) in grid.indices() {
            if grid[(x, y)] > 9 {
                grid[(x, y)] = 0;
                flashes += 1;
            }
        }

        if flashes == grid.size() {
            return Ok(step);
        }
    }

    unreachable!();
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
