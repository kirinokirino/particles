use particles_ecs::components::common;
use particles_ecs::components::physics_components;
use particles_ecs::components::physics_obj;
use particles_ecs::systems::physics_systems::*;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use pprof::criterion::{Output, PProfProfiler};
criterion_main!(benches);

criterion_group! {
    name = benches;
    config = Criterion::default()
        .with_profiler(
            PProfProfiler::new(100, Output::Flamegraph(None))
        );
    targets = bench_bubble_sort, bench_merge_sort
}

fn bench_bubble_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("bubble_sort");
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
                let mut input: Vec<u32> = (0..length).collect();
                let mut rng = thread_rng();
                b.iter(|| {
                    input.shuffle(&mut rng);
                    bubble_sort(&mut input);
                });
            },
        );
    }
    group.finish();
}

fn bench_merge_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("merge_sort");
    for length in (0..=10_000).step_by(1_000) {
        group.throughput(Throughput::Elements(length as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(length),
            &length,
            |b, &length| {
                let mut input: Vec<u32> = (0..length).collect();
                let mut rng = thread_rng();
                b.iter(|| {
                    input.shuffle(&mut rng);
                    merge_sort(&mut input);
                });
            },
        );
    }
    group.finish();
}
