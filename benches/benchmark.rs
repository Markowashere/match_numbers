use match_numbers::{find_pair, find_pair_i, find_pair_map, _generate_vec};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let my_vec = black_box(_generate_vec(1000000, 5643, 967, 40));
    let my_vec_2 = my_vec.clone();
    let target: i32 = 6610;

    c.bench_function("find_pair", |b| b.iter(|| find_pair(&my_vec, &target)));
    c.bench_function("find_pair_i", |b| {
        b.iter(|| find_pair_i(&my_vec_2, &target))
    });
    c.bench_function("find_pair_map", |b| {
        b.iter(|| find_pair_map(&my_vec, &target))
    });
}

criterion_group!{
    name=benches;
    config = Criterion::default().sample_size(10);
    targets=criterion_benchmark
}
criterion_main!(benches);
