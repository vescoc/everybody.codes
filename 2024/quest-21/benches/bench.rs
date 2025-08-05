use criterion::{criterion_group, criterion_main, Criterion};

use quest_2024_21 as quest;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("quest-2024-21");
    group.bench_function("part 1", |b| {
        b.iter(|| quest::part_1(include_bytes!("../data/part_1")));
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
