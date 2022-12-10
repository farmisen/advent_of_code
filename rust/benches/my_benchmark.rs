#![allow(unused)]
use advent_of_code::run_all;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 01 part 01", |b| {
        b.iter(|| advent_of_code::day01::part01)
    });
    c.bench_function("day 01 part 02", |b| {
        b.iter(|| advent_of_code::day01::part02)
    });
    c.bench_function("day 02 part 01", |b| {
        b.iter(|| advent_of_code::day02::part01)
    });
    c.bench_function("day 02 part 02", |b| {
        b.iter(|| advent_of_code::day02::part02)
    });
    c.bench_function("day 03 part 01", |b| {
        b.iter(|| advent_of_code::day03::part01)
    });
    c.bench_function("day 03 part 02", |b| {
        b.iter(|| advent_of_code::day03::part02)
    });
    c.bench_function("day 04 part 01", |b| {
        b.iter(|| advent_of_code::day04::part01)
    });
    c.bench_function("day 04 part 02", |b| {
        b.iter(|| advent_of_code::day04::part02)
    });
    c.bench_function("day 05 part 01", |b| {
        b.iter(|| advent_of_code::day05::part01)
    });
    c.bench_function("day 05 part 02", |b| {
        b.iter(|| advent_of_code::day05::part02)
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
