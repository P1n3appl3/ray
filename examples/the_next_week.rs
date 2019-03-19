extern crate ray;
use itertools::iproduct;
use rand::random;
use ray::axis::Axis;
use ray::camera::Camera;
use ray::model::bvh::BVHNode;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::prism::Prism;
use ray::model::rect::Rect;
use ray::model::sphere::Sphere;
use ray::model::texture::{PerlinVariant::*, *};
use ray::model::transform::{Rotate, Translate};
use ray::model::volume::Volume;
use ray::scene::*;
use ray::vec3::Vec3;
use std::sync::Arc;

pub fn main() {
    let white = Arc::new(Diffuse::new(Box::new(Solid::new(Vec3::from(0.73)))));
    let ground = Arc::new(Diffuse::new(Box::new(Solid::new(Vec3::new(
        0.48, 0.83, 0.53,
    )))));
    let light = Arc::new(Light::new(Box::new(Solid::new(Vec3::from(12)))));
    let mirror = Arc::new(Specular::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    let metal = Arc::new(Specular::new(Vec3::new(0.9, 0.5, 0.5), 0.25));
    let glass = Arc::new(Dielectric::new(1.5));
    let glow_rock = Arc::new(Light::new(Box::new(Perlin::new(
        0.05,
        Vec3::new(1.6, 0.6, 0.2),
        Rock,
    ))));
    let smoke = Arc::new(Isotropic::new(Box::new(Solid::new(Vec3::new(
        0.2, 0.4, 0.9,
    )))));
    let earth = Arc::new(Diffuse::new(Box::new(
        image::open("earth.png").unwrap().to_rgb(),
    )));
    let internal_reflection =
        Box::new(Sphere::new(Vec3::new(360, 150, 145), 70.0, glass.clone()));
    let objects = BVHNode::from(&mut vec![
        // floor
        Box::new(BVHNode::from(
            &mut iproduct!(0..20, 0..20)
                .map(|(x, z)| {
                    let w = 100.0;
                    let x0 = -1000.0 + x as f32 * w;
                    let z0 = -1000.0 + z as f32 * w;
                    let x1 = x0 + w;
                    let z1 = z0 + w;
                    Box::new(Prism::new(
                        Vec3::new(x0, 0, z0),
                        Vec3::new(x1, 100.0 * (random::<f32>() + 0.01), z1),
                        ground.clone(),
                    )) as Box<Hitable>
                })
                .collect::<Vec<Box<Hitable>>>(),
        )) as Box<Hitable>,
        Box::new(Rect::xz(123.0, 147.0, 423.0, 412.0, 554.0, light)),
        Box::new(Sphere::new(Vec3::new(220, 280, 300), 70.0, mirror)),
        Box::new(Sphere::new(Vec3::new(400, 375, 200), 70.0, metal)),
        Box::new(Sphere::new(Vec3::new(260, 150, 45), 50.0, glass)),
        Box::new(Sphere::new(Vec3::new(0, 150, 145), 80.0, glow_rock)),
        Box::new((*internal_reflection).clone()),
        Box::new(Volume::new(0.2, internal_reflection, smoke)),
        // globe
        Box::new(Translate::new(
            Rotate::new(Sphere::new(Vec3::default(), 100.0, earth), Axis::Y, 60.0),
            Vec3::new(400, 200, 400),
        )),
        // many tiny spheres
        Box::new(Translate::new(
            Rotate::new(
                BVHNode::from(
                    &mut (0..1000)
                        .map(|_| {
                            Box::new(Sphere::new(
                                random::<Vec3>() * 165.0,
                                10.0,
                                white.clone(),
                            )) as Box<Hitable>
                        })
                        .collect::<Vec<Box<Hitable>>>(),
                ),
                Axis::Y,
                15.0,
            ),
            Vec3::new(-100, 270, 395),
        )),
    ]);
    let width = 500;
    let height = 500;
    let camera = Camera::new(
        Vec3::new(600, 300, -800),
        Vec3::new(300, 275, 0),
        Vec3::new(0, 1, 0),
        30.0,
        width as f32 / height as f32,
        0.0,
    );
    Scene {
        objects,
        camera,
        width,
        height,
        samples: 500,
        bounces: 100,
        background: Box::new(Solid::new(Color::default())),
        show_bg: true,
    }
    .render_to_file("the_next_week.png")
    .unwrap();
}
