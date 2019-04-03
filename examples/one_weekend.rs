extern crate ray;
use rand::prelude::*;
use ray::background::Gradient;
use ray::bvh::BVHNode;
use ray::camera::Camera;
use ray::geometry::{sphere::Sphere, Hitable};
use ray::material::{dielectric::Dielectric, diffuse::Diffuse, specular::Specular};
use ray::scene::*;
use ray::texture::{checker::*, solid::Solid};
use ray::vec3::Vec3;
use std::sync::Arc;

pub fn main() {
    let checker = Arc::new(Diffuse::new(Checkered3D::new(
        Solid::new(Vec3::new(0.6, 0.1, 0.1)),
        Solid::new(Vec3::from(0.7)),
        10.0,
    )));
    let blue = Arc::new(Diffuse::new(Solid::new(Vec3::new(0.2, 0.3, 0.7))));
    let metal = Arc::new(Specular::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    let glass = Arc::new(Dielectric::new(1.into(), 1.5));
    let mut spheres = vec![
        Box::new(Sphere::new(Vec3::new(0, -1000, 0), 1000.0, checker))
            as Box<dyn Hitable>,
        Box::new(Sphere::new(Vec3::new(-4, 1, 0), 1.0, blue)),
        Box::new(Sphere::new(Vec3::new(4, 1, 0), 1.0, metal)),
        Box::new(Sphere::new(Vec3::new(0, 1, 0), 1.0, glass.clone())),
        Box::new(Sphere::new(Vec3::new(0, 1, 0), -0.99, glass.clone())),
    ];
    for a in -15..15 {
        for b in -15..15 {
            let pos = Vec3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );
            if (pos - Vec3::new(4, 0.2, 0)).len() < 0.9 {
                continue;
            }
            spheres.push(Box::new(Sphere::new(
                pos,
                0.2,
                match (thread_rng().gen_range(0, 100)) as u8 {
                    0...5 => glass.clone(),
                    5...30 => Arc::new(Specular::new(
                        (Vec3::new(1, 1, 1) + random::<Vec3>()) / 2.0,
                        random::<f32>().powi(4),
                    )),
                    _ => Arc::new(Diffuse::new(Solid::new(Vec3::new(
                        random::<f32>() * random::<f32>(),
                        random::<f32>() * random::<f32>(),
                        random::<f32>() * random::<f32>(),
                    )))),
                },
            )));
                                              ;
        }
    }

    let width = 300;
    let height = 200;
    let camera = Camera::new(
        Vec3::new(13, 2, 3),
        Vec3::new(0, 0, 0),
        Vec3::new(0, 1, 0),
        20.0,
        width as f32 / height as f32,
        0.0,
    );
    Scene {
        objects: BVHNode::from(&mut spheres),
        camera,
        width,
        height,
        samples: 50,
        bounces: 50,
        background: Gradient {
            a: Color::new(1.0, 1.0, 1.0),
            b: Color::new(0.5, 0.7, 1.0),
        },
        show_bg: true,
    }
    .render_to_file("one_weekend.png")
    .unwrap();
}
