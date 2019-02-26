extern crate ray;
use ray::camera::Camera;
use ray::model::bvh::BVHNode;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::rect::Prism;
use ray::model::sphere::Sphere;
use ray::model::texture::{Gradient, Solid};
use ray::model::transform::{RotateY, Translate};
use ray::scene::*;
use ray::vec3::Vec3;

pub fn test() -> Scene {
    let objects = BVHNode::from_items(&mut vec![
        Box::new(Sphere::new(
            Vec3::new(-2, 3, 8),
            2.0,
            Box::new(Specular::new(Vec3::from_scalar(1), 0.0)),
        )) as Box<dyn Hitable>,
        Box::new(Sphere::new(
            Vec3::new(2, 2, 4),
            1.0,
            Box::new(Specular::new(Vec3::from_scalar(1), 0.0)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0, -100, 0),
            98.0,
            Box::new(Diffuse::new(Box::new(Solid::new(Color::from_rgb(
                200, 200, 175,
            ))))),
        )),
        Box::new(Sphere::new(
            Vec3::new(3, -1, 0),
            1.0,
            Box::new(Diffuse::new(Box::new(Gradient {}))),
        )),
        Box::new(Translate::new(
            Box::new(RotateY::new(
                Box::new(Prism::new(
                    Vec3::default(),
                    Vec3::new(2, 2, 2),
                    Box::new(Diffuse::new(Box::new(Gradient {}))),
                )),
                -35.0,
            )),
            Vec3::new(-2, -1, 0),
        )),
    ]);
    let width = 600;
    let height = 600;
    let cam = Camera::new(
        Vec3::new(0, 2, -4),
        Vec3::new(0, 0, 0),
        Vec3::new(0, 1, 0),
        90.0,
        width as f32 / height as f32,
        0.0,
    );
    Scene {
        objects,
        camera: cam,
        width: width,
        height: height,
        samples: 10,
        bounces: 50,
        background: Box::new(image::open("pier.png").unwrap().to_rgb()),
    }
}

fn main() {
    test().render_to_file("test.png").unwrap();
}
