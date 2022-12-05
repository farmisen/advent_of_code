use std::collections::HashSet;
use std::{fs, io::Result};

use advent_of_code::{datapath, LINE_END};
use tuple::TupleElements;

fn main() -> Result<()> {
    let res = fs::read_to_string(datapath("day_03")?)?
        .split(LINE_END)
        .fold(0, |accu, loadout| {
            let compartments: Vec<HashSet<u8>> = loadout
                .split_at(loadout.len() / 2)
                .elements()
                .map(|e| HashSet::from_iter(e.bytes()))
                .collect();

            let common = compartments[0]
                .intersection(&compartments[1])
                .next()
                .unwrap();

            accu + match common {
                97..=122 => common - 97 + 1,
                65..=90 => common - 65 + 27,
                _ => unreachable!(),
            } as u32
        });

    // 7917
    println!("Day 03 Part 01:{:?}", res);
    Ok(())
}
