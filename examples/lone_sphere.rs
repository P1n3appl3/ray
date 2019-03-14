extern crate ray;
use ray::camera::Camera;
use ray::model::bvh::BVHNode;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::rect::*;
use ray::model::sphere::Sphere;
use ray::model::texture::{PerlinVariant::*, *};
use ray::scene::*;
use ray::vec3::Vec3;

pub fn main() {
    let materials: Vec<Box<dyn Material>> = vec![
        Box::new(Diffuse::new(Box::new(
            image::open("earth.png").unwrap().to_rgb(),
        ))),
        Box::new(Light::new(Box::new(Solid::new(Vec3::from(15))))),
        Box::new(Diffuse::new(Box::new(Perlin::new(
            0.75,
            Vec3::new(0.8, 0.4, 0.2),
            Marble,
        )))),
    ];
    let (earth, light, marble) = (0, 1, 2);
    let spheres = BVHNode::from(&mut vec![
        Box::new(Sphere::new(Vec3::new(0, 2, 0), 2.0, earth)) as Box<dyn Hitable>,
        Box::new(Sphere::new(Vec3::new(-10, 7, 3), 2.0, light)),
        Box::new(Sphere::new(Vec3::new(10, 7, -1), 2.0, light)),
        Box::new(XZRect::new(-10.0, -10.0, 10.0, 10.0, -0.0, marble)),
    ]);
    let width = 300;
    let height = 200;
    let camera = Camera::new(
        Vec3::new(13, 5, 3),
        Vec3::new(0, 2, 0),
        Vec3::new(0, 1, 0),
        35.0,
        width as f32 / height as f32,
        0.0,
    );
    Scene {
        objects: spheres,
        materials,
        camera,
        width,
        height,
        samples: 100,
        bounces: 50,
        background: Box::new(Solid::new(Color::default())),
    }
    .render_to_file("lone_sphere.png")
    .unwrap();
}
