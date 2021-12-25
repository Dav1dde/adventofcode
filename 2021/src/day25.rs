use aoc2021::grid::Grid;
use aoc2021::Input;

#[derive(Debug, Copy, Clone)]
enum Seafloor {
    Empty,
    CucumberEast,
    CucumberSouth,
}

impl Seafloor {
    fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    fn is_east(&self) -> bool {
        matches!(self, Self::CucumberEast)
    }

    fn is_south(&self) -> bool {
        matches!(self, Self::CucumberSouth)
    }
}

impl std::fmt::Display for Seafloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::CucumberEast => write!(f, ">"),
            Self::CucumberSouth => write!(f, "v"),
        }
    }
}

impl From<u8> for Seafloor {
    fn from(val: u8) -> Self {
        match val {
            b'>' => Self::CucumberEast,
            b'v' => Self::CucumberSouth,
            b'.' => Self::Empty,
            _ => panic!("invalid input"),
        }
    }
}

fn step(seafloor: &mut Grid<Seafloor>) -> bool {
    let mut moved = false;

    for y in 0..seafloor.height() {
        let first = (0, y);
        let last = (seafloor.width() - 1, y);
        let first_is_empty = seafloor[first].is_empty();
        let last_is_east = seafloor[last].is_east();

        let mut just_moved = false;
        for x in 0..seafloor.width() - 1 {
            if just_moved {
                just_moved = false;
                continue;
            }

            let next = (x + 1, y);
            if seafloor[(x, y)].is_east() && seafloor[next].is_empty() {
                seafloor.swap((x, y), next);
                just_moved = true;
                moved = true;
            }
        }

        if first_is_empty && last_is_east {
            seafloor.swap(first, last);
            moved = true;
        }
    }

    for x in 0..seafloor.width() {
        let first = (x, 0);
        let last = (x, seafloor.height() - 1);
        let first_is_empty = seafloor[first].is_empty();
        let last_is_south = seafloor[last].is_south();

        let mut just_moved = false;
        for y in 0..seafloor.height() - 1 {
            if just_moved {
                just_moved = false;
                continue;
            }

            let next = (x, y + 1);
            if seafloor[(x, y)].is_south() && seafloor[next].is_empty() {
                seafloor.swap((x, y), next);
                just_moved = true;
                moved = true;
            }
        }

        if first_is_empty && last_is_south {
            seafloor.swap(first, last);
            moved = true;
        }
    }

    moved
}

pub fn part1(mut reader: Input) -> anyhow::Result<usize> {
    let mut seafloor = Grid::<Seafloor>::read(&mut reader);
    for i in 1.. {
        if !step(&mut seafloor) {
            return Ok(i);
        }
    }
    unreachable!()
}

pub fn part2(_reader: Input) -> anyhow::Result<usize> {
    Ok(0)
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
