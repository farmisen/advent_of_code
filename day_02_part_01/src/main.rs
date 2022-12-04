use std::collections::HashMap;
use std::fs;

#[cfg(windows)]
const LINE_END: &'static str = "\r\n";
#[cfg(windows)]
const EMPTY_LINE: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const EMPTY_LINE: &str = "\n\n";
#[cfg(not(windows))]
const LINE_END: &str = "\n";

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSOR: u32 = 3;
const LOST: u32 = 0;
const DRAW: u32 = 3;
const WON: u32 = 6;

fn main() {
    println!("Day 02 Part 01:{:?}", day02_part01());
}

fn day02_part01() -> u32 {
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

    fs::read_to_string("input.txt")
        .unwrap()
        .split(LINE_END)
        .fold(0, |accu, round| accu + scoring_system[round])
}

#[cfg(test)]
mod tests {
    use crate::day02_part01;

    #[test]
    fn it_works() {
        assert_eq!(day02_part01(), 9651);
    }
}
