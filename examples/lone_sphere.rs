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
    let spheres = BVHNode::from_items(&mut vec![
        Box::new(Sphere::new(
            Vec3::new(0, 2, 0),
            2.0,
            Box::new(Diffuse::new(Box::new(image::open("earth.png").unwrap().to_rgb()))),
        )) as Box<dyn Hitable>,
        Box::new(Sphere::new(
            Vec3::new(-10, 9, 0),
            2.0,
            Box::new(Light::new(Box::new(Solid::new(Vec3::from_scalar(10))))),
        )),
        Box::new(Sphere::new(
            Vec3::new(15, 9, 0),
            2.0,
            Box::new(Light::new(Box::new(Solid::new(Vec3::from_scalar(10))))),
        )),
        Box::new(XZRect::new(
            -10.0,
            -10.0,
            10.0,
            10.0,
            -0.0,
            // Box::new(Diffuse::new(Box::new(Solid::new(Vec3::from_scalar(0.75))))),
            Box::new(Diffuse::new(Box::new(Perlin::new(
                0.75,
                Vec3::new(0.8, 0.4, 0.2),
                Marble,
            )))),
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
        samples: 1000,
        bounces: 50,
        background: Box::new(background::Solid {
            color: Color::default(),
        }),
    }
}

fn main() {
    lone_sphere().render_to_file("lone_sphere.png").unwrap();
}
