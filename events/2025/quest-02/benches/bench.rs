use criterion::{Criterion, criterion_group, criterion_main};

use event_2025_02 as event;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("event-2025-02");
    group.bench_function("part 1", |b| {
        b.iter(|| event::part_1(include_str!("../data/part_1")));
    });
    group.bench_function("part 2", |b| {
        b.iter(|| event::part_2(include_str!("../data/part_2")));
    });
    group.bench_function("part 3", |b| {
        b.iter(|| event::part_3(include_str!("../data/part_3")));
    });

    #[cfg(feature = "simd")]
    {
        let a = event::Complex::parse(include_str!("../data/part_2")).unwrap();
        group.bench_function("simple 10", |b| {
            b.iter(|| event::simple::mandelbrot::<1_000, 10>(a));
        });
        group.bench_function("simd 10/2", |b| {
            b.iter(|| event::simd::mandelbrot::<2, 1_000, 10>(a));
        });
        group.bench_function("simd 10/4", |b| {
            b.iter(|| event::simd::mandelbrot::<4, 1_000, 10>(a));
        });
        group.bench_function("simd 10/8", |b| {
            b.iter(|| event::simd::mandelbrot::<8, 1_000, 10>(a));
        });
        group.bench_function("simd 10/16", |b| {
            b.iter(|| event::simd::mandelbrot::<16, 1_000, 10>(a));
        });
        group.bench_function("simd 10/32", |b| {
            b.iter(|| event::simd::mandelbrot::<32, 1_000, 10>(a));
        });
        group.bench_function("simd 10/64", |b| {
            b.iter(|| event::simd::mandelbrot::<64, 1_000, 10>(a));
        });

        let a = event::Complex::parse(include_str!("../data/part_3")).unwrap();
        group.bench_function("simple 1", |b| {
            b.iter(|| event::simple::mandelbrot::<1_000, 1>(a));
        });
        group.bench_function("simd 1/2", |b| {
            b.iter(|| event::simd::mandelbrot::<2, 1_000, 1>(a));
        });
        group.bench_function("simd 1/4", |b| {
            b.iter(|| event::simd::mandelbrot::<4, 1_000, 1>(a));
        });
        group.bench_function("simd 1/8", |b| {
            b.iter(|| event::simd::mandelbrot::<8, 1_000, 1>(a));
        });
        group.bench_function("simd 1/16", |b| {
            b.iter(|| event::simd::mandelbrot::<16, 1_000, 1>(a));
        });
        group.bench_function("simd 1/32", |b| {
            b.iter(|| event::simd::mandelbrot::<32, 1_000, 1>(a));
        });
        group.bench_function("simd 1/64", |b| {
            b.iter(|| event::simd::mandelbrot::<64, 1_000, 1>(a));
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
