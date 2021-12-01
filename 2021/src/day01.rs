use aoc2021::Input;
use itertools::Itertools;
use std::io::BufRead;

pub fn part1(reader: Input) -> anyhow::Result<u32> {
    let r = reader
        .lines()
        .map(|x| x.unwrap().parse::<u32>().unwrap())
        .scan(None, |state, x| {
            let previous = state.take();
            *state = Some(x);

            match previous {
                Some(previous) if previous < x => Some(1),
                _ => Some(0),
            }
        })
        .sum();

    Ok(r)
}

pub fn part2(reader: Input) -> anyhow::Result<u32> {
    let r = reader
        .lines()
        .map(|x| x.unwrap().parse::<u32>().unwrap())
        // Pad with an additional useless value to yield the last window,
        // since we're using 4 size windows but we only really need 3 values
        // for a full window.
        .chain(std::iter::once(0))
        .tuple_windows()
        .scan(None, |state, (a, b, c, _)| {
            let previous = state.take();
            let sum = a + b + c;
            *state = Some(sum);

            match previous {
                Some(previous) if previous < sum => Some(1),
                _ => Some(0),
            }
        })
        .sum();

    Ok(r)
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
