use criterion::{Criterion, criterion_group, criterion_main};
use day08::{INPUT, part1, part2};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve day08 part1", |b| {
        b.iter(|| part1::solve(black_box(INPUT), 1000));
    });

    c.bench_function("solve day08 part2", |b| {
        b.iter(|| part2::solve(black_box(INPUT)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
