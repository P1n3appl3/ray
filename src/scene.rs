use crate::background::Background;
use crate::camera::Camera;
use crate::model::bvh::BVHNode;
use crate::model::hitable::Hitable;
use crate::ray::Ray;
use crate::vec3::Vec3;
use itertools::iproduct;
use png::HasParameters;
use progressive::progress;
use rand::random;
use rayon::prelude::*;
use std::fs;
use std::io::BufWriter;

pub type Color = Vec3;

fn color(
    r: Ray,
    world: &impl Hitable,
    bg: &dyn Background,
    depth: u16,
    bounces: u16,
) -> Color {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        let emited = hit.material.emit(hit.u, hit.v, hit.point);
        if let Some((attenuation, scattered)) =
            hit.material.scatter(r, hit.normal, hit.point, hit.u, hit.v)
        {
            if depth < bounces {
                return emited
                    + attenuation * color(scattered, world, bg, depth + 1, bounces);
            }
        }
        emited
    } else {
        bg.get_color(r)
    }
}

pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub objects: BVHNode,
    pub camera: Camera,
    pub samples: u16,
    pub bounces: u16,
    pub background: Box<dyn Background>,
}

impl Scene {
    // TODO: add progressive rendering
    pub fn render(&self) -> Vec<u8> {
        progress(iproduct!((0..self.height).rev(), 0..self.width))
            .map(|(y, x)| {
                let col = ((0..self.samples)
                    .into_par_iter()
                    .map(|_| {
                        color(
                            self.camera.get_ray(
                                // TODO: add jitter sampling
                                (x as f32 + random::<f32>()) / self.width as f32,
                                (y as f32 + random::<f32>()) / self.height as f32,
                            ),
                            &self.objects,
                            &*self.background,
                            0,
                            self.bounces,
                        )
                    })
                    .reduce(Color::default, |a, b| a + b)
                    / self.samples as f32)
                    .piecewise_min(Vec3::from_scalar(1));
                vec![
                    // sqrt for gamma 2 correction to brighten image
                    (col.x.sqrt() * 255.999) as u8,
                    (col.y.sqrt() * 255.999) as u8,
                    (col.z.sqrt() * 255.999) as u8,
                ]
            })
            .flatten()
            .collect::<Vec<u8>>()
    }
    pub fn render_to_file(&self, filename: &str) -> std::io::Result<()> {
        let file = fs::File::create(filename)?;
        let writer = BufWriter::new(file);
        let mut encoder =
            png::Encoder::new(writer, self.width as u32, self.height as u32);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        let data = self.render();
        writer.write_image_data(&data)?;
        Ok(())
    }
}
