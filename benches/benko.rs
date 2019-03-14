use criterion::*;
use lazy_static::lazy_static;
use ray::camera;
use ray::model::aabb::AABB;
use ray::ray::Ray;
use ray::vec3::Vec3;

lazy_static! {
    static ref BB: AABB = AABB::new(Vec3::from(0), Vec3::from(1));
    static ref R1: Ray = Ray::new(Vec3::from(-1), Vec3::from(1));
    static ref R2: Ray = Ray::new(Vec3::from(-1), Vec3::from(-1));
}

fn bench_rand(c: &mut Criterion) {
    c.bench_function("unit sphere", |b| b.iter(|| Vec3::rand_in_unit_sphere()));
    c.bench_function("unit disk", |b| b.iter(|| camera::rand_in_unit_disk()));
}

fn bench_aabb(c: &mut Criterion) {
    c.bench_function("aabb hit", |b| {
        b.iter(|| BB.hit(*R1, std::f32::MIN, std::f32::MAX))
    });
    c.bench_function("aabb miss", |b| {
        b.iter(|| BB.hit(*R2, std::f32::MIN, std::f32::MAX))
    });
}

criterion_group!(rand_bench, bench_rand);
criterion_group!(aabb_bench, bench_aabb);
criterion_main!(rand_bench);
