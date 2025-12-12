use criterion::{Criterion, criterion_group, criterion_main};
use day12::{INPUT, part1};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve day12 part1", |b| {
        b.iter(|| part1::solve(black_box(INPUT)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
