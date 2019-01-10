use super::camera::Camera;
use super::hitable::*;
use super::material::{scatter, Material::*};
use super::ray::Ray;
use super::vec3::Vec3;
use itertools::iproduct;
use progressive::progress;
use rand::random;
use rayon::prelude::*;

type Color = Vec3;

fn color(r: Ray, world: &impl Hitable, depth: u16, bounces: u16) -> Color {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        if let Some((attenuation, scattered)) = scatter(r, hit) {
            if depth < bounces {
                return attenuation * color(scattered, world, depth + 1, bounces);
            }
        }
        Color::new(0.0, 0.0, 0.0)
    } else {
        // if rays escape they hit this worldsphere gradient
        let t = (r.dir.normalize().y + 1.0) * 0.5;
        Color::from_scalar(1.0 - t) + Color::new(0.5, 0.7, 1.0).scale(t)
    }
}

pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub objects: HitableGroup,
    pub camera: Camera,
    pub samples: u16,
    pub bounces: u16,
}

impl Scene {
    pub fn render(&self) -> Vec<u8> {
        progress(iproduct!((0..self.height).rev(), 0..self.width))
            .map(|(y, x)| {
                let col = (0..self.samples)
                    .into_par_iter()
                    .map(|_| {
                        color(
                            self.camera.get_ray(
                                (x as f32 + random::<f32>()) / self.width as f32,
                                (y as f32 + random::<f32>()) / self.height as f32,
                            ),
                            &self.objects,
                            0,
                            self.bounces,
                        )
                    })
                    .reduce(|| Color::default(), |a, b| a + b)
                    .scale(1.0 / self.samples as f32);
                vec![
                    // sqrt for gamma 2 correction
                    (col.x.sqrt() * 255.99) as u8,
                    (col.y.sqrt() * 255.99) as u8,
                    (col.z.sqrt() * 255.99) as u8,
                ]
            })
            .flatten()
            .collect::<Vec<u8>>()
    }
    pub fn new(
        objects: HitableGroup,
        cam: Camera,
        width: usize,
        height: usize,
        samples: u16,
        bounces: u16,
    ) -> Self {
        Scene {
            objects: objects,
            camera: cam,
            width: width,
            height: height,
            samples: samples,
            bounces: bounces,
        }
    }
    pub fn book_cover() -> Self {
        let mut spheres = HitableGroup::new(vec![
            Box::new(Sphere::new(
                Vec3::new(0.0, -1000.0, 0.0),
                1000.0,
                Diffuse(Vec3::new(0.5, 0.5, 0.5)),
            )),
            Box::new(Sphere::new(
                Vec3::new(-4.0, 1.0, 0.0),
                1.0,
                Diffuse(Vec3::new(0.2, 0.3, 0.7)),
            )),
            Box::new(Sphere::new(
                Vec3::new(4.0, 1.0, 0.0),
                1.0,
                Metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
            )),
            Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Dielectric(1.5))),
        ]);

        for a in -11..11 {
            for b in -11..11 {
                let pos = Vec3::new(
                    a as f32 + 0.9 * random::<f32>(),
                    0.2,
                    b as f32 + 0.9 * random::<f32>(),
                );
                if (pos - Vec3::new(4.0, 0.2, 0.0)).len() < 0.9 {
                    continue;
                }
                spheres.items.push(Box::new(Sphere::new(
                    pos,
                    0.2,
                    match (random::<f32>() * 100.0) as u8 {
                        0...5 => Dielectric(1.5),
                        5...20 => Metal(
                            (Vec3::new(1.0, 1.0, 1.0)
                                + Vec3::new(random(), random(), random()))
                            .scale(0.5),
                            random::<f32>() / 2.0,
                        ),
                        _ => Diffuse(Vec3::new(
                            random::<f32>() * random::<f32>(),
                            random::<f32>() * random::<f32>(),
                            random::<f32>() * random::<f32>(),
                        )),
                    },
                )))
            }
        }

        let width = 150;
        let height = 100;
        let cam = Camera::new(
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            width as f32 / height as f32,
            0.05,
        );

        Scene {
            objects: spheres,
            camera: cam,
            width: width,
            height: height,
            samples: 50,
            bounces: 5,
        }
    }
}