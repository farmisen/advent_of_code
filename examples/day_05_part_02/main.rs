#![feature(iter_array_chunks)]

use std::{fs, io::Result};

use advent_of_code::{datapath, LINE_END};
use regex::Regex;

fn main() -> Result<()> {
    let mut stacks: Vec<Vec<char>> = (1..=9).map(|_| Vec::new()).collect();

    fs::read_to_string(datapath("day_05")?)?
        .split(LINE_END)
        .for_each(|line| {
            if line.contains('[') {
                line.chars().enumerate().for_each(|(idx, c)| {
                    if ('A'..='Z').contains(&c) {
                        stacks[(idx - 1) / 4].push(c);
                    }
                });
            } else if line.starts_with("move") {
                let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
                let caps = re.captures(line).unwrap();
                let count = caps
                    .get(1)
                    .map_or(0, |m| m.as_str().parse::<u32>().unwrap());
                let from = caps
                    .get(2)
                    .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
                let to = caps
                    .get(3)
                    .map_or(0, |m| m.as_str().parse::<usize>().unwrap());

                let mut t = (0..count)
                    .map(|_| stacks[from - 1].pop().unwrap())
                    .collect::<Vec<char>>();

                t.reverse(); // TODO very disatisfying, revisit
                t.iter().for_each(|char| stacks[to - 1].push(*char));
            } else if line.is_empty() {
                (0..=8usize).for_each(|n| stacks[n].reverse());
            }
        });

    // PQTJRSHWS
    print!("Day 05 Part 02:");
    (0..=8usize).for_each(|n| print!("{}", stacks[n].pop().unwrap()));
    println!();

    Ok(())
}
