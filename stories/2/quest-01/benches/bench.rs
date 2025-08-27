use criterion::{Criterion, criterion_group, criterion_main};

use story_2_01 as story;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("story-2-01");
    group.bench_function("part 1", |b| {
        b.iter(|| story::part_1(include_str!("../data/part_1")));
    });
    group.bench_function("part 2", |b| {
        b.iter(|| story::part_2(include_str!("../data/part_2")));
    });
    group.bench_function("part 3 fast", |b| {
        b.iter(|| story::part_3_fast(include_str!("../data/part_3")));
    });
    group.finish();
}

fn criterion_benchmark_slow(c: &mut Criterion) {
    let mut group = c.benchmark_group("story-2-01");
    group.sample_size(10);
    group.bench_function("part 3 bf", |b| {
        b.iter(|| story::part_3_bf(include_str!("../data/part_3")));
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_slow);
criterion_main!(benches);
