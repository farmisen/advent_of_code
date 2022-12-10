use std::collections::HashSet;

use tuple::TupleElements;

use crate::{lines, LINE_END};

pub fn part01() -> u32 {
    lines("day_03", LINE_END).iter().fold(0, |accu, loadout| {
        let compartments: Vec<HashSet<u8>> = loadout
            .split_at(loadout.len() / 2)
            .elements()
            .map(|e| HashSet::from_iter(e.bytes()))
            .collect();

        let common = compartments[0]
            .intersection(&compartments[1])
            .next()
            .unwrap();

        accu + priority(common)
    })
}

pub fn part02() -> u32 {
    lines("day_03", LINE_END)
        .iter()
        .array_chunks::<3>()
        .map(|group| {
            group
                .iter()
                .map(|rucksack| -> std::collections::HashSet<u8> {
                    HashSet::from_iter(rucksack.bytes())
                })
                .collect::<Vec<HashSet<u8>>>()
        })
        .fold(0, |accu, loadouts| {
            let common = loadouts[0]
                .iter()
                .find(|c| loadouts[1..].iter().all(|set| set.contains(c)))
                .unwrap();

            accu + priority(common)
        })
}

fn priority(c: &u8) -> u32 {
    (match c {
        97..=122 => c - 97 + 1,
        65..=90 => c - 65 + 27,
        _ => unreachable!(),
    }) as u32
}

#[cfg(test)]
mod tests {
    use crate::day03::{part01, part02};

    #[test]
    fn day_03_part_01() {
        assert_eq!(part01(), 7917);
    }

    #[test]
    fn day_03_part_02() {
        assert_eq!(part02(), 2585);
    }
}
