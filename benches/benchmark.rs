use match_numbers::{_generate_vec, find_pair_loop, find_pair_map, find_pair_sort};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let my_vec = black_box(_generate_vec(100_000, 5643, 967, 40));
    let target: i32 = 6610;

    c.bench_function("find_pair_loop", |b| {
        b.iter(|| find_pair_loop(&my_vec, &target))
    });
    c.bench_function("find_pair_sort", |b| {
        b.iter(|| find_pair_sort(&my_vec, &target))
    });
    c.bench_function("find_pair_map", |b| {
        b.iter(|| find_pair_map(&my_vec, &target))
    });
}

criterion_group! {
    name=benches;
    config = Criterion::default().significance_level(0.001).sample_size(50);
    targets=criterion_benchmark
}
criterion_main!(benches);
