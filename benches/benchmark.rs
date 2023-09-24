use match_numbers::{_generate_vec, find_pair, find_pair_iter, find_pair_map};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let my_vec = black_box(_generate_vec(100_000, 5643, 967, 40));
    let target: i32 = 6610;

    c.bench_function("find_pair", |b| b.iter(|| find_pair(&my_vec, &target)));
    c.bench_function("find_pair_iter", |b| {
        b.iter(|| find_pair_iter(&my_vec, &target))
    });
    c.bench_function("find_pair_map", |b| {
        b.iter(|| find_pair_map(&my_vec, &target))
    });
}

criterion_group! {
    name=benches;
    config = Criterion::default().significance_level(0.001).sample_size(10);
    targets=criterion_benchmark
}
criterion_main!(benches);
