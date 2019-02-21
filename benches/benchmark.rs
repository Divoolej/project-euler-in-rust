#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
  // c.bench_function("", |b| b.iter(|| fun()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
