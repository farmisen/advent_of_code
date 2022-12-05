#![feature(iter_array_chunks)]

use std::collections::HashSet;
use std::fs;

#[cfg(windows)]
const LINE_END: &'static str = "\r\n";
#[cfg(windows)]
const EMPTY_LINE: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const EMPTY_LINE: &str = "\n\n";
#[cfg(not(windows))]
const LINE_END: &str = "\n";

fn main() {
    println!("Day 03 Part 01:{:?}", day03_part01());
}

fn day03_part01() -> u32 {
    fs::read_to_string("input.txt")
        .unwrap()
        .split(LINE_END)
        .array_chunks::<3>()
        .map(|group| {
            group
                .iter()
                .map(|rucksack| -> std::collections::HashSet<u8> {
                    HashSet::from_iter(rucksack.bytes())
                })
                .collect::<Vec<HashSet<u8>>>()
        })
        .fold(0, |accu, loadouts| {
            let common = loadouts[0]
                .iter()
                .find(|c| loadouts[1..].iter().all(|set| set.contains(c)))
                .unwrap();

            accu + match common {
                97..=122 => common - 97 + 1,
                65..=90 => common - 65 + 27,
                _ => unreachable!(),
            } as u32
        })
}

#[cfg(test)]
mod tests {
    use crate::day03_part01;

    #[test]
    fn it_works() {
        assert_eq!(day03_part01(), 2585);
    }
}
