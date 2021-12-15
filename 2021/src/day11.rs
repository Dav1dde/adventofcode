use aoc2021::{Input, grid};

type Grid = grid::Grid<u8>;

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
