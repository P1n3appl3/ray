use criterion::*;
use lazy_static::lazy_static;
use ray::camera;
use ray::model::aabb::AABB;
use ray::ray::Ray;
use ray::vec3::Vec3;

lazy_static! {
    static ref aabb: AABB = AABB::new(Vec3::from_scalar(0), Vec3::from_scalar(1));
    static ref r1: Ray = Ray::new(Vec3::from_scalar(-1), Vec3::from_scalar(1));
    static ref r2: Ray = Ray::new(Vec3::from_scalar(-1), Vec3::from_scalar(-1));
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("unit sphere", |b| b.iter(|| Vec3::rand_in_unit_sphere()));
    c.bench_function("unit disk", |b| b.iter(|| camera::rand_in_unit_disk()));
    c.bench_function("aabb hit", |b| {
        b.iter(|| aabb.hit(*r1, std::f32::MIN, std::f32::MAX))
    });
    c.bench_function("aabb miss", |b| {
        b.iter(|| aabb.hit(*r2, std::f32::MIN, std::f32::MAX))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
