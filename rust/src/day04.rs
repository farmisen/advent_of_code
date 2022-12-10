use crate::{lines, LINE_END};

pub fn part01() -> u32 {
    apply_strategy(any_contains)
}

pub fn part02() -> u32 {
    apply_strategy(overlaps)
}

fn apply_strategy<F>(strategy: F) -> u32
where
    F: Fn((u32, u32), (u32, u32)) -> bool,
{
    lines("day_04", LINE_END)
        .iter()
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
            // if contains(r0, r1) || contains(r1, r0) {
            if strategy(r0, r1) {
                accu + 1
            } else {
                accu
            }
        })
}

fn priority(c: &u8) -> u32 {
    (match c {
        97..=122 => c - 97 + 1,
        65..=90 => c - 65 + 27,
        _ => unreachable!(),
    }) as u32
}

fn contains(r0: (u32, u32), r1: (u32, u32)) -> bool {
    r0.0 <= r1.0 && r0.1 >= r1.1
}

fn any_contains(r0: (u32, u32), r1: (u32, u32)) -> bool {
    contains(r0, r1) || contains(r1, r0)
}

fn overlaps(r0: (u32, u32), r1: (u32, u32)) -> bool {
    r0.0 >= r1.0 && r0.0 <= r1.1 || r1.0 >= r0.0 && r1.0 <= r0.1
}
#[cfg(test)]
mod tests {
    use crate::day04::{part01, part02};

    #[test]
    fn day_04_part_01() {
        assert_eq!(part01(), 441);
    }

    #[test]
    fn day_04_part_02() {
        assert_eq!(part02(), 861);
    }
}
