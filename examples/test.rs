extern crate ray;
use ray::background::Gradient;
use ray::camera::Camera;
use ray::model::bvh::BVHNode;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::mesh::Mesh;
use ray::model::sphere::Sphere;
use ray::model::texture::{Checkered3D, Solid};
use ray::model::transform::{RotateY, Translate};
use ray::scene::*;
use ray::vec3::Vec3;

pub fn main() {
    let materials = vec![
        Box::new(Diffuse::new(Box::new(Checkered3D::new(
            Box::new(Solid::new(Vec3::new(0.6, 0.1, 0.1))),
            Box::new(Solid::new(Vec3::from(0.8))),
            10.0,
        )))) as Box<dyn Material>,
        Box::new(Diffuse::new(Box::new(Solid::new(Vec3::from(0.7))))),
        Box::new(Specular::new(Vec3::from(1), 0.0)),
    ];
    let (checker, white, mirror) = (0, 1, 2);
    let objects = BVHNode::from(&mut vec![
        Box::new(Sphere::new(Vec3::new(0, -1000, 0), 1000.0, checker))
            as Box<dyn Hitable>,
        Box::new(Translate::new(
            Box::new(Mesh::new("teapot.obj", 0.8, white)),
            Vec3::new(1.5, 0, 2),
        )),
        Box::new(Translate::new(
            Box::new(RotateY::new(
                Box::new(Mesh::new("cube.obj", 1.0, mirror)),
                30.0,
            )),
            Vec3::new(-2, 1, 3),
        )),
    ]);
    let width = 500;
    let height = 500;
    let camera = Camera::new(
        Vec3::new(0, 3, -4),
        Vec3::new(0, 1, 0),
        Vec3::new(0, 1, 0),
        90.0,
        width as f32 / height as f32,
        0.0,
    );
    Scene {
        objects,
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
        show_bg: true,
    }
    .render_to_file("test.png")
    .unwrap();
}
