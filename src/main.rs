mod camera;
mod hitable;
mod material;
mod ray;
mod vec3;
use self::camera::Camera;
use self::hitable::HitableGroup;
use self::material::scatter;
use self::ray::Ray;
use self::vec3::Vec3;
use itertools::iproduct;
use png::HasParameters;
use progressive::progress;
use rand::random;
use rayon::prelude::*;
use std::fs;
use std::io::BufWriter;

const WIDTH: usize = 600;
const HEIGHT: usize = 400;
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
    let cam_pos = Vec3::new(0.0, 2.0, 6.0);
    let subject_pos = Vec3::new(0.0, 0.0, 0.0);
    let cam = Camera::new(
        cam_pos,
        subject_pos,
        Vec3::new(0.0, 1.0, 0.0),
        50.0,
        WIDTH as f32 / HEIGHT as f32,
        0.05,
        (cam_pos - subject_pos).len(),
    );
    let world = HitableGroup::random_scene();

    let file = fs::File::create("test.png").unwrap();
    let ref mut writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, WIDTH as u32, HEIGHT as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let data = progress(iproduct!((0..HEIGHT).rev(), 0..WIDTH))
        .map(|(y, x)| {
            let col = (0..SAMPLES)
                .into_par_iter()
                .map(|_| {
                    let u = (x as f32 + random::<f32>()) / WIDTH as f32;
                    let v = (y as f32 + random::<f32>()) / HEIGHT as f32;
                    let r = cam.get_ray(u, v);
                    color(r, &world, 0)
                })
                .reduce(|| Color::default(), |a, b| a + b)
                .scale(1.0 / SAMPLES as f32);
            vec![
                // sqrt for gamma 2 correction
                (col.x.sqrt() * 255.99) as u8,
                (col.y.sqrt() * 255.99) as u8,
                (col.z.sqrt() * 255.99) as u8,
            ]
        })
        .flatten()
        .collect::<Vec<u8>>();
    writer.write_image_data(&data).unwrap();
}
