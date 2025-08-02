use criterion::{criterion_group, criterion_main, Criterion};

use quest_19 as quest;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| {
        b.iter(|| quest::part_1(include_bytes!("../data/part_1")));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
