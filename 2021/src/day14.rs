use aoc2021::Input;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;

struct PolyTemplate([u8; 26 * 26]);

impl PolyTemplate {
    fn new() -> Self {
        Self([0; 26 * 26])
    }

    fn insert(&mut self, (a, b): (u8, u8), result: u8) {
        self.0[(a - b'A') as usize + 26 * (b - b'A') as usize] = result;
    }

    fn lookup(&self, (a, b): (u8, u8)) -> u8 {
        self.0[(a - b'A') as usize + 26 * (b - b'A') as usize]
    }

    fn expand(&self, input: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(input.len() * 2 - 1);

        for (a, b) in input.iter().copied().tuple_windows() {
            result.push(a);
            result.push(self.lookup((a, b)));
        }
        result.push(input[input.len() - 1]);

        result
    }
}

impl Default for PolyTemplate {
    fn default() -> Self {
        Self::new()
    }
}

fn parse(input: &str) -> ((u8, u8), u8) {
    let input = input.as_bytes();
    ((input[0], input[1]), input[6])
}

fn expand(template: &PolyTemplate, input: &[u8], iterations: usize) -> usize {
    let mut result = [0; 26];
    let mut cache = HashMap::new();

    for (a, b) in input.iter().copied().tuple_windows() {
        result[(a - b'A') as usize] += 1;
        expand_inner(template, (a, b), iterations, &mut result, &mut cache);
    }
    result[(input[input.len() - 1] - b'A') as usize] += 1;

    let (min, max) = result
        .into_iter()
        .filter(|&x| x != 0)
        .minmax()
        .into_option()
        .unwrap();
    max - min
}

fn expand_inner(
    template: &PolyTemplate,
    (a, b): (u8, u8),
    iterations: usize,
    result: &mut [usize; 26],
    cache: &mut HashMap<(u8, u8, usize), [usize; 26]>,
) {
    if let Some(cached) = cache.get(&(a, b, iterations)) {
        *result = *cached;
        return;
    }
    if iterations == 0 {
        return;
    }
    let expanded = template.lookup((a, b));

    result[(expanded - b'A') as usize] += 1;

    let mut tmp1 = [0; 26];
    expand_inner(template, (a, expanded), iterations - 1, &mut tmp1, cache);
    cache.insert((a, expanded, iterations - 1), tmp1);

    let mut tmp2 = [0; 26];
    expand_inner(template, (expanded, b), iterations - 1, &mut tmp2, cache);
    cache.insert((expanded, b, iterations - 1), tmp2);

    for i in 0..26 {
        result[i] += tmp1[i] + tmp2[i];
    }
}

pub fn part1(reader: Input) -> anyhow::Result<usize> {
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut input: Vec<_> = lines.next().unwrap().bytes().collect();

    let mut template = PolyTemplate::new();
    for line in lines.skip(1) {
        let (left, right) = parse(&line);
        template.insert(left, right);
    }

    for _ in 0..10 {
        input = template.expand(&input);
    }
    let (min, max) = input
        .into_iter()
        .counts()
        .into_values()
        .minmax()
        .into_option()
        .unwrap();

    Ok(max - min)
}

pub fn part2(reader: Input) -> anyhow::Result<usize> {
    let mut lines = reader.lines().map(|line| line.unwrap());
    let input: Vec<_> = lines.next().unwrap().bytes().collect();

    let mut template = PolyTemplate::new();
    for line in lines.skip(1) {
        let (left, right) = parse(&line);
        template.insert(left, right);
    }

    Ok(expand(&template, &input, 40))
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
