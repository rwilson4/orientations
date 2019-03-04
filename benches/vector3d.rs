#[macro_use]
extern crate criterion;

use criterion::{Criterion, Benchmark, Throughput};
use orientations::Vector3d;

fn dot_product(c: &mut Criterion) {
    let x = Vector3d::new([1.0, 2.0, 3.0]);
    let y = Vector3d::new([4.0, 5.0, 6.0]);
    let bench = Benchmark::new(
        "dot_product",
        move |b| b.iter(|| x.dot(&y))
    ).throughput(Throughput::Elements(1));

    c.bench("vector3d:dot_product", bench);
}

fn cross_product(c: &mut Criterion) {
    let x = Vector3d::new([1.0, 2.0, 3.0]);
    let y = Vector3d::new([4.0, 5.0, 6.0]);
    let bench = Benchmark::new(
        "cross_product",
        move |b| b.iter(|| x.cross(&y))
    ).throughput(Throughput::Elements(1));

    c.bench("vector3d:cross_product", bench);
}

criterion_group!(benches, dot_product, cross_product);
criterion_main!(benches);
