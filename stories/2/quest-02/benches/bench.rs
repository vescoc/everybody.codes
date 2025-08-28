use criterion::{Criterion, criterion_group, criterion_main};

use story_2_02 as story;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("story-2-02");
    group.bench_function("part 1", |b| {
        b.iter(|| story::part_1(include_str!("../data/part_1")));
    });
    group.bench_function("part 2", |b| {
        b.iter(|| story::part_2::<100>(include_str!("../data/part_2")));
    });
    group.bench_function("part 3", |b| {
        b.iter(|| story::part_3(include_str!("../data/part_3")));
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
