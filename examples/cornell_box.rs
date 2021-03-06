extern crate ray;
use ray::axis::Axis;
use ray::bvh::BVHNode;
use ray::camera::Camera;
use ray::geometry::prism::Prism;
use ray::geometry::rect::Rect;
use ray::geometry::sphere::Sphere;
use ray::geometry::transform::*;
use ray::geometry::Hitable;
use ray::material::{
    dielectric::Dielectric, diffuse::Diffuse, light::Light, specular::Specular,
};
use ray::scene::*;
use ray::texture::solid::Solid;
use ray::vec3::Vec3;
use std::sync::Arc;

pub fn main() {
    let red = Arc::new(Diffuse::new(Solid::new(Vec3::new(0.65, 0.05, 0.05))));
    let green = Arc::new(Diffuse::new(Solid::new(Vec3::new(0.12, 0.45, 0.15))));
    let white = Arc::new(Diffuse::new(Solid::new(Vec3::from(0.73))));
    let light = Arc::new(Light::new(Solid::new(Vec3::from(5))));
    let metal = Arc::new(Specular::new(Vec3::new(0.91, 0.91, 0.92), 0.1));
    let glass = Arc::new(Dielectric::new(Color::new(0.7, 0.7, 1.0), 1.5));
    let left_box = Box::new(Translate::new(
        Rotate::new(
            Prism::new(Vec3::zero(), Vec3::new(165, 330, 165), white.clone()),
            Axis::Y,
            19.0,
        ),
        Vec3::new(265, 0, 295),
    ));
    let right_box = Box::new(Translate::new(
        Rotate::new(
            Prism::new(Vec3::zero(), Vec3::new(165, 165, 165), white.clone()),
            Axis::Y,
            -22.0,
        ),
        Vec3::new(130, 0, 65),
    ));
    let objects = BVHNode::from(&mut vec![
        // left wall
        Box::new(FlipNormal::new(Rect::yz(
            0.0, 0.0, 555.0, 555.0, 555.0, green,
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
            0.0, 0.0, 555.0, 555.0, 555.0, white,
        ))),
        right_box,
        left_box,
        // glass sphere on right box
        Box::new(Sphere::new(Vec3::new(190, 235, 145), 70.0, glass)),
        // aluminum sphere on left box
        Box::new(Sphere::new(Vec3::new(355, 400, 300), 70.0, metal)),
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
        background: Solid::new(Color::zero()),
        show_bg: false,
    }
    .render_to_file("cornell_box.png")
    .unwrap();
}
