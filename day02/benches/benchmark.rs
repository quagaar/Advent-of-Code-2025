use criterion::{Criterion, criterion_group, criterion_main};
use day02::{INPUT, part1, part2};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve day02 part1", |b| {
        b.iter(|| part1::solve(black_box(INPUT)));
    });

    c.bench_function("solve day02 no rayon", |b| {
        b.iter(|| part1::solve_no_rayon(black_box(INPUT)));
    });

    c.bench_function("solve day02 part1 by string", |b| {
        b.iter(|| part1::solve_by_string(black_box(INPUT)));
    });

    c.bench_function("solve day02 part2", |b| {
        b.iter(|| part2::solve(black_box(INPUT)));
    });

    c.bench_function("solve day02 part2 no rayon", |b| {
        b.iter(|| part2::solve_no_rayon(black_box(INPUT)));
    });

    c.bench_function("solve day02 part2 by string", |b| {
        b.iter(|| part2::solve_by_string(black_box(INPUT)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
