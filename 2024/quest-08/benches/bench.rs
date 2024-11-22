use criterion::{criterion_group, criterion_main, Criterion};

use quest_08 as quest;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| {
        b.iter(|| quest::part_1(include_bytes!("../data/part_1")))
    });
    c.bench_function("part 2", |b| {
        b.iter(|| quest::part_2::<1111, 20240000>(include_bytes!("../data/part_2")))
    });
    c.bench_function("part 3", |b| {
        b.iter(|| quest::part_3::<1111, 202400000>(include_bytes!("../data/part_3")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
