#![feature(iter_array_chunks)]

use std::collections::HashSet;
use std::{fs, io::Result};

use advent_of_code::{datapath, LINE_END};

fn main() -> Result<()> {
    let res = fs::read_to_string(datapath("day_03_part_02")?)?
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
        });

    // 2585
    println!("Day 03 Part 02:{:?}", res);
    Ok(())
}
