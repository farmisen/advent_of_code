use std::collections::{HashMap, HashSet};

use crate::{ReturnOneLiners, EMPTY_LINE, LINE_END};

#[derive(Debug, PartialEq, Clone)]
pub struct Pos(i64, i64);

impl Pos {
    pub fn path_to(&self, other: &Pos) -> Vec<Self> {
        let &Pos(x_start, y_start) = self;
        let Pos(x_end, y_end) = other;

        let (dx, dy) = ((x_end - x_start).signum(), (y_end - y_start).signum());

        (x_start..x_end + dx)
            .step_by(dx)
            .zip(y_start..y_end + dy)
            .step_by(dy)
            .map(|(x, y)| Pos(x, y))
            .collect()

        // let mut path = vec![];

        // let (dx, dy) = ((x_end - x_start).signum(), (y_end - y_start).signum());
        // let mut p = self.clone();
        // loop {
        //     p = Pos(p.0 + dx, p.1 + dy);
        //     path.push(p.clone());
        //     if p.eq(other) {
        //         break;
        //     }
        // }

        // path
    }
}

#[derive(Debug, PartialEq)]
pub enum Unit {
    Rock(Pos),
    Sand(Pos),
}

pub fn parse_path(input: &str) -> Vec<Pos> {
    input
        .split(" -> ")
        .map(|token| token.split_once(',').unwrap())
        .map(|(str_x, str_y)| Pos(str_x.parse::<i64>().unwrap(), str_y.parse::<i64>().unwrap()))
        .fold(Vec::<Pos>::new(), |mut path, pos| {
            println!("\nPATH:{path:?}, POS:{pos:?}");

            let res = match path.iter().last() {
                None => vec![pos.clone()],
                Some(p) => {
                    println!("P:{p:?}");
                    let mut pt = p.path_to(&pos);
                    println!("PT:{pt:?}");
                    path.append(&mut pt);
                    println!("AP:{path:?}");
                    path
                }
            };
            println!("RES:{res:?}");
            res
        })
}

pub fn parse_scan(input: &str) -> HashSet<Unit> {
    let t = input
        .split(LINE_END)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    println!("{t:?}");

    HashSet::new()
}

pub fn part01(input: &str) -> u64 {
    // parse the input into a HashMap<Pos, Unit> of units
    parse_scan(input);
    // figure out the rock boundary

    // iterate until a unit of sand fall out the boundary

    0
}

#[cfg(test)]
mod tests {
    use crate::day14::{parse_path, Pos};

    const PATH: &str = "498,4 -> 498,6 -> 496,6";

    #[test]
    fn day_14_parse_path() {
        assert_eq!(
            parse_path(PATH),
            vec![
                Pos(498, 4),
                Pos(498, 5),
                Pos(498, 6),
                Pos(497, 6),
                Pos(496, 6)
            ]
        );
    }

    const SCAN: &str = "\
    498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9";

    // #[test]
    // fn day_14_part_01_example() {
    //     assert_eq!(part01(SCAN), 24);
    // }
}
