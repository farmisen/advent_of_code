use regex::Regex;

use crate::{lines, LINE_END};

pub fn part01() -> Vec<char> {
    apply_strategy(strat01)
}

pub fn part02() -> Vec<char> {
    apply_strategy(strat02)
}

fn strat01(from: usize, to: usize, count: u32, stacks: &mut [Vec<char>]) {
    (0..count).for_each(|_| {
        let t = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(t);
    });
}

fn strat02(from: usize, to: usize, count: u32, stacks: &mut [Vec<char>]) {
    let mut tmp = (0..count)
        .map(|_| stacks[from - 1].pop().unwrap())
        .collect::<Vec<char>>();

    tmp.reverse(); // TODO very disatisfying, revisit
    tmp.iter().for_each(|char| stacks[to - 1].push(*char));
}

fn apply_strategy<F>(strategy: F) -> Vec<char>
where
    F: Fn(usize, usize, u32, &mut [Vec<char>]),
{
    let mut stacks: Vec<Vec<char>> = (1..=9).map(|_| Vec::new()).collect();

    lines("day_05", LINE_END).iter().for_each(|line| {
        if line.contains('[') {
            line.chars().enumerate().for_each(|(idx, c)| {
                if c.is_ascii_uppercase() {
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

            strategy(from, to, count, &mut stacks);
        } else if line.is_empty() {
            (0..=8usize).for_each(|n| stacks[n].reverse());
        }
    });

    (0..=8usize).map(|n| stacks[n].pop().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use crate::day05::{part01, part02};

    #[test]
    fn day_05_part_01() {
        assert_eq!(part01(), vec!['H', 'B', 'T', 'M', 'T', 'B', 'S', 'D', 'C']);
    }

    #[test]
    fn day_05_part_02() {
        assert_eq!(part02(), vec!['P', 'Q', 'T', 'J', 'R', 'S', 'H', 'W', 'S']);
    }
}
