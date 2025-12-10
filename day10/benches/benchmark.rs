use criterion::{Criterion, criterion_group, criterion_main};
use day10::{INPUT, part1, part2};
use std::hint::black_box;

// Part 2 is too slow on the real input, so we benchmark on the example instead.
const EXAMPLE: &str = include_str!("../example.txt");

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve day10 part1", |b| {
        b.iter(|| part1::solve(black_box(INPUT)));
    });

    c.bench_function("solve day10 part2", |b| {
        b.iter(|| part2::solve(black_box(EXAMPLE)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
