#![allow(unused)]
use advent_of_code::input;
use glassbench::*;

pub fn benchmark_day01(bench: &mut Bench) {
    bench.task("day 01 part 01", |task| {
        task.iter(advent_of_code::day01::part01)
    });
    bench.task("day 01 part 02", |task| {
        task.iter(advent_of_code::day01::part02)
    });
}

pub fn benchmark_day02(bench: &mut Bench) {
    bench.task("day 02 part 01", |task| {
        task.iter(advent_of_code::day02::part01)
    });
    bench.task("day 02 part 02", |task| {
        task.iter(advent_of_code::day02::part02)
    });
}

pub fn benchmark_day03(bench: &mut Bench) {
    bench.task("day 03 part 01", |task| {
        task.iter(advent_of_code::day03::part01)
    });
    bench.task("day 03 part 02", |task| {
        task.iter(advent_of_code::day03::part02)
    });
}

pub fn benchmark_day04(bench: &mut Bench) {
    bench.task("day 04 part 01", |task| {
        task.iter(advent_of_code::day04::part01)
    });
    bench.task("day 04 part 02", |task| {
        task.iter(advent_of_code::day04::part02)
    });
}

pub fn benchmark_day05(bench: &mut Bench) {
    bench.task("day 05 part 01", |task| {
        task.iter(advent_of_code::day05::part01)
    });
    bench.task("day 05 part 02", |task| {
        task.iter(advent_of_code::day05::part02)
    });
}

pub fn benchmark_day11(bench: &mut Bench) {
    let binding = input("day_11");
    let input = binding.as_str();
    bench.task("day 11 part 01", |task| {
        task.iter(|| advent_of_code::day11::part01(input))
    });
    bench.task("day 11 part 02", |task| {
        task.iter(|| advent_of_code::day11::part02(input))
    });
}

pub fn benchmark_day12(bench: &mut Bench) {
    let binding = input("day_12");
    let input = binding.as_str();
    bench.task("day 12 part 01", |task| {
        task.iter(|| advent_of_code::day12::part01(input, 143))
    });
    bench.task("day 12 part 02", |task| {
        task.iter(|| advent_of_code::day12::part02(input, 143))
    });
}

pub fn benchmark_day13(bench: &mut Bench) {
    let binding = input("day_13");
    let input = binding.as_str();
    bench.task("day 13 part 01", |task| {
        task.iter(|| advent_of_code::day13::part01(input))
    });
    bench.task("day 13 part 02", |task| {
        task.iter(|| advent_of_code::day13::part02(input))
    });
}

pub fn benchmark_day14(bench: &mut Bench) {
    let binding = input("day_14");
    let input = binding.as_str();
    bench.task("day 14 part 01", |task| {
        task.iter(|| advent_of_code::day14::part01(input))
    });
    bench.task("day 14 part 02", |task| {
        task.iter(|| advent_of_code::day14::part02(input))
    });
}

pub fn benchmark_day15(bench: &mut Bench) {
    let binding = input("day_15");
    let input = binding.as_str();
    bench.task("day 15 part 01", |task| {
        task.iter(|| advent_of_code::day15::part01(input, 2000000))
    });
    bench.task("day 15 part 02", |task| {
        task.iter(|| advent_of_code::day15::part02(input, 4000000))
    });
}

pub fn benchmark_day18(bench: &mut Bench) {
    let binding = input("day_18");
    let input = binding.as_str();
    bench.task("day 18 part 01", |task| {
        task.iter(|| advent_of_code::day18::part01(input))
    });
    bench.task("day 18 part 02", |task| {
        task.iter(|| advent_of_code::day18::part02(input))
    });
}

glassbench!(
    "AOC 2022",
    benchmark_day01,
    benchmark_day02,
    benchmark_day03,
    benchmark_day04,
    benchmark_day05,
    benchmark_day11,
    benchmark_day12,
    benchmark_day13,
    benchmark_day14,
    benchmark_day15,
    benchmark_day18,
);
