use std::collections::HashMap;

use crate::{lines, LINE_END};

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSOR: u32 = 3;
const LOST: u32 = 0;
const DRAW: u32 = 3;
const WON: u32 = 6;

pub fn part01() -> u32 {
    let scoring_system: HashMap<String, u32> = HashMap::from([
        ("A X".into(), ROCK + DRAW),
        ("A Y".into(), PAPER + WON),
        ("A Z".into(), SCISSOR + LOST),
        ("B X".into(), ROCK + LOST),
        ("B Y".into(), PAPER + DRAW),
        ("B Z".into(), SCISSOR + WON),
        ("C X".into(), ROCK + WON),
        ("C Y".into(), PAPER + LOST),
        ("C Z".into(), SCISSOR + DRAW),
    ]);

    lines("day_02", LINE_END)
        .iter()
        .fold(0, |accu, round| accu + scoring_system[round])
}

pub fn part02() -> u32 {
    let scoring_system: HashMap<String, u32> = HashMap::from([
        ("A X".into(), SCISSOR + LOST),
        ("A Y".into(), ROCK + DRAW),
        ("A Z".into(), PAPER + WON),
        ("B X".into(), ROCK + LOST),
        ("B Y".into(), PAPER + DRAW),
        ("B Z".into(), SCISSOR + WON),
        ("C X".into(), PAPER + LOST),
        ("C Y".into(), SCISSOR + DRAW),
        ("C Z".into(), ROCK + WON),
    ]);

    lines("day_02", LINE_END)
        .iter()
        .fold(0, |accu, round| accu + scoring_system[round])
}

#[cfg(test)]
mod tests {
    use crate::day02::{part01, part02};

    #[test]
    fn day_02_part_01() {
        assert_eq!(part01(), 9651);
    }

    #[test]
    fn day_02_part_02() {
        assert_eq!(part02(), 10560);
    }
}
