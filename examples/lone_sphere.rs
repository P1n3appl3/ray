extern crate ray;
use ray::background;
use ray::camera::Camera;
use ray::model::bvh::BVHNode;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::rect::*;
use ray::model::sphere::Sphere;
use ray::model::texture::{PerlinVariant::*, *};
use ray::scene::*;
use ray::vec3::Vec3;

pub fn lone_sphere() -> Scene {
    let spheres = BVHNode::from_items_sah(&mut vec![
        Box::new(Sphere::new(
            Vec3::new(0, -60, 0),
            60.0,
            Box::new(Diffuse::new(Box::new(Perlin::new(
                0.75,
                Vec3::new(0.8, 0.4, 0.2),
                Marble,
            )))),
        )) as Box<dyn Hitable>,
        Box::new(Sphere::new(
            Vec3::new(0, 2, 0),
            2.0,
            Box::new(Diffuse::new(Box::new(Image::new("earth.png")))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0, 7, 0),
            2.0,
            Box::new(Light::new(Box::new(Solid::new(Vec3::from_scalar(4))))),
        )),
        Box::new(XYRect::new(
            3.0,
            1.0,
            5.0,
            3.0,
            -2.0,
            Box::new(Light::new(Box::new(Solid::new(Vec3::from_scalar(4))))),
        )),
    ]);
    let width = 300;
    let height = 200;
    let cam = Camera::new(
        Vec3::new(13, 5, 3),
        Vec3::new(0, 2, 0),
        Vec3::new(0, 1, 0),
        35.0,
        width as f32 / height as f32,
        0.0,
    );
    Scene {
        objects: spheres,
        camera: cam,
        width: width,
        height: height,
        samples: 250,
        bounces: 50,
        background: Box::new(background::Solid {
            color: Color::default(),
        }),
    }
}

fn main() {
    lone_sphere().render_to_file("lone_sphere.png").unwrap();
}
