use criterion::{criterion_group, criterion_main, Criterion};

use event_2024_11 as event;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("event-2024-11");
    group.bench_function("part 1", |b| {
        b.iter(|| event::part_1(include_bytes!("../data/part_1")));
    });
    group.bench_function("part 1 matrix", |b| {
        b.iter(|| event::part_1_matrix(include_bytes!("../data/part_1")));
    });
    group.bench_function("part 2", |b| {
        b.iter(|| event::part_2(include_bytes!("../data/part_2")));
    });
    group.bench_function("part 2 matrix", |b| {
        b.iter(|| event::part_2_matrix(include_bytes!("../data/part_2")));
    });
    group.bench_function("part 3", |b| {
        b.iter(|| event::part_3::<3>(include_bytes!("../data/part_3")));
    });
    group.bench_function("part 3 matrix", |b| {
        b.iter(|| event::part_3_matrix::<3>(include_bytes!("../data/part_3")));
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
