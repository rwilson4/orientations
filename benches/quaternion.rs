#[macro_use]
extern crate criterion;

use criterion::{Criterion, Benchmark, Throughput};
use orientations::{Vector3d, Rotation, Quaternion};
use std::f64::consts::PI;

fn from_angle_axis(c: &mut Criterion) {
    let angle = PI / 2.0;
    let axis = Vector3d::x();
    let bench = Benchmark::new(
        "from_angle_axis",
        move |b| b.iter(|| Quaternion::from_angle_axis(angle, &axis))
    ).throughput(Throughput::Elements(1));

    c.bench("quaternion:from_angle_axis", bench);
}

fn inverse(c: &mut Criterion) {
    let angle = PI / 2.0;
    let axis = Vector3d::x();
    let q = Quaternion::from_angle_axis(angle, &axis);
    let bench = Benchmark::new(
        "inverse",
        move |b| b.iter(|| q.inverse().unwrap())
    ).with_function(
        "inverse_without_check",
        move |b| b.iter(|| q.inverse_unchecked())
    ).throughput(Throughput::Elements(1));

    c.bench("quaternion:inverse", bench);
}

fn angle_axis(c: &mut Criterion) {
    let angle = PI / 2.0;
    let axis = Vector3d::x();
    let q = Quaternion::from_angle_axis(angle, &axis);
    let bench = Benchmark::new(
        "angle_axis",
        move |b| b.iter(|| q.angle_axis())
    ).throughput(Throughput::Elements(1));

    c.bench("quaternion:angle_axis", bench);
}

fn before_after(c: &mut Criterion) {
    let angle = PI / 2.0;
    let q1 = Quaternion::from_angle_axis(angle, &Vector3d::x());
    let q2 = Quaternion::from_angle_axis(angle, &Vector3d::y());
    let bench = Benchmark::new(
        "before",
        move |b| b.iter(|| q1.before(&q2))
    ).with_function(
        "after_safe",
        move |b| b.iter(|| q2.inverse().unwrap().multiply(&q1.inverse().unwrap()).inverse().unwrap())
    ).with_function(
        "after",
        move |b| b.iter(|| q2.after(&q1))
    ).throughput(Throughput::Elements(1));

    c.bench("quaternion:before_after", bench);
}

criterion_group!(benches, from_angle_axis, inverse, angle_axis, before_after);
criterion_main!(benches);
