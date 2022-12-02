use std::fs;

use itertools::sorted;

#[cfg(windows)]
const LINE_END: &'static str = "\r\n";
#[cfg(windows)]
const EMPTY_LINE: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const EMPTY_LINE: &str = "\n\n";
#[cfg(not(windows))]
const LINE_END: &str = "\n";

fn main() {
    println!("Day 01 Part 01:{:?}", day01_part02());
}

fn day01_part02() -> u32 {
    sorted(
        fs::read_to_string("input.txt")
            .unwrap()
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
    .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::day01_part02;


    #[test]
    fn it_works() {
        assert_eq!(day01_part02(), 213089);
    }
}
