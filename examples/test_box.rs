extern crate ray;
use ray::camera::Camera;
use ray::model::bvh::BVHNode;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::mesh::Mesh;
use ray::model::rect::*;
use ray::model::texture::*;
use ray::model::transform::*;
use ray::scene::*;
use ray::vec3::Vec3;

pub fn main() {
    let materials: Vec<Box<dyn Material>> = vec![
        Box::new(Diffuse::new(Box::new(Solid::new(Vec3::new(
            0.65, 0.05, 0.05,
        ))))),
        Box::new(Diffuse::new(Box::new(Solid::new(Vec3::new(
            0.12, 0.45, 0.15,
        ))))),
        Box::new(Diffuse::new(Box::new(Solid::new(Vec3::from(0.73))))),
        Box::new(Light::new(Box::new(Solid::new(Vec3::from(5))))),
        Box::new(Specular::new(Vec3::new(0.91, 0.91, 0.92), 0.0)),
    ];
    let (red, green, white, light, metal) = (0, 1, 2, 3, 4);
    let objects = BVHNode::from(&mut vec![
        // left wall
        Box::new(FlipNormal::new(Box::new(YZRect::new(
            0.0, 0.0, 555.0, 555.0, 555.0, green,
        )))) as Box<Hitable>,
        // right wall
        Box::new(YZRect::new(0.0, 0.0, 555.0, 555.0, 0.0, red)),
        // light
        Box::new(XZRect::new(113.0, 127.0, 443.0, 432.0, 554.0, light)),
        // ceiling
        Box::new(FlipNormal::new(Box::new(XZRect::new(
            0.0, 0.0, 555.0, 555.0, 555.0, white,
        )))),
        // floor
        Box::new(XZRect::new(0.0, 0.0, 555.0, 555.0, 0.0, white)),
        // back wall
        Box::new(FlipNormal::new(Box::new(XYRect::new(
            0.0, 0.0, 555.0, 555.0, 555.0, white,
        )))),
        Box::new(Translate::new(
            Box::new(Mesh::new("teapot.obj", 80.0, metal)),
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
        materials,
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
