use std::{fs, io::Result};

use advent_of_code::{datapath, EMPTY_LINE, LINE_END};

fn main() -> Result<()> {
    let res = fs::read_to_string(datapath("day_01_part_01")?)?
        .split(EMPTY_LINE)
        .map(|inventory| {
            inventory.split(LINE_END).fold(0, |accu, calories| {
                accu + match calories {
                    "" => 0,
                    _ => calories.parse::<u32>().unwrap(),
                }
            })
        })
        .max()
        .unwrap();

    // 72718
    println!("Day 01 Part 01:{:?}", res);
    Ok(())
}
