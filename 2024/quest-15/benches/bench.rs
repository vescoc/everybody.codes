use criterion::{criterion_group, criterion_main, Criterion};

use quest_2024_15 as quest;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("quest-2024-15");
    group.bench_function("part 1", |b| {
        b.iter(|| quest::part_1(include_bytes!("../data/part_1")));
    });
	group.bench_function("part 2", |b| {
        b.iter(|| quest::part_2(include_bytes!("../data/part_2")));
    });

    group.sample_size(10).bench_function("part 3", |b| {
        b.iter(|| quest::part_3(include_bytes!("../data/part_3")));
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
