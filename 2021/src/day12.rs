use aoc2021::Input;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

#[derive(Debug)]
struct Cave {
    big: bool,
    connected_to: HashSet<String>,
}

impl Cave {
    fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let big = name.chars().all(|c| c.is_uppercase());
        Self {
            big,
            connected_to: HashSet::new(),
        }
    }

    fn connect_with(&mut self, name: impl Into<String>) {
        self.connected_to.insert(name.into());
    }

    fn connected_to(&self) -> &HashSet<String> {
        &self.connected_to
    }

    fn is_big(&self) -> bool {
        self.big
    }

    fn is_small(&self) -> bool {
        !self.is_big()
    }
}

fn parse_caves(reader: Input) -> HashMap<String, Cave> {
    let mut result = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let (name, connection) = line.split_once("-").unwrap();
        let c = result
            .entry(name.into())
            .or_insert_with_key(|key| Cave::new(key));
        c.connect_with(connection);
        let c = result
            .entry(connection.into())
            .or_insert_with_key(|key| Cave::new(key));
        c.connect_with(name);
    }
    result
}

fn visit1<'a>(
    cave: &'a Cave,
    caves: &'a HashMap<String, Cave>,
    visited: &mut HashSet<&'a str>,
) -> usize {
    let mut result = 0;
    for path in cave.connected_to() {
        if path == "end" {
            result += 1;
            continue;
        }
        if path == "start" {
            continue;
        }

        if visited.contains(path.as_str()) {
            continue;
        }

        let connected_cave = caves.get(path).unwrap();

        let mut visited = visited.clone();
        if connected_cave.is_small() {
            visited.insert(path);
        }

        result += visit1(connected_cave, caves, &mut visited);
    }
    result
}

fn visit2<'a>(
    cave: &'a Cave,
    caves: &'a HashMap<String, Cave>,
    visited: &mut HashSet<&'a str>,
    double_visit: Option<&'a str>,
) -> usize {
    let mut result = 0;
    for path in cave.connected_to() {
        if path == "end" {
            result += 1;
            continue;
        }
        if path == "start" {
            continue;
        }

        let double_visit = if visited.contains(path.as_str()) {
            if double_visit.is_some() {
                continue;
            } else {
                Some(path.as_str())
            }
        } else {
            double_visit
        };

        let connected_cave = caves.get(path).unwrap();

        let mut visited = visited.clone();
        if connected_cave.is_small() {
            visited.insert(path);
        }

        result += visit2(connected_cave, caves, &mut visited, double_visit);
    }
    result
}

pub fn part1(reader: Input) -> anyhow::Result<usize> {
    let mut caves = parse_caves(reader);
    let start = caves.remove("start").unwrap();
    Ok(visit1(&start, &caves, &mut HashSet::new()))
}

pub fn part2(reader: Input) -> anyhow::Result<usize> {
    let mut caves = parse_caves(reader);
    let start = caves.remove("start").unwrap();
    Ok(visit2(&start, &caves, &mut HashSet::new(), None))
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
