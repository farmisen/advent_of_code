#![feature(iter_array_chunks)]

use std::{fs, io::Result};

use advent_of_code::{datapath, LINE_END};

fn overlaps(r0: (u32, u32), r1: (u32, u32)) -> bool {
    r0.0 >= r1.0 && r0.0 <= r1.1 || r1.0 >= r0.0 && r1.0 <= r0.1
}

fn main() -> Result<()> {
    let res = fs::read_to_string(datapath("day_04")?)?
        .split(LINE_END)
        .map(|pair| {
            // split the pairs into ranges
            pair.split(',')
                .map(|range| {
                    // split the ranges into numbers and
                    range
                        .split('-')
                        .take(2)
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                        .as_slice()
                        .try_into()
                        .unwrap()
                })
                .map(|numbers: [u32; 2]| (numbers[0], numbers[1]))
                .collect::<Vec<(u32, u32)>>()
                .as_slice()
                .try_into()
                .unwrap()
        })
        .map(|ranges: [(u32, u32); 2]| (ranges[0], ranges[1]))
        .fold(
            0,
            |accu, (r0, r1)| {
                if overlaps(r0, r1) {
                    accu + 1
                } else {
                    accu
                }
            },
        );

    // 861
    println!("Day 04 Part 02:{:?}", res);
    Ok(())
}
