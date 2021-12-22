use aoc2021::Input;
use serde_scan::scan;
use std::io::BufRead;

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
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
struct Cuboid {
    min: Vec3,
    max: Vec3,
}

impl Cuboid {
    fn new(a: Vec3, b: Vec3) -> Self {
        let mut result = Self { min: a, max: a };
        result.expand(b);
        result
    }

    fn expand(&mut self, point: Vec3) {
        self.min.x = point.x.min(self.min.x);
        self.min.y = point.y.min(self.min.y);
        self.min.z = point.z.min(self.min.z);
        self.max.x = point.x.max(self.max.x);
        self.max.y = point.y.max(self.max.y);
        self.max.z = point.z.max(self.max.z);
    }

    fn volume(&self) -> u64 {
        // (self.max.x - self.min.x + 1) as u64 * (self.max.y - self.min.y + 1) as u64 * (self.max.z - self.min.z + 1) as u64
        (self.max.x - self.min.x) as u64
            * (self.max.y - self.min.y) as u64
            * (self.max.z - self.min.z) as u64
    }
}

/// Removes `b` from `a` and yields the remaining cuboids
/// Divide and conquer, always split off one side and process the remaining cuboid
fn remove(a: &Cuboid, b: &Cuboid) -> Vec<Cuboid> {
    let no_x_overlap = a.min.x != b.min.x && (a.min.x >= b.max.x || a.max.x <= b.min.x);
    let no_y_overlap = a.min.y != b.min.y && (a.min.y >= b.max.y || a.max.y <= b.min.y);
    let no_z_overlap = a.min.z != b.min.z && (a.min.z >= b.max.z || a.max.z <= b.min.z);
    if no_x_overlap || no_y_overlap || no_z_overlap {
        return vec![*a];
    }

    let mut result = Vec::new();
    if a.min.x < b.min.x && a.max.x > b.min.x {
        let left_side = Cuboid::new(a.min, Vec3::new(b.min.x, a.max.y, a.max.z));
        let right_side = Cuboid::new(Vec3::new(b.min.x, a.min.y, a.min.z), a.max);
        //println!("A:\n{:?}\n{:?}\n{:?}", a, left_side, right_side);
        result.push(left_side);
        result.extend(remove(&right_side, b));
    } else if a.min.x < b.max.x && a.max.x > b.max.x {
        let left_side = Cuboid::new(a.min, Vec3::new(b.max.x, a.max.y, a.max.z));
        let right_side = Cuboid::new(Vec3::new(b.max.x, a.min.y, a.min.z), a.max);
        //println!("B:\n{:?}\n{:?}\n{:?}", a, left_side, right_side);
        result.push(right_side);
        result.extend(remove(&left_side, b));
    } else if a.min.y < b.min.y && a.max.y > b.min.y {
        let bottom_side = Cuboid::new(a.min, Vec3::new(a.max.x, b.min.y, a.max.z));
        let top_side = Cuboid::new(Vec3::new(a.min.x, b.min.y, a.min.z), a.max);
        //println!("C:\n{:?}\n{:?}\n{:?}\n{:?}", a, b, top_side, bottom_side);
        result.push(bottom_side);
        result.extend(remove(&top_side, b));
    } else if a.min.y < b.max.y && a.max.y > b.max.y {
        let top_side = Cuboid::new(Vec3::new(a.min.x, b.max.y, a.min.z), a.max);
        let bottom_side = Cuboid::new(a.min, Vec3::new(a.max.x, b.max.y, a.max.z));
        //println!("D:\n{:?}\n{:?}\n{:?}\n{:?}", a, b, top_side, bottom_side);
        result.push(top_side);
        result.extend(remove(&bottom_side, b));
    } else if a.min.z < b.min.z && a.max.z > b.min.z {
        let back_side = Cuboid::new(a.min, Vec3::new(a.max.x, a.max.y, b.min.z));
        let front_side = Cuboid::new(Vec3::new(a.min.x, a.min.y, b.min.z), a.max);
        //println!("E:\n{:?}\n{:?}\n{:?}\n{:?}", a, b, front_side, back_side);
        result.push(back_side);
        result.extend(remove(&front_side, b));
    } else if a.min.z < b.max.z && a.max.z > b.max.z {
        let back_side = Cuboid::new(a.min, Vec3::new(a.max.x, a.max.y, b.max.z));
        let front_side = Cuboid::new(Vec3::new(a.min.x, a.min.y, b.max.z), a.max);
        //println!("F:\n{:?}\n{:?}\n{:?}\n{:?}", a, b, back_side, front_side);
        result.push(front_side);
        result.extend(remove(&back_side, b));
    }
    result
}

