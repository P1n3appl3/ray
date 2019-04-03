extern crate ray;
use ray::background;
use ray::bvh::BVHNode;
use ray::camera::Camera;
use ray::geometry::sphere::Sphere;
use ray::geometry::Hitable;
use ray::material::{dielectric::Dielectric, diffuse::Diffuse, specular::Specular};
use ray::scene::*;
use ray::texture::solid::Solid;
use ray::vec3::Vec3;
use std::sync::Arc;

pub fn main() {
    let ground = Arc::new(Diffuse::new(Solid::new(Color::new(0.8, 0.8, 0))));
    let blue = Arc::new(Diffuse::new(Solid::new(Color::new(0.1, 0.2, 0.5))));
    let metal = Arc::new(Specular::new(Color::new(0.8, 0.6, 0.2), 0.1));
    let glass = Arc::new(Dielectric::new(1.into(), 1.5));
    let spheres = BVHNode::from(&mut vec![
        Box::new(Sphere::new(Vec3::new(0, -100.5, -1), 100.0, ground))
            as Box<dyn Hitable>,
        Box::new(Sphere::new(Vec3::new(0, 0, -1), 0.5, blue)),
        Box::new(Sphere::new(Vec3::new(1, 0, -1), 0.5, metal)),
        Box::new(Sphere::new(Vec3::new(-1, 0, -1), 0.5, glass.clone())),
        Box::new(Sphere::new(Vec3::new(-1, 0, -1), -0.45, glass)),
    ]);
    let width = 200;
    let height = 100;
    let camera = Camera::new(
        Vec3::new(0, 0, 0),
        Vec3::new(0, 0, -1),
        Vec3::new(0, 1, 0),
        90.0,
        width as f32 / height as f32,
        0.0,
    );
    Scene {
        objects: spheres,
        camera,
        width,
        height,
        samples: 100,
        bounces: 50,
        background: background::Gradient {
            a: Color::new(1.0, 1.0, 1.0),
            b: Color::new(0.5, 0.7, 1.0),
        },
        show_bg: true,
    }
    .render_to_file("three_spheres.png")
    .unwrap();
}
