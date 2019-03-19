#[allow(unused_imports)]
extern crate ray;
use ray::axis::Axis;
use ray::background::Gradient;
use ray::camera::Camera;
use ray::model::bvh::BVHNode;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::mesh::Mesh;
use ray::model::prism::Prism;
use ray::model::rect::Rect;
use ray::model::texture::{Checkered, Solid};
use ray::model::transform::*;
use ray::scene::*;
use ray::vec3::Vec3;
use std::sync::Arc;

#[allow(unused_variables)]
pub fn main() {
    let checker = Arc::new(Diffuse::new(Box::new(Checkered::new(
        Box::new(Solid::new(Vec3::from(0.4))),
        Box::new(Solid::new(Vec3::from(0.8))),
        20.0,
    ))));
    let white = Arc::new(Diffuse::new(Box::new(Solid::new(Vec3::from(0.7)))));
    let red = Arc::new(Specular::new(Vec3::from_rgb(240, 17, 24), 0.35));
    let glass = Arc::new(Dielectric::new(1.5));
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
        // Box::new(Translate::new(
        //     Mesh::new("teapot.obj", 8.0, white.clone()),
        //     Vec3::new(-5, 0, 0),
        // )),
        Box::new(Translate::new(
            Rotate::new(
                Rotate::new(
                    Rotate::new(
                        Prism::new((-10).into(), 10.into(), white),
                        Axis::X,
                        45.0,
                    ),
                    Axis::Z,
                    45.0,
                ),
                Axis::Y,
                87.0,
            ),
            Vec3::new(0, 20, -10),
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
        background: Box::new(Gradient {
            a: Color::new(1.0, 1.0, 1.0),
            b: Color::new(0.5, 0.7, 1.0),
        }),
        show_bg: true,
    }
    .render_to_file("test.png")
    .unwrap();
}
