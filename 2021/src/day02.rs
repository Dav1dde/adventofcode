use aoc2021::Input;
use std::io::BufRead;

#[derive(Copy, Clone, Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl std::str::FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, amount) = match s.trim().split_once(' ') {
            Some(x) => x,
            None => anyhow::bail!("cannot parse command: {:?}", s),
        };

        let amount = amount.parse::<u32>()?;
        let command = match cmd {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            err => anyhow::bail!("invalid command {:?}", err),
        };

        Ok(command)
    }
}

#[derive(Default, Debug, Copy, Clone)]
struct State {
    forward: u32,
    down: u32,
    aim: u32,
}

impl State {
    fn apply_part1(mut self, cmd: Command) -> Self {
        match cmd {
            Command::Forward(x) => self.forward += x,
            Command::Down(x) => self.down += x,
            Command::Up(x) => self.down -= x,
        }
        self
    }

    fn apply_part2(mut self, cmd: Command) -> Self {
        match cmd {
            Command::Forward(x) => {
                self.forward += x;
                self.down += x * self.aim;
            }
            Command::Down(x) => self.aim += x,
            Command::Up(x) => self.aim -= x,
        }
        self
    }
}

pub fn part1(reader: Input) -> anyhow::Result<u32> {
    let State { forward, down, .. } = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .fold(State::default(), |state, cmd| state.apply_part1(cmd));

    Ok(forward * down)
}

pub fn part2(reader: Input) -> anyhow::Result<u32> {
    let State { forward, down, .. } = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .fold(State::default(), |state, cmd| state.apply_part2(cmd));

    Ok(forward * down)
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
