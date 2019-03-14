extern crate ray;
use ray::background;
use ray::camera::Camera;
use ray::model::bvh::BVHNode;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::sphere::Sphere;
use ray::model::texture::*;
use ray::scene::*;
use ray::vec3::Vec3;

pub fn main() {
    let materials: Vec<Box<dyn Material>> = vec![
        Box::new(Diffuse::new(Box::new(Solid::new(Color::new(0.8, 0.8, 0))))),
        Box::new(Diffuse::new(Box::new(Solid::new(Color::new(
            0.1, 0.2, 0.5,
        ))))),
        Box::new(Specular::new(Color::new(0.8, 0.6, 0.2), 0.1)),
        Box::new(Dielectric::new(1.5)),
    ];
    let (ground, blue, metal, glass) = (0, 1, 2, 3);
    let spheres = BVHNode::from(&mut vec![
        Box::new(Sphere::new(Vec3::new(0, -100.5, -1), 100.0, ground))
            as Box<dyn Hitable>,
        Box::new(Sphere::new(Vec3::new(0, 0, -1), 0.5, blue)),
        Box::new(Sphere::new(Vec3::new(1, 0, -1), 0.5, metal)),
        Box::new(Sphere::new(Vec3::new(-1, 0, -1), 0.5, glass)),
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
        materials,
        camera,
        width,
        height,
        samples: 100,
        bounces: 50,
        background: Box::new(background::Gradient {
            a: Color::new(1.0, 1.0, 1.0),
            b: Color::new(0.5, 0.7, 1.0),
        }),
    }
    .render_to_file("three_spheres.png")
    .unwrap();
}
