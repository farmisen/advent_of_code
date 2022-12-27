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
    input(puzzle).split(splitter).map(str::to_string).collect()
}

pub fn input(puzzle: &str) -> String {
    fs::read_to_string(datapath(puzzle).unwrap()).unwrap()
}

trait ReturnOneLiners<T> {
    fn push_and_ret(self, item: T) -> Self;
    fn extend_and_ret(self, item: Vec<T>) -> Self;
}

impl<T> ReturnOneLiners<T> for Vec<T> {
    #[inline]
    fn push_and_ret(mut self, item: T) -> Self {
        self.push(item);
        self
    }

    fn extend_and_ret(mut self, item: Self) -> Self {
        self.extend(item);
        self
    }
}

pub trait LineSplitting {
    fn trimmed_lines(&self) -> LinesIterator;
}

impl LineSplitting for &str {
    fn trimmed_lines(&self) -> LinesIterator {
        LinesIterator { string: self }
    }
}
pub struct LinesIterator<'a> {
    string: &'a str,
}

impl<'a> Iterator for LinesIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.string.is_empty() {
            true => None,
            _ => match self.string.split_once(LINE_END) {
                Some((str1, str2)) => {
                    self.string = str2;
                    Some(str1.trim())
                }
                None => {
                    let res = Some(self.string.trim());
                    self.string = "";
                    res
                }
            },
        }
    }
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day11;
// pub mod day12;
// pub mod day13;
// pub mod nom13;
// pub mod day14;
pub mod day18;

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

    day11::part01(input("day_11").as_str());
}
