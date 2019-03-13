extern crate ray;
use ray::camera::Camera;
use ray::model::bvh::BVHNode;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::mesh::Mesh;
use ray::model::rect::Prism;
use ray::model::sphere::Sphere;
use ray::model::texture::{load_ldr_image, Gradient, Solid};
use ray::model::transform::{RotateY, Translate};
use ray::model::triangle::Triangle;
use ray::scene::*;
use ray::vec3::Vec3;

pub fn main() {
    let materials = vec![
        Box::new(Specular::new(Vec3::from_scalar(1), 0.0)) as Box<dyn Material>,
        Box::new(Diffuse::new(Box::new(Gradient {}))),
        Box::new(Diffuse::new(Box::new(Solid::new(Color::from_rgb(
            200, 200, 175,
        ))))),
    ];
    let (mirror, gradient, floor) = (0, 1, 2);
    let objects = BVHNode::from_items(&mut vec![
        Box::new(Sphere::new(Vec3::new(-2, 3, 4), 2.0, mirror)) as Box<dyn Hitable>,
        Box::new(Sphere::new(Vec3::new(2, 5, 5), 2.0, mirror)),
        // Box::new(Sphere::new(Vec3::new(0, -500, 0), 498.0, floor)),
        Box::new(Sphere::new(Vec3::new(4, -1, -1), 1.0, gradient)),
        Box::new(Translate::new(
            Box::new(Triangle::new(
                Vec3::new(0, 0, 0),
                Vec3::new(1, 2, 0),
                Vec3::new(2, 0, 0),
                gradient,
            )),
            Vec3::new(1.5, 2, -1),
        )),
        Box::new(Translate::new(
            Box::new(RotateY::new(
                Box::new(Prism::new(Vec3::default(), Vec3::new(2, 2, 2), gradient)),
                -35.0,
            )),
            Vec3::new(-4, -1, -2),
        )),
        Box::new(Translate::new(
            Box::new(RotateY::new(Box::new(Mesh::new("teapot.obj", 2)), -30.0)),
            Vec3::new(1, -1, 1),
        )),
    ]);
    let width = 600;
    let height = 600;
    let camera = Camera::new(
        Vec3::new(0, 2, -5),
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
        samples: 10,
        bounces: 50,
        background: Box::new(load_ldr_image("bg.png")),
    }
    .render_to_file("test.png")
    .unwrap();
}
