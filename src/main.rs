mod camera;
mod hitable;
mod material;
mod ray;
mod vec3;
use self::camera::Camera;
use self::hitable::{HitableGroup, Sphere};
use self::material::{scatter, Material::*};
use self::ray::Ray;
use self::vec3::Vec3;
use itertools::iproduct;
use png::HasParameters;
use progressive::progress;
use rand::random;
use std::fs;
use std::io::BufWriter;

const WIDTH: usize = 600;
const HEIGHT: usize = 300;
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
    let mut world = HitableGroup::default();
    world.items = vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Diffuse(Vec3::new(0.1, 0.2, 0.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Diffuse(Vec3::new(0.8, 0.8, 0.0)),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Metal(Vec3::new(0.8, 0.6, 0.2), 0.8),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.2, 0.0, -0.5),
            0.25,
            Dielectric(1.5),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.2, 0.0, -0.5),
            -0.20,
            Dielectric(1.5),
        )),
    ];

    let file = fs::File::create("test.png").unwrap();
    let ref mut writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, WIDTH as u32, HEIGHT as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let data = progress(iproduct!((0..HEIGHT).rev(), 0..WIDTH))
        .map(|(y, x)| {
            let mut col = Color::default();
            for _ in 0..SAMPLES {
                let u = (x as f32 + random::<f32>()) / WIDTH as f32;
                let v = (y as f32 + random::<f32>()) / HEIGHT as f32;
                let r = cam.get_ray(u, v);
                col += color(r, &world, 0)
            }
            let temp = col.scale(1.0 / SAMPLES as f32);
            vec![
                // sqrt for gamma 2 correction
                (temp.x.sqrt() * 255.99) as u8,
                (temp.y.sqrt() * 255.99) as u8,
                (temp.z.sqrt() * 255.99) as u8,
            ]
        })
        .flatten()
        .collect::<Vec<u8>>();
    writer.write_image_data(&data).unwrap();
}
