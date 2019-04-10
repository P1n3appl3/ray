use criterion::*;
use lazy_static::lazy_static;
use ray::camera;
use ray::geometry::aabb::AABB;
use ray::ray::Ray;
use ray::vec3::Vec3;

lazy_static! {
    static ref BB: AABB = black_box(AABB::new(Vec3::from(0), Vec3::from(1)));
    static ref R1: Ray = black_box(Ray::new(Vec3::from(-1), Vec3::from(1)));
    static ref R2: Ray = black_box(Ray::new(Vec3::from(-1), Vec3::from(-1)));
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

lazy_static! {
    static ref vec_a: Vec3 = black_box(Vec3::new(-1, 5.3, 7.2));
    static ref vec_b: Vec3 = black_box(Vec3::new(-0.000001, 0, 7.2));
}

fn bench_vec3(c: &mut Criterion) {
    c.bench_function("add", |b| b.iter(|| *vec_a + *vec_b));
    c.bench_function("sub", |b| b.iter(|| *vec_a - *vec_b));
    c.bench_function("mul", |b| b.iter(|| *vec_a * *vec_b));
    c.bench_function("div", |b| b.iter(|| *vec_a / *vec_b));
    c.bench_function("mul scalar", |b| b.iter(|| *vec_a * 3.14));
    c.bench_function("div scalar", |b| b.iter(|| *vec_a / 13.37));
    c.bench_function("max", |b| b.iter(|| vec_a.piecewise_max(&vec_b)));
    c.bench_function("min", |b| b.iter(|| vec_a.piecewise_min(&vec_b)));
    c.bench_function("dot", |b| b.iter(|| vec_a.dot(&vec_b)));
    c.bench_function("cross", |b| b.iter(|| vec_a.cross(&vec_b)));
    c.bench_function("len", |b| b.iter(|| vec_a.len()));
}

criterion_group!(rand_bench, bench_rand);
criterion_group!(aabb_bench, bench_aabb);
criterion_group!(vec3_bench, bench_vec3);
criterion_main!(vec3_bench);
