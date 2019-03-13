extern crate ray;
use rand::prelude::*;
use ray::background::Gradient;
use ray::camera::Camera;
use ray::model::bvh::BVHNode;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::sphere::Sphere;
use ray::model::texture::*;
use ray::scene::*;
use ray::vec3::Vec3;

pub fn main() {
    let mut materials: Vec<Box<dyn Material>> = vec![
        Box::new(Diffuse::new(Box::new(Checkered3D::new(
            Box::new(Solid::new(Vec3::new(0.6, 0.1, 0.1))),
            Box::new(Solid::new(Vec3::from_scalar(0.7))),
            10.0,
        )))),
        Box::new(Diffuse::new(Box::new(Solid::new(Vec3::new(0.2, 0.3, 0.7))))),
        Box::new(Specular::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
        Box::new(Dielectric::new(1.5)),
    ];
    let checker = 0;
    let blue = 1;
    let metal = 2;
    let glass = 3;
    let mut spheres = vec![
        Box::new(Sphere::new(Vec3::new(0, -1000, 0), 1000.0, checker))
            as Box<dyn Hitable>,
        Box::new(Sphere::new(Vec3::new(-4, 1, 0), 1.0, blue)),
        Box::new(Sphere::new(Vec3::new(4, 1, 0), 1.0, metal)),
        Box::new(Sphere::new(Vec3::new(0, 1, 0), 1.0, glass)),
        Box::new(Sphere::new(Vec3::new(0, 1, 0), -0.99, glass)),
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
            spheres.push(Box::new(Sphere::new(pos, 0.2, materials.len() as MatID)));
            materials.push(match (thread_rng().gen_range(0, 100)) as u8 {
                0...5 => Box::new(Dielectric::new(1.5)),
                5...30 => Box::new(Specular::new(
                    (Vec3::new(1, 1, 1) + random::<Vec3>()) / 2.0,
                    random::<f32>().powi(4),
                )),
                _ => Box::new(Diffuse::new(Box::new(Solid::new(Vec3::new(
                    random::<f32>() * random::<f32>(),
                    random::<f32>() * random::<f32>(),
                    random::<f32>() * random::<f32>(),
                ))))),
            });
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
        objects: BVHNode::from_items(&mut spheres),
        materials,
        camera,
        width,
        height,
        samples: 50,
        bounces: 50,
        background: Box::new(Gradient {
            a: Color::new(1.0, 1.0, 1.0),
            b: Color::new(0.5, 0.7, 1.0),
        }),
    }
    .render_to_file("one_weekend.png")
    .unwrap();
}
