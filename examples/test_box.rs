extern crate ray;
use ray::axis::Axis;
use ray::camera::Camera;
use ray::model::bvh::BVHNode;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::mesh::Mesh;
use ray::model::rect::Rect;
use ray::model::texture::*;
use ray::model::transform::*;
use ray::scene::*;
use ray::vec3::Vec3;
use std::sync::Arc;

pub fn main() {
    let red = Arc::new(Diffuse::new(Box::new(Solid::new(Vec3::new(
        0.65, 0.05, 0.05,
    )))));
    let blue = Arc::new(Diffuse::new(Box::new(Solid::new(Vec3::from_rgb(
        32, 32, 237,
    )))));
    let white = Arc::new(Diffuse::new(Box::new(Solid::new(Vec3::from(0.73)))));
    let light = Arc::new(Light::new(Box::new(Solid::new(Vec3::from(5)))));
    let metal = Arc::new(Specular::new(Vec3::new(0.91, 0.91, 0.92), 0.0));
    let objects = BVHNode::from(&mut vec![
        // left wall
        Box::new(FlipNormal::new(Rect::yz(
            0.0, 0.0, 555.0, 555.0, 555.0, blue,
        ))) as Box<Hitable>,
        // right wall
        Box::new(Rect::yz(0.0, 0.0, 555.0, 555.0, 0.0, red)),
        // light
        Box::new(Rect::xz(113.0, 127.0, 443.0, 432.0, 554.0, light)),
        // ceiling
        Box::new(FlipNormal::new(Rect::xz(
            0.0,
            0.0,
            555.0,
            555.0,
            555.0,
            white.clone(),
        ))),
        // floor
        Box::new(Rect::xz(0.0, 0.0, 555.0, 555.0, 0.0, white.clone())),
        // back wall
        Box::new(FlipNormal::new(Rect::xy(
            0.0,
            0.0,
            555.0,
            555.0,
            555.0,
            white.clone(),
        ))),
        Box::new(Translate::new(
            Mesh::new("teapot.obj", 80.0, metal),
            Vec3::new(275, 0, 275),
        )),
    ]);
    let width = 500;
    let height = 500;
    let camera = Camera::new(
        Vec3::new(278, 278, -760),
        Vec3::new(278, 278, 0),
        Vec3::new(0, 1, 0),
        40.0,
        width as f32 / height as f32,
        0.0,
    );
    Scene {
        objects,
        camera,
        width,
        height,
        samples: 500,
        bounces: 50,
        background: Box::new(Solid::new(Color::default())),
        show_bg: false,
    }
    .render_to_file("test_box.png")
    .unwrap();
}
