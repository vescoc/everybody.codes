use std::hint;

use criterion::{Criterion, criterion_group, criterion_main};

use event_2025_14 as event;

const PART_1: &str = include_str!("../data/part_1");
const PART_2: &str = include_str!("../data/part_2");
const PART_3: &str = include_str!("../data/part_3");

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("event-2025-14");
    group.bench_function("part 1", |b| {
        b.iter(|| hint::black_box(event::part_1(hint::black_box(PART_1))));
    });
    group.bench_function("part 2", |b| {
        b.iter(|| hint::black_box(event::part_2(hint::black_box(PART_2))));
    });
    group.bench_function("part 3", |b| {
        b.iter(|| hint::black_box(event::part_3(hint::black_box(PART_3))));
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