fn parse(line: &str) -> (bool, Cuboid) {
    let (on_off, x1, x2, y1, y2, z1, z2): (String, i32, i32, i32, i32, i32, i32) =
        scan!("{} x={}..{},y={}..{},z={}..{}" <- line).unwrap();

    let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
    let (min_z, max_z) = if z1 < z2 { (z1, z2) } else { (z2, z1) };

    (
        on_off.starts_with("on"),
        Cuboid::new(
            Vec3::new(min_x, min_y, min_z),
            Vec3::new(max_x + 1, max_y + 1, max_z + 1),
        ),
    )
}

fn restart_reactor(input: impl Iterator<Item = (bool, Cuboid)>) -> u64 {
    let mut cubes = Vec::new();
    for (on_off, cuboid) in input {
        if on_off {
            let mut tmp_cubes = vec![cuboid];
            for cube in &cubes {
                tmp_cubes = tmp_cubes.iter().flat_map(|c| remove(c, cube)).collect();
            }
            cubes.extend(tmp_cubes);
        } else {
            cubes = cubes
                .into_iter()
                .flat_map(|c| remove(&c, &cuboid))
                .collect();
        }
    }

    cubes.iter().map(|x| x.volume()).sum::<u64>()
}

pub fn part1(reader: Input) -> anyhow::Result<u64> {
    let input = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse(&line))
        .filter(|(_, cuboid)| {
            !(cuboid.min.x < -50
                || cuboid.max.x > 51
                || cuboid.min.y < -50
                || cuboid.max.y > 51
                || cuboid.min.z < -50
                || cuboid.max.z > 51)
        });
    Ok(restart_reactor(input))
}

