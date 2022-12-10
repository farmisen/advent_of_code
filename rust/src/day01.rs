use itertools::sorted;

use crate::{lines, EMPTY_LINE, LINE_END};

pub fn part01() -> u32 {
    *calories().iter().max().unwrap()
}

pub fn part02() -> u32 {
    sorted(calories()).rev().take(3).sum()
}

fn calories() -> Vec<u32> {
    lines("day_01", EMPTY_LINE)
        .iter()
        .map(|inventory| {
            inventory.split(LINE_END).fold(0, |accu, calories| {
                accu + match calories {
                    "" => 0,
                    _ => calories.parse::<u32>().unwrap(),
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day01::{part01, part02};

    #[test]
    fn day_01_part_01() {
        assert_eq!(part01(), 72718);
    }

    #[test]
    fn day_01_part_02() {
        assert_eq!(part02(), 213089);
    }
}
