use criterion::{Criterion, criterion_group, criterion_main};

use event_2025_02 as event;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("event-2025-02");
    group.bench_function("part 1", |b| {
        b.iter(|| event::part_1(include_str!("../data/part_1")))
    });
    group.bench_function("part 2", |b| {
        b.iter(|| event::part_2(include_str!("../data/part_2")))
    });
    group.bench_function("part 3", |b| {
        b.iter(|| event::part_3(include_str!("../data/part_3")))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
