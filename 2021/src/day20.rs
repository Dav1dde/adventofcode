use aoc2021::grid::Grid;
use aoc2021::Input;
use itertools::Itertools;
use std::io::BufRead;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Pixel {
    Light,
    Dark,
}

impl Pixel {
    fn as_bit(&self) -> u8 {
        match self {
            Self::Light => 1,
            Self::Dark => 0,
        }
    }
}

impl From<u8> for Pixel {
    fn from(data: u8) -> Self {
        match data {
            b'#' => Pixel::Light,
            b'.' => Pixel::Dark,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Light => write!(f, "#"),
            Self::Dark => write!(f, "."),
        }
    }
}

fn read_table(reader: &mut Input) -> Vec<Pixel> {
    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .as_bytes()
        .iter()
        .copied()
        .map(Into::into)
        .collect()
}

fn enhance(mut image: Grid<Pixel>, table: &[Pixel], iterations: usize) -> usize {
    let mut def = Pixel::Dark;
    for _ in 0..iterations {
        let data = image
            .kernel_ext_3x3(def)
            .map(|(_, k)| {
                let index = k
                    .map(|pixel| pixel.as_bit() as u16)
                    .into_iter()
                    .reduce(|a, b| a << 1 | b)
                    .unwrap();
                table[index as usize]
            })
            .collect_vec();

        image = Grid::new(data, image.width() + 2, image.height() + 2);

        if table[0] != Pixel::Dark {
            def = match def {
                // all light pixels -> is 0b111_111_111 -> swap to the pixel at that index
                Pixel::Light => table[0b111_111_111],
                // all dark pixels -> is 0b000_000_000 -> swap to the pixel at that index
                Pixel::Dark => table[0],
            }
        }
    }

    image
        .data()
        .iter()
        .filter(|pixel| matches!(pixel, Pixel::Light))
        .count()
}

pub fn part1(mut reader: Input) -> anyhow::Result<usize> {
    let table = read_table(&mut reader);
    let image = Grid::read(&mut reader);

    Ok(enhance(image, &table, 2))
}

pub fn part2(mut reader: Input) -> anyhow::Result<usize> {
    let table = read_table(&mut reader);
    let image = Grid::read(&mut reader);

    Ok(enhance(image, &table, 50))
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
