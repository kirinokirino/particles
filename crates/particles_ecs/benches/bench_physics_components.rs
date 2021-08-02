// Based on this blog post - Thank you!
// https://www.worthe-it.co.za/blog/2021-06-19-rust-performance-optimization-tools.html

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use particles_ecs::*;
use pprof::criterion::{Output, PProfProfiler};
criterion_main!(benches);

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(50)
        .with_profiler(
            PProfProfiler::new(100, Output::Flamegraph(None))
        );
    targets = bench_components_init, bench_objects_init, bench_components_loop, bench_objects_loop
}

fn bench_components_init(c: &mut Criterion) {
    let mut group = c.benchmark_group("components_init");
    // We want to see how our sorting algorithms perform on different
    // sized inputs, so we call group.bench_with_input in a
    // loop. These are 11 different benchmarks, with names like
    // bubble_sort/1000 and bubble_sort/2000. Criterion will know to
    // show these benchmarks together on the report because they're
    // all in the same group.
    for length in (0..=10_000).step_by(1_000) {
        group.throughput(Throughput::Elements(length as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(length),
            &length,
            |b, &length| {
                let input = &length;
                b.iter(|| {
                    init_ecs_components(&input);
                });
            },
        );
    }
    group.finish();
}

fn bench_objects_init(c: &mut Criterion) {
    let mut group = c.benchmark_group("objects_init");
    // We want to see how our sorting algorithms perform on different
    // sized inputs, so we call group.bench_with_input in a
    // loop. These are 11 different benchmarks, with names like
    // bubble_sort/1000 and bubble_sort/2000. Criterion will know to
    // show these benchmarks together on the report because they're
    // all in the same group.
    for length in (0..=10_000).step_by(1_000) {
        group.throughput(Throughput::Elements(length as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(length),
            &length,
            |b, &length| {
                let input = &length;
                b.iter(|| {
                    init_ecs_obj(&input);
                });
            },
        );
    }
    group.finish();
}

fn bench_components_loop(c: &mut Criterion) {
    let mut group = c.benchmark_group("components_loop");
    // We want to see how our sorting algorithms perform on different
    // sized inputs, so we call group.bench_with_input in a
    // loop. These are 11 different benchmarks, with names like
    // bubble_sort/1000 and bubble_sort/2000. Criterion will know to
    // show these benchmarks together on the report because they're
    // all in the same group.
    for length in (0..=10_000).step_by(1_000) {
        group.throughput(Throughput::Elements(length as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(length),
            &length,
            |b, &length| {
                let input = &length;
                b.iter(|| {
                    components_loop(&input);
                });
            },
        );
    }
    group.finish();
}

fn bench_objects_loop(c: &mut Criterion) {
    let mut group = c.benchmark_group("objects_loop");
    // We want to see how our sorting algorithms perform on different
    // sized inputs, so we call group.bench_with_input in a
    // loop. These are 11 different benchmarks, with names like
    // bubble_sort/1000 and bubble_sort/2000. Criterion will know to
    // show these benchmarks together on the report because they're
    // all in the same group.
    for length in (0..=10_000).step_by(1_000) {
        group.throughput(Throughput::Elements(length as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(length),
            &length,
            |b, &length| {
                let input = &length;
                b.iter(|| {
                    obj_loop(&input);
                });
            },
        );
    }
    group.finish();
}
