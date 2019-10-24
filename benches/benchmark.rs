use criterion::{black_box, Criterion};
use phys::MarbleBoard;

#[macro_use]
extern crate criterion;

fn bench_shuffled_generation(c: &mut Criterion) {
    c.bench_function("shuffled(1800)", |b| {
        b.iter(|| MarbleBoard::new_shuffled(black_box(1800)))
    });
}

criterion_group!(benches, bench_shuffled_generation);
criterion_main!(benches);
