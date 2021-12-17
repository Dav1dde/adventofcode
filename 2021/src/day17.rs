use aoc2021::Input;
use serde_scan::scan;
use std::io::BufRead;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn add(&mut self, x: i64, y: i64) {
        self.x += x;
        self.y += y;
    }
}

#[derive(Debug)]
struct Area(Point, Point);

impl Area {
    fn contains(&self, point: Point) -> bool {
        point.x >= self.0.x && point.x <= self.1.x && point.y >= self.0.y && point.y <= self.1.y
    }

    fn is_too_far(&self, point: Point) -> bool {
        point.x > self.1.x
    }

    fn is_too_short(&self, point: Point) -> bool {
        point.x < self.0.x && point.y < self.1.y
    }

    fn is_above(&self, point: Point) -> bool {
        point.x >= self.0.x && point.x <= self.1.x && self.0.y < point.y
    }
}

fn shoot(area: Area) -> (i64, i64, (i64, i64)) {
    // there is so much to optimize even in the non equasion solution like this,
    // but I am just over it at this point
    let mut max_y = i64::MIN;
    let mut result = (0, 0);
    let mut total = 0;

    for x_vel_start in 0..(area.1.x * 2) {
        for y_vel_start in -1000..1000 {
            let mut position = Point::new(0, 0);
            let mut x_vel = x_vel_start;
            let mut y_vel = y_vel_start;
            let mut max_y_tmp = i64::MIN;
            for _ in 1.. {
                position.add(x_vel, y_vel);
                x_vel = (x_vel - 1).max(0); // TODO towards 0, use sign
                y_vel -= 1;
                max_y_tmp = max_y_tmp.max(position.y);

                if area.contains(position) {
                    if max_y_tmp > max_y {
                        max_y = max_y_tmp;
                        result = (x_vel_start, y_vel_start);
                    }
                    total += 1;
                    break;
                }
                if x_vel == 0 && area.is_too_short(position) {
                    break;
                }
                if x_vel == 0 && !area.is_above(position) {
                    break;
                }
                if area.is_too_far(position) {
                    break;
                }
            }
        }
    }

    (max_y, total, result)
}

fn parse(reader: Input) -> Area {
    let input = reader.lines().next().unwrap().unwrap();
    let s = input.as_str();
    let (x1, x2, y1, y2) = scan!("target area: x={}..{}, y={}..{}" <- s).unwrap();
    Area(Point::new(x1, y1), Point::new(x2, y2))
}

pub fn part1(reader: Input) -> anyhow::Result<i64> {
    let result = shoot(parse(reader));
    Ok(result.0)
}

pub fn part2(reader: Input) -> anyhow::Result<i64> {
    let result = shoot(parse(reader));
    Ok(result.1)
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_contains() {
        let area = Area(Point::new(20, -10), Point::new(30, -5));
        assert!(area.contains(area.0));
        assert!(area.contains(area.1));
        assert!(area.contains(Point::new(20, -5)));
        assert!(area.contains(Point::new(30, -10)));
        assert!(area.contains(Point::new(30, -7)));
        assert!(area.contains(Point::new(25, -5)));
    }
}
