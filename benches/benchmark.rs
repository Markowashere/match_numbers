use match_numbers::{
    find_pair,
    find_pair_i,
    find_pair_map,
    generate_vec
};

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

fn criterion_benchmark(c: &mut Criterion) {
    let my_vec = black_box(generate_vec(10000, 5643, 967, 40));
    let mut my_vec_2 = my_vec.clone();
    let target: i32 = 6610;

    c.bench_function("find_pair", |b| b.iter(|| find_pair(&my_vec, &target)));
    c.bench_function("find_pair_i", |b| b.iter(|| find_pair_i(&mut my_vec_2, &target)));
    c.bench_function("find_pair_map", |b| b.iter(|| find_pair_map(&my_vec, &target)));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);