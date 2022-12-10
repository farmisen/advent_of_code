#![feature(iter_array_chunks)]

use std::{env::current_dir, fs, io::Result, path::PathBuf};

#[cfg(windows)]
pub const LINE_END: &'static str = "\r\n";
#[cfg(windows)]
pub const EMPTY_LINE: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
pub const EMPTY_LINE: &str = "\n\n";
#[cfg(not(windows))]
pub const LINE_END: &str = "\n";

pub fn datapath(puzzle: &str) -> Result<PathBuf> {
    let mut path = current_dir()?;
    path.push("..");
    path.push("puzzles");
    path.push(puzzle);
    path.push("input.txt");
    Ok(path)
}

pub fn lines(puzzle: &str, splitter: &str) -> Vec<String> {
    fs::read_to_string(datapath(puzzle).unwrap())
        .unwrap()
        .split(splitter)
        .map(str::to_string)
        .collect()
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

pub fn run_all() {
    day01::part01();
    day01::part02();
    day02::part01();
    day02::part02();
    day03::part01();
    day03::part02();
    day04::part01();
    day04::part02();
    day05::part01();
    day05::part02();
}
