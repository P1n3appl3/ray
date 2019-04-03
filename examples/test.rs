#[allow(unused_imports)]
extern crate ray;
use ray::axis::Axis;
use ray::background::Gradient;
use ray::bvh::BVHNode;
use ray::camera::Camera;
use ray::geometry::{mesh::Mesh, prism::Prism, rect::Rect, transform::*, Hitable};
use ray::material::{dielectric::Dielectric, diffuse::Diffuse, specular::Specular};
use ray::scene::*;
use ray::texture::{checker::Checkered, solid::Solid};
use ray::vec3::Vec3;
use std::sync::Arc;

#[allow(unused_variables)]
pub fn main() {
    let checker = Arc::new(Diffuse::new(Checkered::new(
        Solid::new(Vec3::from(0.4)),
        Solid::new(Vec3::from(0.8)),
        20.0,
    )));
    let white = Arc::new(Diffuse::new(Solid::new(Vec3::from(0.7))));
    let red = Arc::new(Specular::new(Vec3::from_rgb(240, 17, 24), 0.35));
    let gold = Arc::new(Specular::new(Vec3::from_rgb(244, 212, 66), 0.6));
    let glass = Arc::new(Dielectric::new(1.into(), 1.5));
    let objects = BVHNode::from(&mut vec![
        Box::new(Rect::xy(
            -100.0,
            -100.0,
            100.0,
            100.0,
            100.0,
            checker.clone(),
        )) as Box<dyn Hitable>,
        Box::new(Rect::xz(-100.0, -100.0, 100.0, 100.0, 0.0, checker.clone())),
        Box::new(Translate::new(
            Rotate::new(Mesh::new("lucy.obj", 200.0, glass), Axis::Y, 180.0),
            Vec3::new(5, 0, 0),
        )),
    ]);
    let width = 300;
    let height = 300;
    let camera = Camera::new(
        Vec3::new(15, 30, -100),
        Vec3::new(0, 15, 0),
        Vec3::new(0, 1, 0),
        30.0,
        width as f32 / height as f32,
        0.0,
    );
    Scene {
        objects: BVHNode::from(objects),
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
    .render_to_file("test.png")
    .unwrap();
}
