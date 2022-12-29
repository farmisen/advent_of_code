use crate::{LineSplitting, ReturnOneLiners};
use itertools::Itertools;
use regex::Regex;
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(i64, i64);

impl From<(i64, i64)> for Pos {
    fn from(value: (i64, i64)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<&(i64, i64)> for Pos {
    fn from(value: &(i64, i64)) -> Self {
        Self(value.0, value.1)
    }
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    x: i64,
    y: i64,
    locking_distance: u64,
}

impl From<&str> for Sensor {
    fn from(value: &str) -> Self {
        let raw_re = r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)";

        let re = Regex::new(raw_re).unwrap();
        let caps = re.captures(value).unwrap();

        let x = caps["sx"].parse::<i64>().unwrap();
        let y = caps["sy"].parse::<i64>().unwrap();
        let bx = caps["bx"].parse::<i64>().unwrap();
        let by = caps["by"].parse::<i64>().unwrap();

        Self {
            x,
            y,
            locking_distance: x.abs_diff(bx) + y.abs_diff(by),
        }
    }
}

impl Sensor {
    fn in_range(&self, pos: &Pos) -> bool {
        let &Pos(x, y) = pos;
        self.x.abs_diff(x) + self.y.abs_diff(y) <= self.locking_distance
    }
}

impl HasPerimeter for Sensor {
    fn perimeter(&self, distance: u64) -> PerimeterIterator {
        PerimeterIterator {
            sensor: *self,
            idx: 0,
            zone: Zone::Right,
            distance,
        }
    }
}

pub trait HasPerimeter {
    fn perimeter(&self, distance: u64) -> PerimeterIterator;
}

enum Zone {
    Right,
    TopRight,
    Top,
    TopLeft,
    Left,
    BottomLeft,
    Bottom,
    BottomRight,
}

pub struct PerimeterIterator {
    sensor: Sensor,
    idx: i64,
    zone: Zone,
    distance: u64,
}

impl Iterator for PerimeterIterator {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let d = self.distance as i64;
        let idx = self.idx;
        let x = self.sensor.x;
        let y = self.sensor.y;

        match self.zone {
            Zone::Right => {
                self.idx = 1;
                self.zone = Zone::TopRight;
                Some(Pos(x + d, y))
            }
            Zone::TopRight => {
                if idx == d - 1 {
                    self.idx = 0;
                    self.zone = Zone::Top;
                } else {
                    self.idx += 1;
                }

                Some(Pos(x + d - idx, y + idx))
            }
            Zone::Top => {
                self.idx = 1;
                self.zone = Zone::TopLeft;
                Some(Pos(x, y + d))
            }
            Zone::TopLeft => {
                if idx == d - 1 {
                    self.idx = 0;
                    self.zone = Zone::Left;
                } else {
                    self.idx += 1;
                }
                Some(Pos(x - idx, y + d - idx))
            }
            Zone::Left => {
                self.idx = 1;
                self.zone = Zone::BottomLeft;
                Some(Pos(x - d, y))
            }
            Zone::BottomLeft => {
                if idx == d - 1 {
                    self.idx = 0;
                    self.zone = Zone::Bottom;
                } else {
                    self.idx += 1;
                }
                Some(Pos(x - d + idx, y - idx))
            }
            Zone::Bottom => {
                self.idx = 1;
                self.zone = Zone::BottomRight;
                Some(Pos(x, y - d))
            }
            Zone::BottomRight => {
                if idx == d {
                    None
                } else {
                    self.idx += 1;
                    Some(Pos(x + idx, y - d + idx))
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Range(i64, i64);

impl Range {
    fn overlaps(&self, other: &Self) -> bool {
        (self.0 <= other.0 && self.1 <= other.0) || (self.0 <= other.1 && self.1 >= other.1)
    }

    fn merge(&self, other: &Self) -> Self {
        Self(cmp::min(self.0, other.0), cmp::max(self.1, other.1))
    }

    fn len(&self) -> u64 {
        self.1.abs_diff(self.0)
    }
}

fn parse_report(report: &str) -> Vec<Sensor> {
    report.trimmed_lines().map(Sensor::from).collect()
}

pub fn part01(input: &str, row: i64) -> u64 {
    // parse input
    // for each sensor that can reach the col (i.e |s.y - row| <= s.locking_distance)
    // calculate the ranges covering the row:
    // s.x - s.locking_distance + |s.y - row|
    // s.x + s.locking_distance - |s.y - row|
    // sort them
    // merge them
    // size them
    // sum them

    parse_report(input)
        .iter()
        .filter(|s| s.y.abs_diff(row) <= s.locking_distance)
        .map(|s| {
            Range(
                s.x - s.locking_distance as i64 + (s.y - row).abs(),
                s.x + s.locking_distance as i64 - (s.y - row).abs(),
            )
        })
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .fold(vec![], |accu: Vec<Range>, range| match accu.split_last() {
            None => vec![range],
            Some((last, elements)) => match range.overlaps(last) || last.overlaps(&range) {
                false => elements.to_vec().push_and_ret(*last).push_and_ret(range),
                true => elements.to_vec().push_and_ret(last.merge(&range)),
            },
        })
        .iter()
        .map(|r| r.len())
        .sum::<u64>()
}

pub fn part02(input: &str, max_coord: i64) -> u64 {
    // parse input
    // check each sensor perimeter to find the only location that is out of range from every sensor

    let sensors = parse_report(input);

    sensors
        .iter()
        .flat_map(|sensor| sensor.perimeter(sensor.locking_distance + 1))
        .filter(|&Pos(x, y)| x >= 0 && x <= max_coord && y >= 0 && y <= max_coord)
        .filter(|pos| sensors.iter().all(|sensor| !sensor.in_range(pos)))
        .map(|Pos(x, y)| x * 4000000 + y)
        .next()
        .unwrap() as u64
}

#[cfg(test)]
mod tests {

    use crate::{
        day15::{part01, part02, HasPerimeter, Pos},
        input,
    };

    use super::Sensor;

    #[test]
    fn day_15_it_has_a_perimeter() {
        let sensor = Sensor {
            x: 0,
            y: 0,
            locking_distance: 3,
        };
        let expectation = [
            (3 + sensor.x, sensor.y),
            (2 + sensor.x, 1 + sensor.y),
            (1 + sensor.x, 2 + sensor.y),
            (sensor.x, 3 + sensor.y),
            (-1 + sensor.x, 2 + sensor.y),
            (-2 + sensor.x, 1 + sensor.y),
            (-3 + sensor.x, sensor.y),
            (-2 + sensor.x, -1 + sensor.y),
            (-1 + sensor.x, -2 + sensor.y),
            (sensor.x, -3 + sensor.y),
            (1 + sensor.x, -2 + sensor.y),
            (2 + sensor.x, -1 + sensor.y),
        ]
        .iter()
        .map(|(x, y)| Pos(*x, *y))
        .collect::<Vec<Pos>>();
        assert_eq!(sensor.perimeter(3).collect::<Vec<Pos>>(), expectation);
    }

    const REPORT: &str = "\
    Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn day_15_part_01_example() {
        assert_eq!(part01(REPORT, 10), 26);
    }

    #[test]
    fn day_15_part_01() {
        assert_eq!(part01(input("day_15").as_str(), 2000000), 5564017);
    }

    #[test]
    fn day_15_part_02_example() {
        assert_eq!(part02(REPORT, 20), 56000011);
    }

    #[test]
    fn day_15_part_02() {
        assert_eq!(part02(input("day_15").as_str(), 4000000), 11558423398893);
    }
}
