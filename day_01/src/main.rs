use std::fs;

#[cfg(windows)]
const LINE_END: &'static str = "\r\n";
#[cfg(windows)]
const EMPTY_LINE: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const EMPTY_LINE: &str = "\n\n";
#[cfg(not(windows))]
const LINE_END: &str = "\n";

fn main() {
    println!("DAY 01:{:?}", day01());
}

fn day01() -> u32 {
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
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day01;

    #[test]
    fn it_works() {
        assert_eq!(day01(), 72718);
    }
}
