#![allow(dead_code)]
use aoc2021::Input;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug, Copy, Clone)]
enum Direction {
    D00,
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
}

impl Direction {
    fn apply(&self, v: Vec3) -> Vec3 {
        match self {
            // Self::D00 => Vec3::new(v.x, v.y, v.z),
            // Self::D01 => Vec3::new(-v.x, v.y, v.z),
            // Self::D02 => Vec3::new(v.x, -v.y, v.z),
            // Self::D03 => Vec3::new(v.x, v.y, -v.z),

            // Self::D04 => Vec3::new(v.x, v.z, v.y),
            // Self::D05 => Vec3::new(-v.x, v.z, v.y),
            // Self::D06 => Vec3::new(v.x, -v.z, v.y),
            // Self::D07 => Vec3::new(v.x, v.z, -v.y),

            // Self::D08 => Vec3::new(v.y, v.x, v.z),
            // Self::D09 => Vec3::new(-v.y, v.x, v.z),
            // Self::D10 => Vec3::new(v.y, -v.x, v.z),
            // Self::D11 => Vec3::new(v.y, v.x, -v.z),

            // Self::D12 => Vec3::new(v.y, v.z, v.x),
            // Self::D13 => Vec3::new(-v.y, v.z, v.x),
            // Self::D14 => Vec3::new(v.y, -v.z, v.x),
            // Self::D15 => Vec3::new(v.y, v.z, -v.x),

            // Self::D16 => Vec3::new(v.z, v.y, v.x),
            // Self::D17 => Vec3::new(-v.z, v.y, v.x),
            // Self::D18 => Vec3::new(v.z, -v.y, v.x),
            // Self::D19 => Vec3::new(v.z, v.y, -v.x),

            // Self::D20 => Vec3::new(v.z, v.x, v.y),
            // Self::D21 => Vec3::new(-v.z, v.x, v.y),
            // Self::D22 => Vec3::new(v.z, -v.x, v.y),
            // Self::D23 => Vec3::new(v.z, v.x, -v.y),
        Self::D00 => Vec3::new(v.x, v.y, v.z),     //[x, y, z]
        Self::D01 => Vec3::new(v.x, v.z, -v.y),    //[x, z, -y],
        Self::D02 => Vec3::new(v.x, -v.y, -v.z),   //[x, -y, -z],
        Self::D03 => Vec3::new(v.x, -v.z, v.y),    //[x, -z, y],
        Self::D04 => Vec3::new(v.y, v.x, -v.z),    //[y, x, -z],
        Self::D05 => Vec3::new(v.y, v.z, v.x),     //[y, z, x],
        Self::D06 => Vec3::new(v.y, -v.x, v.z),    //[y, -x, z],
        Self::D07 => Vec3::new(v.y, -v.z, -v.x),   //[y, -z, -x],
        Self::D08 => Vec3::new(v.z, v.x, v.y),     //[z, x, y],
        Self::D09 => Vec3::new(v.z, v.y, -v.x),    //[z, y, -x],
        Self::D10 => Vec3::new(v.z, -v.x, -v.y),  //[z, -x, -y],
        Self::D11 => Vec3::new(v.z, -v.y, v.x),   //[z, -y, x],
        Self::D12 => Vec3::new(-v.x, v.y, -v.z),  //[-x, y, -z],
        Self::D13 => Vec3::new(-v.x, v.z, v.y),   //[-x, z, y],
        Self::D14 => Vec3::new(-v.x, -v.y, v.z),  //[-x, -y, z],
        Self::D15 => Vec3::new(-v.x, -v.z, -v.y), //[-x, -z, -y],
        Self::D16 => Vec3::new(-v.y, v.x, v.z),   //[-y, x, z],
        Self::D17 => Vec3::new(-v.y, v.z, -v.x),  //[-y, z, -x],
        Self::D18 => Vec3::new(-v.y, -v.x, -v.z), //[-y, -x, -z],
        Self::D19 => Vec3::new(-v.y, -v.z, v.x),  //[-y, -z, x],
        Self::D20 => Vec3::new(-v.z, v.x, -v.y),  //[-z, x, -y],
        Self::D21 => Vec3::new(-v.z, v.y, v.x),   //[-z, y, x],
        Self::D22 => Vec3::new(-v.z, -v.x, v.y),  //[-z, -x, y],
        Self::D23 => Vec3::new(-v.z, -v.y, -v.x), //[-z, -y, -x],

        }
    }

