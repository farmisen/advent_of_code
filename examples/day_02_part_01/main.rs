use std::collections::HashMap;
use std::{fs, io::Result};

use advent_of_code::{datapath, LINE_END};

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSOR: u32 = 3;
const LOST: u32 = 0;
const DRAW: u32 = 3;
const WON: u32 = 6;

fn main() -> Result<()> {
    let scoring_system: HashMap<&str, u32> = HashMap::from([
        ("A X", ROCK + DRAW),
        ("A Y", PAPER + WON),
        ("A Z", SCISSOR + LOST),
        ("B X", ROCK + LOST),
        ("B Y", PAPER + DRAW),
        ("B Z", SCISSOR + WON),
        ("C X", ROCK + WON),
        ("C Y", PAPER + LOST),
        ("C Z", SCISSOR + DRAW),
    ]);

    let res = fs::read_to_string(datapath("day_02_part_01")?)?
        .split(LINE_END)
        .fold(0, |accu, round| accu + scoring_system[round]);
    // 9651
    println!("Day 01 Part 01:{:?}", res);
    Ok(())
}
