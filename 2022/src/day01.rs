use aoc2022::Input;
use std::io::BufRead;

struct Max<const N: usize>([u32; N]);

impl<const N: usize> Max<N> {
    pub fn new() -> Self {
        Self([0; N])
    }

    pub fn push(&mut self, new_val: u32) {
        for v in self.0.iter_mut().rev() {
            if *v < new_val {
                *v = new_val;
                break;
            }
        }
    }

    pub fn sum(&self) -> u32 {
        self.0.iter().sum()
    }
}

impl<const N: usize> Default for Max<N> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn part1(reader: Input) -> anyhow::Result<u32> {
    let mut current = 0;
    let mut max = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            max = max.max(current);
            current = 0;
        } else {
            current = current + line.parse::<u32>().unwrap();
        }
    }

    Ok(max)
}

pub fn part2(reader: Input) -> anyhow::Result<u32> {
    let mut current = 0;
    let mut max = Max::<3>::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            max.push(current);
            current = 0;
        } else {
            current = current + line.parse::<u32>().unwrap();
        }
    }

    Ok(max.sum())
}

pub fn main() {
    aoc2022::cli::run(part1, part2).unwrap();
}
