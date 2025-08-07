use criterion::{criterion_group, criterion_main, Criterion};

use event_2024_18 as event;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("event-2024-18");
    group.bench_function("part 1", |b| {
        b.iter(|| event::part_1(include_bytes!("../data/part_1")));
    });
    group.bench_function("part 2", |b| {
        b.iter(|| event::part_2(include_bytes!("../data/part_2")));
    });

    group.sample_size(10);
    group.bench_function("part 3/par", |b| {
        b.iter(|| event::part_3_par(include_bytes!("../data/part_3")));
    });
    group.bench_function("part 3/nopar", |b| {
        b.iter(|| event::part_3_nopar(include_bytes!("../data/part_3")));
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
