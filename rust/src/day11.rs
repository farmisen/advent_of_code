use regex::Regex;

use crate::day11::Operation::{Add, Mult, Square};
use crate::EMPTY_LINE;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    divider: u64,
    op: Operation,
    on_true: usize,
    on_false: usize,
    inspections: u64,
}

fn parse_monkey(input: &str) -> Monkey {
    let raw_re = r"Monkey (?P<id>\d+):\s*
  Starting items: (?P<items>\d+(?:,\s*\d+)*)\s*
  Operation: new = old (?P<op>\*|\+) (?P<op_val>old|\d+)\s*
  Test: divisible by (?P<divider>\d+)\s*
    If true: throw to monkey (?P<on_true>\d+)\s*
    If false: throw to monkey (?P<on_false>\d+)";

    let re = Regex::new(raw_re).unwrap();
    let caps = re.captures(input).unwrap();

    Monkey {
        items: caps["items"]
            .split(", ")
            .map(|str| str.parse::<u64>().unwrap())
            .collect(),
        divider: caps["divider"].parse::<u64>().unwrap(),
        op: match &caps["op_val"] {
            "old" => Square,
            _ => {
                let op_val = caps["op_val"].parse::<u64>().unwrap();
                match &caps["op"] {
                    "+" => Add(op_val),
                    "*" => Mult(op_val),
                    _ => unreachable!("unknown op"),
                }
            }
        },
        on_true: caps["on_true"].parse::<usize>().unwrap(),
        on_false: caps["on_false"].parse::<usize>().unwrap(),
        inspections: 0,
    }
}

fn simulate<F>(mut monkeys: Vec<Monkey>, worry_reducer: F, rounds: u64) -> u64
where
    F: Fn(u64) -> u64,
{
    for _ in 0..rounds {
        // iterate on monkeys
        for idx in 0..monkeys.len() {
            // iterate on items
            for item_idx in 0..monkeys[idx].items.len() {
                let item = monkeys[idx].items[item_idx];
                let new_item = worry_reducer(match monkeys[idx].op {
                    Add(val) => item + val,
                    Mult(val) => item * val,
                    Square => item * item,
                });

                let receiver_idx = match new_item % monkeys[idx].divider {
                    0 => monkeys[idx].on_true,
                    _ => monkeys[idx].on_false,
                };
                monkeys[idx].inspections += 1;
                monkeys[receiver_idx].items.push(new_item);
            }

            monkeys[idx].items = vec![];
        }
    }
    let mut t = monkeys.iter().map(|m| m.inspections).collect::<Vec<u64>>();
    t.sort();
    t.split_off(t.len() - 2).iter().product()
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input.split(EMPTY_LINE).map(parse_monkey).collect()
}

pub fn part01(input: &str) -> u64 {
    simulate(parse_monkeys(input), |x| x / 3, 20)
}

pub fn part02(input: &str) -> u64 {
    let monkeys = parse_monkeys(input);
    let modulo: u64 = monkeys.iter().map(|m| m.divider).product();
    simulate(monkeys, |x| x % modulo, 10_000)
}

#[cfg(test)]
mod tests {
    use crate::{
        day11::{part01, part02},
        input,
    };

    const MONKEYS: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
        ";

    #[test]
    fn day_11_part_01() {
        assert_eq!(part01(MONKEYS), 10605);
        assert_eq!(part01(input("day_11").as_str()), 100345);
    }

    #[test]
    fn day_11_part_02() {
        assert_eq!(part02(MONKEYS), 2713310158);
        assert_eq!(part02(input("day_11").as_str()), 28537348205);
    }
}
