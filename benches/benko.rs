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
    c.bench(
        "unit sphere",
        Benchmark::new("Brute force", |b| b.iter(|| Vec3::rand_in_unit_sphere()))
            .with_function("Polar", |b| b.iter(|| Vec3::almost_faster_rand())),
    );
    c.bench(
        "unit disk",
        Benchmark::new("Brute force", |b| b.iter(|| camera::rand_in_unit_disk()))
            .with_function("Polar", |b| b.iter(|| camera::almost_faster_rand())),
    );
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
