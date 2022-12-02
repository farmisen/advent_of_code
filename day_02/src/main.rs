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
    println!("DAY 02:{:?}", day01());
}

fn day01() -> u32 {
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
    use crate::day01;

    #[test]
    fn it_works() {
        assert_eq!(day01(), 213089);
    }
}
