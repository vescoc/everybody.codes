#![allow(clippy::unreadable_literal)]

use criterion::{criterion_group, criterion_main, Criterion};

use event_2024_08 as event;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("event-2024-08");
    group.bench_function("part 1", |b| {
        b.iter(|| event::part_1(include_bytes!("../data/part_1")));
    });
    group.bench_function("part 2", |b| {
        b.iter(|| event::part_2::<1111, 20240000>(include_bytes!("../data/part_2")));
    });
    group.bench_function("part 3", |b| {
        b.iter(|| event::part_3::<1111, 202400000>(include_bytes!("../data/part_3")));
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
