extern crate ray;
use itertools::iproduct;
use rand::random;
use ray::background;
use ray::camera::Camera;
use ray::model::bvh::BVHNode;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::rect::*;
use ray::model::sphere::Sphere;
use ray::model::texture::{PerlinVariant::*, *};
use ray::model::transform::{RotateY, Translate};
use ray::model::volume::Volume;
use ray::scene::*;
use ray::vec3::Vec3;

pub fn all_features() -> Scene {
    let white = Box::new(Diffuse::new(Box::new(Solid::new(Vec3::from_scalar(0.73)))));
    let ground = Box::new(Diffuse::new(Box::new(Solid::new(Vec3::new(
        0.48, 0.83, 0.53,
    )))));
    let light = Box::new(Light::new(Box::new(Solid::new(Vec3::from_scalar(7.0)))));
    let internal_reflection = Box::new(Sphere::new(
        Vec3::new(360, 150, 145),
        70.0,
        Box::new(Dielectric::new(1.5)),
    ));
    let objects = BVHNode::from_items_sah(&mut vec![
        // floor
        Box::new(BVHNode::from_items_sah(
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
                        ground.clone_box(),
                    )) as Box<Hitable>
                })
                .collect::<Vec<Box<Hitable>>>(),
        )) as Box<Hitable>,
        // light
        Box::new(XZRect::new(123.0, 147.0, 423.0, 412.0, 554.0, light)),
        // mirror
        Box::new(Sphere::new(
            Vec3::new(220, 280, 300),
            70.0,
            Box::new(Specular::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
        )),
        // metal
        Box::new(Sphere::new(
            Vec3::new(400, 375, 200),
            70.0,
            Box::new(Specular::new(Vec3::new(0.9, 0.5, 0.5), 0.25)),
        )),
        // // glass
        Box::new(Sphere::new(
            Vec3::new(260, 150, 45),
            50.0,
            Box::new(Dielectric::new(1.5)),
        )),
        // marbled
        Box::new(Sphere::new(
            Vec3::new(0, 150, 145),
            80.0,
            Box::new(Light::new(Box::new(Perlin::new(
                0.05,
                Vec3::new(1.6, 0.6, 0.2),
                Rock,
            )))),
        )),
        // internal reflection
        internal_reflection.clone(),
        Box::new(Volume::new(
            0.2,
            internal_reflection,
            Box::new(Solid::new(Vec3::new(0.2, 0.4, 0.9))),
        )),
        // smoke
        // Box::new(Volume::new(
        //     0.00005,
        //     Box::new(Sphere::new(
        //         Vec3::default(),
        //         5000.0,
        //         Box::new(Dielectric::new(
        //             0.0, // arbitrary
        //         )),
        //     )),
        //     Box::new(Solid::new(Vec3::from_scalar(1))),
        // )),
        // globe
        Box::new(Translate::new(
            Box::new(RotateY::new(
                Box::new(Sphere::new(
                    Vec3::default(),
                    100.0,
                    Box::new(Diffuse::new(Box::new(Image::new("earth.png")))),
                )),
                60.0,
            )),
            Vec3::new(400, 200, 400),
        )),
        // many tiny spheres
        Box::new(Translate::new(
            Box::new(RotateY::new(
                Box::new(BVHNode::from_items_sah(
                    &mut (0..1000)
                        .map(|_| {
                            Box::new(Sphere::new(
                                Vec3::rand() * 165.0,
                                10.0,
                                white.clone_box(),
                            )) as Box<Hitable>
                        })
                        .collect::<Vec<Box<Hitable>>>(),
                )),
                15.0,
            )),
            Vec3::new(-100, 270, 395),
        )),
    ]);
    let width = 250;
    let height = 250;
    let cam = Camera::new(
        Vec3::new(600, 300, -800),
        Vec3::new(300, 275, 0),
        Vec3::new(0, 1, 0),
        30.0,
        width as f32 / height as f32,
        0.0,
    );
    Scene {
        objects,
        camera: cam,
        width: width,
        height: height,
        samples: 500,
        bounces: 100,
        background: Box::new(background::Solid {
            color: Color::default(),
        }),
    }
}

fn main() {
    all_features().render_to_file("all_features.png").unwrap();
}