    fn reverse(&self) -> Self {
        match self {
        Self::D00 => Self::D00,     //[x, y, z]
        Self::D01 => Self::D03, // Vec3::new(v.x, v.z, -v.y),    //[x, z, -y],
        Self::D02 => Self::D02, // Vec3::new(v.x, -v.y, -v.z),   //[x, -y, -z],
        Self::D03 => Self::D01, // Vec3::new(v.x, -v.z, v.y),    //[x, -z, y],
        Self::D04 => Self::D04, // Vec3::new(v.y, v.x, -v.z),    //[y, x, -z],
        Self::D05 => Self::D08, // Vec3::new(v.y, v.z, v.x),     //[y, z, x],
        Self::D06 => Self::D16, // Vec3::new(v.y, -v.x, v.z),    //[y, -x, z],
        Self::D07 => Self::D20, // Vec3::new(v.y, -v.z, -v.x),   //[y, -z, -x],
        Self::D08 => Self::D05, // Vec3::new(v.z, v.x, v.y),     //[z, x, y],
        Self::D09 => Self::D21, // Vec3::new(v.z, v.y, -v.x),    //[z, y, -x],
        Self::D10 => Self::D19, // Vec3::new(v.z, -v.x, -v.y),  //[z, -x, -y],
        Self::D11 => Self::D11, // Vec3::new(v.z, -v.y, v.x),   //[z, -y, x],
        Self::D12 => Self::D12, // Vec3::new(-v.x, v.y, -v.z),  //[-x, y, -z],
        Self::D13 => Self::D13, // Vec3::new(-v.x, v.z, v.y),   //[-x, z, y],
        Self::D14 => Self::D14, // Vec3::new(-v.x, -v.y, v.z),  //[-x, -y, z],
        Self::D15 => Self::D15, // Vec3::new(-v.x, -v.z, -v.y), //[-x, -z, -y],
        Self::D16 => Self::D06, // Vec3::new(-v.y, v.x, v.z),   //[-y, x, z],
        Self::D17 => Self::D22, // Vec3::new(-v.y, v.z, -v.x),  //[-y, z, -x],
        Self::D18 => Self::D18, // Vec3::new(-v.y, -v.x, -v.z), //[-y, -x, -z],
        Self::D19 => Self::D10, // Vec3::new(-v.y, -v.z, v.x),  //[-y, -z, x],
        Self::D20 => Self::D07, // Vec3::new(-v.z, v.x, -v.y),  //[-z, x, -y],
        Self::D21 => Self::D09, // Vec3::new(-v.z, v.y, v.x),   //[-z, y, x],
        Self::D22 => Self::D17, // Vec3::new(-v.z, -v.x, v.y),  //[-z, -x, y],
        Self::D23 => Self::D23, // Vec3::new(-v.z, -v.y, -v.x), //[-z, -y, -x],
        }
    }

    fn all() -> &'static [Direction; 24] {
        static DIRECTIONS: [Direction; 24] = [
            Direction::D00,
            Direction::D01,
            Direction::D02,
            Direction::D03,
            Direction::D04,
            Direction::D05,
            Direction::D06,
            Direction::D07,
            Direction::D08,
            Direction::D09,
            Direction::D10,
            Direction::D11,
            Direction::D12,
            Direction::D13,
            Direction::D14,
            Direction::D15,
            Direction::D16,
            Direction::D17,
            Direction::D18,
            Direction::D19,
            Direction::D20,
            Direction::D21,
            Direction::D22,
            Direction::D23,
        ];
        &DIRECTIONS
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn magnitude(&self) -> f64 {
        (((self.x as u64).pow(2) + (self.y as u64).pow(2) + (self.z as u64).pow(2)) as f64).sqrt()
    }

    fn sq(&self) -> Vec3 {
        Vec3 { x: self.x.pow(2), y: self.y.pow(2), z: self.z.pow(2) }
    }

    fn manhattan(&self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32 + self.z.abs() as u32
    }

