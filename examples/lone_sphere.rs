extern crate ray;
use ray::background;
use ray::camera::Camera;
use ray::model::group::HitableGroup;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::rect::*;
use ray::model::sphere::Sphere;
use ray::model::texture::{PerlinVariant::*, *};
use ray::scene::*;
use ray::vec3::Vec3;

pub fn lone_sphere() -> Scene {
    let spheres: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere::new(
            Vec3::new(0, -1000, 0),
            1000.0,
            Box::new(Diffuse {
                texture: Box::new(Perlin::new(5.0, Vec3::new(0.8, 0.4, 0.2), Marble)),
            }),
        )),
        Box::new(Sphere::new(
            Vec3::new(0, 2, 0),
            2.0,
            Box::new(Diffuse {
                texture: Box::new(Checkered {
                    a: Box::new(Solid {
                        color: Vec3::new(0.6, 0.1, 0.1),
                    }),
                    b: Box::new(Solid {
                        color: Vec3::new(0.4, 0.9, 0.9),
                    }),
                    size: 50.0,
                }),
                // texture: Box::new(Image::new("earth.png")),
            }),
        )),
        Box::new(Sphere::new(
            Vec3::new(0, 7, 0),
            2.0,
            Box::new(Light {
                texture: Box::new(Solid {
                    color: Vec3::from_scalar(4),
                }),
            }),
        )),
        Box::new(XYRect::new(
            3.0,
            1.0,
            5.0,
            3.0,
            -2.0,
            Box::new(Light {
                texture: Box::new(Solid {
                    color: Vec3::from_scalar(4),
                }),
            }),
        )),
    ];
    let width = 150;
    let height = 100;
    let cam = Camera::new(
        Vec3::new(13, 5, 3),
        Vec3::new(0, 2, 0),
        Vec3::new(0, 1, 0),
        35.0,
        width as f32 / height as f32,
        0.0,
    );
    Scene {
        objects: HitableGroup::new(spheres),
        camera: cam,
        width: width,
        height: height,
        samples: 500,
        bounces: 50,
        background: Box::new(background::Solid {
            color: Color::default(),
        }),
    }
}

fn main() {
    lone_sphere().render_to_file("lone_sphere.png").unwrap();
}
