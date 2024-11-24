use criterion::{criterion_group, criterion_main, Criterion};

use quest_15 as quest;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simple", |b| {
        b.iter(|| quest::part_1(include_bytes!("../data/part_1")))
    });
    c.bench_function("complex", |b| {
        b.iter(|| quest::part_2(include_bytes!("../data/part_2")))
    });
    c.bench_function("target", |b| {
        b.iter(|| quest::part_3(include_bytes!("../data/part_3")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
