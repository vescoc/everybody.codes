use criterion::{criterion_group, criterion_main, Criterion};

use quest_18 as quest;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| {
        b.iter(|| quest::part_1(include_bytes!("../data/part_1")))
    });
    c.bench_function("part 2", |b| {
        b.iter(|| quest::part_2(include_bytes!("../data/part_2")))
    });

    let mut group = c.benchmark_group("part_3");
    group.sample_size(10);
    group.bench_function("par", |b| {
        b.iter(|| quest::part_3_par(include_bytes!("../data/part_3")))
    });
    group.bench_function("nopar", |b| {
        b.iter(|| quest::part_3_nopar(include_bytes!("../data/part_3")))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
