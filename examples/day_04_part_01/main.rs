#![feature(iter_array_chunks)]

use std::{fs, io::Result};

use advent_of_code::{datapath, LINE_END};

fn contains(r0: (u32, u32), r1: (u32, u32)) -> bool {
    r0.0 <= r1.0 && r0.1 >= r1.1
}

fn main() -> Result<()> {
    let res = fs::read_to_string(datapath("day_04_part_01")?)?
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
        .fold(0, |accu, (r0, r1)| {
            if contains(r0, r1) || contains(r1, r0) {
                accu + 1
            } else {
                accu
            }
        });

    // 441
    println!("Day 04 Part 01:{:?}", res);
    Ok(())
}