    fn abs(&self) -> Self {
        Vec3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    fn sorted(&self) -> Self {
        let mut x = self.x;
        let mut y = self.x;
        let mut z = self.x;

        if x > z {
            std::mem::swap(&mut x, &mut z);
        }
        if x > y {
            std::mem::swap(&mut x, &mut y);
        }
        if y > z {
            std::mem::swap(&mut y, &mut z);
        }

        Vec3 { x, y, z }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    name: String,
    measurements: Vec<Vec3>,
}

impl Scanner {
    fn read(mut input: impl Iterator<Item = String>) -> Option<Self> {
        let line = input.next()?;
        let name = line.trim_matches('-').trim().to_string();

        let measurements = input
            .take_while(|line| !line.is_empty())
            .map(|line| {
                line.splitn(3, ',')
                    .map(|c| c.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .map(|(x, y, z)| Vec3 { x, y, z })
            .collect();

        Some(Self { name, measurements })
    }

    fn relative_differences(&self) -> Vec<(Vec3, Vec3, Vec3)> {
        self.measurements
            .iter()
            .tuple_combinations()
            //.map(|(&a, &b)| (a, b, (a - b).abs()))
            .map(|(&a, &b)| (a, b, a - b))
            .collect::<Vec<_>>()
    }
}

pub fn part1(reader: Input) -> anyhow::Result<u32> {
    let mut lines = reader.lines().map(|line| line.unwrap());

    let mut scanners = std::iter::repeat(0)
        .map_while(|_| Scanner::read(&mut lines))
        // .take(2)
        .collect::<Vec<_>>();
    
    // scanners.reverse();

    // let (a, b) = (&r[0], &r[1]);
    // dbg!(a.relative_differences());
    // dbg!(b.relative_differences());

    let x = scanners.clone();
    let x = x
        .iter()
        .map(|a| (&a.name, a.relative_differences()))
        .collect_vec();

    // let mut all_matches = HashMap::new();
    let mut origins = HashMap::new();

    // {
    //     let diff = Vec3::new(1125, -168, 72);
    //     let s2 = Vec3::new(682, -795, 504);
    //     let s4 = Vec3::new(-627, -443, -432);
    //     let dir = Direction::D04;

    //     assert_eq!(dir.apply(s4) + diff, s2);
    //     assert_eq!(dir.reverse().apply(s2 - diff), s4);
    // }

    //  (1125, -168, 72) 
    for ((a_name, a), (b_name, b)) in x.into_iter().tuple_combinations() {
        // let duplicates = a.iter().filter(|v| b.contains(v)).count();
        // println!("{} <-> {}: {}", a_name, b_name, duplicates);

        let mut matches = HashMap::<Vec3, Vec3>::new();

        let mut dir = None;
        for direction in Direction::all() {
            // if !matches!(direction, Direction::D00) {
            // continue;
            // }
            //println!("{} <-> {}", a_name, b_name);
            for (aa, ab, adiff) in &a {
                for (ba, bb, bdiff) in &b {
                    if *adiff == direction.apply(*bdiff) {
                        //println!("{} {} {} {}", aa, ab, direction.apply(*ba), direction.apply(*bb));
                        //println!("{} {}", aa < ab, direction.apply(*ba) < direction.apply(*bb));
                        //println!("{} {} {} {}", aa.magnitude(), ab.magnitude(), ba.magnitude(), bb.magnitude());
                        //println!("({} {}) ({} {})", aa, ab, direction.apply(ba), direction.apply(bb));
                        // println!("{} {}", ab, ba);
                        // matches.insert(*aa, direction.apply(*ba));
                        // matches.insert(*ab, direction.apply(*bb));

                        // println!("{} {} {}", adiff, ab, direction.apply(ba));
                        // println!("{} {} {}", bdiff, aa, direction.apply(bb));
                        // println!("{}", *aa - *ab);
                        // println!("{} {}", adiff, bdiff);
                        // println!("{} {}", aa, ab);
                        // println!("{} {}", ba, bb);
                        // println!("{}", *aa - direction.apply(*ba));
                        // println!("{}", *ab - direction.apply(*bb));
                        let ba = direction.apply(*ba);
                        let bb = direction.apply(*bb);

                        if aa < ab {
                            if ba < bb {
                                //println!("a {} {}", aa, ba);
                                //println!("a {} {}", ab, bb);
                                matches.insert(*aa, ba);
                                matches.insert(*ab, bb);
                            } else {
                                //println!("b {} {}", aa, bb);
                                //println!("b {} {}", ab, ba);
                                matches.insert(*aa, bb);
                                matches.insert(*ab, ba);
                            }
                        } else {
                            if ba < bb {
                                //println!("c {} {}", aa, bb);
                                //println!("c {} {}", ab, ba);
                                matches.insert(*aa, bb);
                                matches.insert(*ab, ba);
                            } else {
                                //println!("d {} {} {}", aa, ba, *aa - ba);
                                //println!("d {} {} {}", ab, bb, *ab - bb);
                                matches.insert(*aa, ba);
                                matches.insert(*ab, bb);
                            }
                        }
                        // dbg!(diff);
                        //println!("{}", diff);

                        // matches.insert(*ab, direction.apply(*ba));
                        // matches.insert(*aa, direction.apply(*bb));

                        // matches.insert(*aa, direction.apply(*ba));
                        // matches.insert(*ab, direction.apply(*bb));

                        // matches.insert(*aa, *ba);
                        // matches.insert(*ab, *bb);

                        // matches.insert(*aa, direction.apply(*bb));
                        // matches.insert(*ab, direction.apply(*ba));

                        // matches.insert(*aa, direction.apply(*ba));
                        // matches.insert(*ab, direction.apply(*bb));

                        // matches.insert(*ab, *ba);
                        // matches.insert(*aa, *bb);
                        //println!();
                    }
                }
            }
            //println!("==> {}", i);

            // dbg!(matches.len(), &matches);
            // if i >= 66 {
            if matches.len() >= 10 {
            //println!("{} <-> {}", a_name, b_name);
                dir = Some(direction);
                break;
            } else {
                matches.clear();
            }
        }

        if matches.len() >= 10 {
            let direction = dir.unwrap();
            // println!("{} <-> {} ({})", a_name, b_name, matches.len());
            // println!("{:#?}", matches);
            // for (k, v) in matches.iter() {
            //     println!("{} | {}", k, v);
            //     // println!("Vec3::new({})", v);
            // }
            // println!("{:?}", matches.len());
            let (k, v) = matches.iter().next().unwrap();
            // let diff = *k + *v;
            let diff = *k - *v;
            // println!("===> {} {}", diff2, diff);
            // let diff = *k - *v;
            // println!("{}|{} -> {}", k, v, diff);

            origins.insert((a_name.as_str(), b_name.as_str()), (*direction, diff));
            // origins.insert((a_name.as_str(), b_name.as_str()), diff);
            // differences.insert((b_name, a_name), direction.apply(diff));

            // all_matches.insert((a_name, b_name), (direction, diff));
            // all_matches
            //     .entry(a_name.as_str())
            //     .or_insert(Vec::new())
            //     .push((b_name, direction, diff));
        }

        // println!("{} ({:?}", a_name, a);
        // println!("{} ({:?}", b_name, b);
    }

    // dbg!(&origins);

    //dbg!(transform(Vec3::new(686,422,578), "scanner 1", "scanner 0", &origins));
    // dbg!(transform(Vec3::new(-211,-452,876), "scanner 4", "scanner 1", &origins));
    // dbg!(transform(Vec3::new(-364,-763,-893), "scanner 1", "scanner 0", &origins));
    //

    let start = scanners.remove(0);
    let transitions = build_transitions(&start.name, &origins);
    // let mut f = vec![
    //     Vec3::new(-258, -428, 682),
    //     Vec3::new(-575, 615, 604),
    //     Vec3::new(-293, -554, 779),
    //     Vec3::new(-680, 325, -822),
    //     Vec3::new(-743, 427, -804),
    //     Vec3::new(-211, -452, 876),
    //     Vec3::new(-393, 719, 612),
    //     Vec3::new(-485, 667, 467),
    //     Vec3::new(-714, 465, -776),
    //     Vec3::new(-627, -443, -432),
    //     Vec3::new(-660, -479, -426),
    //     Vec3::new(-652, -548, -490),
    //     Vec3::new(-211, -452, 876),
    // ];
    // transform2(
    //     &mut f,
    //     "scanner 4",
    //     "scanner 0",
    //     &transitions
    // );
    //dbg!(f);

    // let mut f = vec![
    //     Vec3::new(-258, -428, 682),
    // ];
    //transform2(
    //    &mut f,
    //    "scanner 2",
    //    "scanner 0",
    //    &transitions
    //);

    let mut system = HashSet::new();
    start.measurements.iter().for_each(|m| { system.insert(*m); });

    for scanner in scanners.into_iter() {
        let mut f = scanner.measurements;
        transform2(
            &mut f,
            &scanner.name,
            start.name.as_ref(),
            &transitions
        );

        f.iter().for_each(|m| { system.insert(*m); });
    }

    // for p in system.iter() {
    //     println!("{},{},{}", p.x, p.y, p.z);
    // }

    dbg!(system.len());

    let mut coords = HashMap::new();
    for path in transitions.iter() {
        for step in path {
            if !coords.contains_key(&step.0) {
                let mut f = vec![Vec3::new(0, 0, 0)];
                // dbg!(&start.name, step.0, &f);
                transform2(&mut f, step.0, &start.name, &transitions);
                // dbg!(&f);
                coords.insert(step.0, f[0]);
            }
        }
    }
    // dbg!(&coords);


    let s = coords.values().tuple_combinations().map(|(&a, &b)| (a - b).manhattan()).max().unwrap();
    dbg!(s);
    

    Ok(0)
}

fn transform2(
    points: &mut Vec<Vec3>,
    from: &str,
    to: &str,
    transitions: &Vec<Vec<(&str, Operation)>>,
) {
    let transition = transitions
        .iter()
        // TODO: check that to is before from
        .find(|transition| transition.iter().any(|(name, _)| name == &from))
        .unwrap();

    let r = transition
        .iter()
        .rev()
        .skip_while(|(name, _)| name != &from)
        .take_while(|(name, _)| name != &to);

    for &(_, op) in r {
        for point in points.iter_mut() {
            *point = op.apply(*point);
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Normal(Direction, Vec3),
    Reverse(Direction, Vec3),
}

impl Operation {
    fn diff(&self) -> Vec3 {
        match self {
            Self::Normal(_, direction) => *direction,
            Self::Reverse(_, direction) => *direction,
        }
    }
    fn apply(&self, target: Vec3) -> Vec3 {
        match self {
            Self::Normal(direction, diff) => {
                direction.apply(target) + *diff
            },
            Self::Reverse(direction, diff) => {
                direction.reverse().apply(target - *diff)
            }
        }

    }
}

impl Default for Operation {
    fn default() -> Self {
        Self::Normal(Direction::D00, Vec3::new(0, 0, 0))
    }
}

fn build_transitions<'a>(
    start: &'a str,
    origins: &HashMap<(&'a str, &'a str), (Direction, Vec3)>,
) -> Vec<Vec<(&'a str, Operation)>> {
    let mut seen = HashSet::new();
    seen.insert(start);
    let mut result = Vec::new();

    let mut next_values = vec![(start, vec![(start, Operation::default())])];
    while let Some((next, previous)) = next_values.pop() {
        let mut more = false;
        for (&(from, to), &(direction, origin)) in origins.iter() {
            if from == next {
                if seen.insert(to) {
                    more = true;
                    let mut previous = previous.clone();
                    previous.push((to, Operation::Normal(direction, origin)));
                    next_values.push((to, previous));
                }
            }
            if to == next {
                if seen.insert(from) {
                    more = true;
                    // TODO reverse direction here?
                    let mut previous = previous.clone();
                    previous.push((from, Operation::Reverse(direction, origin)));
                    next_values.push((from, previous));
                }
            }
        }

        if !more {
            result.push(previous);
        }
    }

    result
}

pub fn part2(_reader: Input) -> anyhow::Result<u32> {
    Ok(0)
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directions() {
        let start = Vec3::new(1, -2, 3);

        // for direction in Direction::all() {
        //     assert_eq!(
        //         start,
        //         direction.apply_reverse(direction.apply(start)),
        //         "{:?}",
        //         direction
        //     );
        // }

        for direction in Direction::all() {
            assert_eq!(
                start,
                direction.reverse().apply(direction.apply(start)),
                "{:?}",
                direction
            );
            //for reverse in Direction::all() {
            //    if reverse.apply(direction.apply(start)) == start {
            //        println!("{:?} <-> {:?}", direction, reverse);
            //        break;
            //    }
            //}
        }
    }
}
