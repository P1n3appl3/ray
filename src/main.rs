mod camera;
mod hitable;
mod image;
mod material;
mod ray;
mod vec3;
use self::camera::Camera;
use self::hitable::{HitableGroup, Sphere};
use self::image::Image;
use self::material::{scatter, Material};
use self::ray::Ray;
use self::vec3::Vec3;
use rand::random;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;
const SAMPLES: u16 = 100;
const BOUNCES: u16 = 50;

type Color = Vec3;

fn color(r: Ray, world: &impl hitable::Hitable, depth: u16) -> Color {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        if let Some((attenuation, scattered)) = scatter(r, hit) {
            if depth < BOUNCES {
                return attenuation * color(scattered, world, depth + 1);
            }
        }
        Color::new(0.0, 0.0, 0.0)
    } else {
        let t = (r.dir.normalize().y + 1.0) * 0.5;
        Color::from_scalar(1.0 - t) + Color::new(0.5, 0.7, 1.0).scale(t)
    }
}

fn main() {
    let cam = Camera::default();
    let mut img = Image::new(WIDTH, HEIGHT);
    let mut world = HitableGroup::default();
    world.items = vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Material::Diffuse(Vec3::new(0.8, 0.3, 0.3)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Material::Diffuse(Vec3::new(0.8, 0.8, 0.0)),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Material::Metal(Vec3::new(0.8, 0.6, 0.2)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Material::Metal(Vec3::new(0.8, 0.8, 0.8)),
        )),
    ];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut col = Color::default();
            for _ in 0..SAMPLES {
                let u = (x as f32 + random::<f32>()) / WIDTH as f32;
                let v = (y as f32 + random::<f32>()) / HEIGHT as f32;
                let r = cam.get_ray(u, v);
                col += color(r, &world, 0)
            }
            img.content[y][x] = col.scale(1.0 / SAMPLES as f32);
        }
    }
    img.output_png("test.png");
}
