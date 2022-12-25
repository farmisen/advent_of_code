use std::cmp::Ordering;

use itertools::Itertools;

use crate::nom13::{parse_list, Value};
use crate::{EMPTY_LINE, LINE_END};

pub fn parse_packet(input: &str) -> Vec<Value> {
    // print!("IN:{:?}", input);
    match parse_list(input) {
        Ok(list) => list,
        Err(e) => panic!("{e:?}"),
    }
}

pub fn parse_packet_pair(input: &str) -> (Vec<Value>, Vec<Value>) {
    let (a, b) = input.split_once(LINE_END).unwrap();
    (parse_packet(a.trim()), parse_packet(b.trim()))
}

pub fn part01(input: &str) -> u64 {
    input
        .split(EMPTY_LINE)
        .map(parse_packet_pair)
        .enumerate()
        .filter(|(_, (a, b))| a.cmp(b) == Ordering::Less)
        .map(|(idx, _)| idx + 1)
        .sum::<usize>() as u64
}

pub fn part02(input: &str) -> u64 {
    let div1 = vec![Value::List(vec![Value::Int(2)])];
    let div2 = vec![Value::List(vec![Value::Int(6)])];
    let dividers = vec![div1.clone(), div2.clone()];

    input
        .split(LINE_END)
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(parse_packet)
        .chain(dividers)
        .sorted_by(|a, b| a.cmp(b))
        .enumerate()
        .filter(|(_, p)| p.eq(&div1) || p.eq(&div2))
        .map(|(idx, _)| idx + 1)
        .product::<usize>() as u64
}

#[cfg(test)]
mod tests {

    use std::cmp::Ordering;

    use crate::{
        day13::{parse_packet_pair, part01, part02},
        input,
    };

    const PAIR1: &str = "\
    [1,1,3,1,1]
    [1,1,5,1,1]";

    #[test]
    fn compare_not_nested_lists() {
        let (a, b) = parse_packet_pair(PAIR1);
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
        assert_eq!(a.cmp(&a), Ordering::Equal);
    }

    const PAIR2: &str = "\
    [1,1,3]
    [1,1,3,1,1]";

    #[test]
    fn compare_not_nested_lists_of_different_len() {
        let (a, b) = parse_packet_pair(PAIR2);
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
        assert_eq!(a.cmp(&a), Ordering::Equal);
    }

    const PAIR3: &str = "\
    [1,1,[3,1,1]]
    [1,1,[5,1,1]]";

    #[test]
    fn compare_nested_lists() {
        let (a, b) = parse_packet_pair(PAIR3);
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
        assert_eq!(a.cmp(&a), Ordering::Equal);
    }

    const PAIR4: &str = "\
    [1,1,[3],1,1]
    [1,1,5,1,1]";

    #[test]
    fn compare_nested_lists_and_num() {
        let (a, b) = parse_packet_pair(PAIR4);
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
        assert_eq!(a.cmp(&a), Ordering::Equal);
    }

    const PACKETS: &str = "\
    [1,1,3,1,1]
    [1,1,5,1,1]

    [[1],[2,3,4]]
    [[1],4]

    [9]
    [[8,7,6]]

    [[4,4],4,4]
    [[4,4],4,4,4]

    [7,7,7,7]
    [7,7,7]

    []
    [3]

    [[[]]]
    [[]]

    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]
    ";

    #[test]
    fn day_13_part_01_example() {
        assert_eq!(part01(PACKETS), 13);
    }

    #[test]
    fn day_13_part_01() {
        assert_eq!(part01(input("day_13").as_str()), 6046);
    }

    #[test]
    fn day_13_part_02_example() {
        assert_eq!(part02(PACKETS), 140);
    }

    #[test]
    fn day_13_part_02() {
        assert_eq!(part02(input("day_13").as_str()), 21423);
    }
}
