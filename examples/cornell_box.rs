extern crate ray;
use ray::camera::Camera;
use ray::model::bvh::BVHNode;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::rect::*;
use ray::model::sphere::Sphere;
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
        Box::new(Diffuse::new(Box::new(Solid::new(Vec3::from_scalar(0.73))))),
        Box::new(Light::new(Box::new(Solid::new(Vec3::from_scalar(5))))),
        Box::new(Specular::new(Vec3::new(0.91, 0.91, 0.92), 0.1)),
        Box::new(Dielectric::new(1.5)),
    ];
    let (red, green, white, light, metal, glass) = (0, 1, 2, 3, 4, 5);
    let left_box = Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Prism::new(Vec3::default(), Vec3::new(165, 330, 165), white)),
            19.0,
        )),
        Vec3::new(265, 0, 295),
    ));
    let right_box = Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Prism::new(Vec3::default(), Vec3::new(165, 165, 165), white)),
            -22.0,
        )),
        Vec3::new(130, 0, 65),
    ));
    let objects = BVHNode::from_items(&mut vec![
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
        materials,
        camera,
        width,
        height,
        samples: 500,
        bounces: 50,
        background: Box::new(Solid::new(Color::default())),
    }
    .render_to_file("cornell_box.png")
    .unwrap();
}
