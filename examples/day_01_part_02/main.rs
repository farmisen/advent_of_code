use std::{fs, io::Result};

use advent_of_code::{datapath, EMPTY_LINE, LINE_END};
use itertools::sorted;

fn main() -> Result<()> {
    let res = sorted(
        fs::read_to_string(datapath("day_01")?)?
            .split(EMPTY_LINE)
            .map(|inventory| {
                inventory.split(LINE_END).fold(0, |accu, calories| {
                    accu + match calories {
                        "" => 0,
                        _ => calories.parse::<u32>().unwrap(),
                    }
                })
            }),
    )
    .rev()
    .take(3)
    .sum::<u32>();

    // 21308
    println!("Day 01 Part 01:{:?}", res);
    Ok(())
}
