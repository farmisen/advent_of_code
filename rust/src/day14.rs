use crate::LINE_END;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Vec2(i64, i64);

impl Vec2 {
    pub fn path_to(&self, other: &Vec2) -> Vec<Self> {
        let &Vec2(x0, y0) = self;
        let Vec2(x1, y1) = other;
        let mut path = vec![];
        let (dx, dy) = ((x1 - x0).signum(), (y1 - y0).signum());
        let mut p = *self;
        loop {
            p = Vec2(p.0 + dx, p.1 + dy);
            path.push(p);
            if p.eq(other) {
                break;
            }
        }
        path
    }
}

impl From<&str> for Vec2 {
    fn from(value: &str) -> Self {
        let digits: Vec<i64> = value
            .splitn(2, ',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();
        Self(digits[0], digits[1])
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Unit {
    Rock,
    Sand,
}

pub fn parse_path(input: &str) -> Vec<Vec2> {
    input
        .split(" -> ")
        .map(Vec2::from)
        .fold(Vec::<Vec2>::new(), |mut path, vec2| {
            match path.iter().last() {
                None => vec![vec2],
                Some(p) => {
                    let mut pt = p.path_to(&vec2);
                    path.append(&mut pt);
                    path
                }
            }
        })
}

pub fn parse_scan(input: &str) -> HashMap<Vec2, Unit> {
    input
        .split(LINE_END)
        .filter(|l| !l.is_empty())
        .flat_map(parse_path)
        .map(|pos| (pos, Unit::Rock))
        .collect::<HashMap<_, _>>()
}

pub fn simulate_step<F>(units: &mut HashMap<Vec2, Unit>, bottom: i64, criteria: &F) -> bool
where
    F: Fn(&Vec2, i64, &HashMap<Vec2, Unit>) -> bool,
{
    let mut pos = Vec2(500, 0);

    let get_unit_at = |pos: &Vec2| -> Option<&Unit> {
        match units.get(pos) {
            Some(u) => Some(u),
            None => match pos.1 {
                y if y == bottom + 2 => Some(&Unit::Rock),
                _ => None,
            },
        }
    };

    loop {
        if criteria(&pos, bottom, units) {
            return true;
        }

        let Vec2(x, y) = pos;
        if let Some(p) = match get_unit_at(&Vec2(x, y + 1)) {
            None => Some(Vec2(x, y + 1)),
            Some(_) => match get_unit_at(&Vec2(x - 1, y + 1)) {
                None => Some(Vec2(x - 1, y + 1)),
                Some(_) => match get_unit_at(&Vec2(x + 1, y + 1)) {
                    None => Some(Vec2(x + 1, y + 1)),
                    Some(_) => None,
                },
            },
        } {
            pos = p;
        } else {
            units.insert(pos, Unit::Sand);
            break;
        }
    }
    false
}


pub fn simulate<F>(input: &str, criteria: F) -> u64 where
F: Fn(&Vec2, i64, &HashMap<Vec2, Unit>) -> bool, {
    // parse the input into a HashMap<Pos, Unit> of units
    let mut units = parse_scan(input);

    // figure out the rocks bottom y
    let bottom = units.keys().map(|p| p.1).sorted().last().unwrap();

    let mut step = 0;
    // iterate until a unit of sand fall out the boundary
    loop {
        if simulate_step(&mut units, bottom, &criteria) {
            break;
        }
        step += 1;
    }
    step
}


pub fn part01(input: &str) -> u64 {
    simulate(input, |pos, bottom, _| pos.1 == bottom)
}

pub fn part02(input: &str) -> u64 {
    simulate(input, |_, _, units| units.get(&Vec2(500, 0)).is_some())
}

#[cfg(test)]
mod tests {
    use crate::{
        day14::{parse_path, part01, part02, Vec2},
        input,
    };

    const PATH: &str = "498,4 -> 498,6 -> 496,6";

    #[test]
    fn day_14_parse_path() {
        assert_eq!(
            parse_path(PATH),
            vec![
                Vec2(498, 4),
                Vec2(498, 5),
                Vec2(498, 6),
                Vec2(497, 6),
                Vec2(496, 6)
            ]
        );
    }

    const SCAN: &str = "\
    498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn day_14_part_01_example() {
        assert_eq!(part01(SCAN), 24);
    }

    #[test]
    fn day_14_part_01() {
        assert_eq!(part01(input("day_14").as_str()), 832);
    }

    #[test]
    fn day_14_part_02_example() {
        assert_eq!(part02(SCAN), 93);
    }

    #[test]
    fn day_14_part_2() {
        assert_eq!(part02(input("day_14").as_str()), 27601);
    }
}
