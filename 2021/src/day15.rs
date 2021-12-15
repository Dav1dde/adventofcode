use aoc2021::{grid, Input};
use std::collections::BinaryHeap;

type Grid = grid::Grid<u8>;

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

struct Node {
    pos: (usize, usize),
    cost: usize,
    total_cost: usize,
}

impl Node {
    fn new(pos: (usize, usize), cost: usize, estimated: usize) -> Self {
        Self {
            pos,
            cost,
            total_cost: cost + estimated,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.total_cost.cmp(&self.total_cost)
    }
}

fn find_shortest(grid: &Grid) -> Option<usize> {
    let destination = (grid.width() - 1, grid.height() - 1);

    let mut checked = grid::Grid::new(
        vec![false; grid.width() * grid.height()],
        grid.width(),
        grid.height(),
    );
    let mut candidates = BinaryHeap::new();

    let est_cost = |(x, y): (usize, usize)| -> usize {
        abs_diff(x, destination.0) + abs_diff(y, destination.1)
    };
    candidates.push(Node::new((0, 0), 0, est_cost((0, 0))));

    // A*
    loop {
        let current = if let Some(current) = candidates.pop() {
            current
        } else {
            break None;
        };

        if current.pos == destination {
            break Some(current.cost);
        }

        checked[current.pos] = true;

        let (x, y) = current.pos;
        let more_candidates = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            .filter(|&(x, y)| !checked.get(x, y).unwrap_or(&true))
            .map(|new_pos| {
                Node::new(
                    new_pos,
                    current.cost + grid[new_pos] as usize,
                    est_cost(new_pos),
                )
            });

        candidates.extend(more_candidates);
    }
}

pub fn part1(reader: Input) -> anyhow::Result<usize> {
    let grid: Grid = reader.into();
    Ok(find_shortest(&grid).unwrap())
}

pub fn part2(reader: Input) -> anyhow::Result<usize> {
    let grid: Grid = reader.into();
    let width = grid.width();
    let height = grid.height();

    let mut data = Vec::new();
    data.resize(grid.width() * 5 * grid.height() * 5, 0);
    let mut new_grid = Grid::new(data, grid.width() * 5, grid.height() * 5);
    for (x, y, &value) in grid.values() {
        for x_diff in 0..5 {
            for y_diff in 0..5 {
                new_grid[(x + width * x_diff, y + height * y_diff)] =
                    (value + x_diff as u8 + y_diff as u8 - 1) % 9 + 1;
            }
        }
    }

    Ok(find_shortest(&new_grid).unwrap())
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