pub fn part2(reader: Input) -> anyhow::Result<u64> {
    let input = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse(&line));
    Ok(restart_reactor(input))
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(a: (i32, i32, i32), b: (i32, i32, i32)) -> Cuboid {
        Cuboid::new(Vec3::new(a.0, a.1, a.2), Vec3::new(b.0, b.1, b.2))
    }

    #[test]
    fn test_remove1() {
        // |AA
        // |ABB
        // ----
        let a = Cuboid::new(Vec3::new(0, 0, 0), Vec3::new(2, 2, 0));
        let b = Cuboid::new(Vec3::new(1, 0, 0), Vec3::new(3, 1, 0));

        let mut r = remove(&a, &b);
        r.sort();
        assert_eq!(r, vec![c((0, 0, 0), (1, 2, 0)), c((1, 1, 0), (2, 2, 0)),]);

        let mut r = remove(&b, &a);
        r.sort();
        assert_eq!(r, vec![c((2, 0, 0), (3, 1, 0)),]);
    }

    #[test]
    fn test_remove2() {
        // |AA
        // |ABB
        // |AA
        // ----
        let a = Cuboid::new(Vec3::new(0, 0, 0), Vec3::new(2, 3, 0));
        let b = Cuboid::new(Vec3::new(1, 1, 0), Vec3::new(3, 2, 0));

        let mut r = remove(&a, &b);
        r.sort();

        // |13
        // |1BB
        // |12
        // ----
        assert_eq!(
            r,
            vec![
                c((0, 0, 0), (1, 3, 0)),
                c((1, 0, 0), (2, 1, 0)),
                c((1, 2, 0), (2, 3, 0))
            ]
        );

        // |AA
        // |AB1
        // |AA
        // ----
        let mut r = remove(&b, &a);
        r.sort();
        assert_eq!(r, vec![c((2, 1, 0), (3, 2, 0)),]);
    }

    #[test]
    fn test_remove3() {
        // |AAA
        // |ABA
        // |AAA
        // ----
        let a = Cuboid::new(Vec3::new(0, 0, 0), Vec3::new(3, 3, 0));
        let b = Cuboid::new(Vec3::new(1, 1, 0), Vec3::new(2, 2, 0));

        let mut r = remove(&a, &b);
        r.sort();
        // |134
        // |1B4
        // |124
        // ----
        assert_eq!(
            r,
            vec![
                c((0, 0, 0), (1, 3, 0)),
                c((1, 0, 0), (2, 1, 0)),
                c((1, 2, 0), (2, 3, 0)),
                c((2, 0, 0), (3, 3, 0)),
            ]
        );

        let mut r = remove(&b, &a);
        r.sort();
        assert_eq!(r, vec![]);
    }

    #[test]
    fn test_remove4() {
        // |BBB
        // |AAA
        // ----
        let a = Cuboid::new(Vec3::new(0, 0, 0), Vec3::new(3, 1, 0));
        let b = Cuboid::new(Vec3::new(0, 1, 0), Vec3::new(3, 2, 0));

        let mut r = remove(&a, &b);
        r.sort();
        assert_eq!(r, vec![a]);

        let mut r = remove(&b, &a);
        r.sort();
        assert_eq!(r, vec![b]);
    }

    #[test]
    fn test_remove5() {
        // |AB
        // |AB
        // ----
        let a = Cuboid::new(Vec3::new(0, 0, 0), Vec3::new(1, 2, 0));
        let b = Cuboid::new(Vec3::new(1, 0, 0), Vec3::new(2, 2, 0));

        let mut r = remove(&a, &b);
        r.sort();
        assert_eq!(r, vec![a]);

        let mut r = remove(&b, &a);
        r.sort();
        assert_eq!(r, vec![b]);
    }

    #[test]
    fn test_remove6() {
        // |AAA
        // |BBB (A underneath)
        // ----
        let a = Cuboid::new(Vec3::new(0, 0, 0), Vec3::new(3, 2, 0));
        let b = Cuboid::new(Vec3::new(0, 0, 0), Vec3::new(3, 1, 0));

        let mut r = remove(&a, &b);
        r.sort();
        assert_eq!(r, vec![c((0, 1, 0), (3, 2, 0))]);

        let mut r = remove(&b, &a);
        r.sort();
        assert_eq!(r, vec![]);
    }

    #[test]
    fn test_remove1z() {
        // |AA
        // |ABB
        // ----
        let a = Cuboid::new(Vec3::new(0, 0, 0), Vec3::new(2, 0, 2));
        let b = Cuboid::new(Vec3::new(1, 0, 0), Vec3::new(3, 0, 1));

        let mut r = remove(&a, &b);
        r.sort();
        assert_eq!(r, vec![c((0, 0, 0), (1, 0, 2)), c((1, 0, 1), (2, 0, 2)),]);

        let mut r = remove(&b, &a);
        r.sort();
        assert_eq!(r, vec![c((2, 0, 0), (3, 0, 1)),]);
    }

    #[test]
    fn test_remove2z() {
        // |AA
        // |ABB
        // |AA
        // ----
        let a = Cuboid::new(Vec3::new(0, 0, 0), Vec3::new(2, 0, 3));
        let b = Cuboid::new(Vec3::new(1, 0, 1), Vec3::new(3, 0, 2));

        let mut r = remove(&a, &b);
        r.sort();

        // |13
        // |1BB
        // |12
        // ----
        assert_eq!(
            r,
            vec![
                c((0, 0, 0), (1, 0, 3)),
                c((1, 0, 0), (2, 0, 1)),
                c((1, 0, 2), (2, 0, 3))
            ]
        );

        // |AA
        // |AB1
        // |AA
        // ----
        let mut r = remove(&b, &a);
        r.sort();
        assert_eq!(r, vec![c((2, 0, 1), (3, 0, 2)),]);
    }
}
