use criterion::{black_box, criterion_group, criterion_main, Criterion};
use colorid::colorid;
use uuid::Uuid;
use nanoid::nanoid;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("colorid", |b| b.iter(|| colorid!()));
    c.bench_function("uuid", |b| b.iter(|| Uuid::new_v4()));
    c.bench_function("nanoid", |b| b.iter(|| nanoid!()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);