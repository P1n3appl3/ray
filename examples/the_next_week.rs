extern crate ray;
use itertools::iproduct;
use rand::random;
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

pub fn main() {
    let materials: Vec<Box<dyn Material>> = vec![
        Box::new(Diffuse::new(Box::new(Solid::new(Vec3::from_scalar(0.73))))),
        Box::new(Diffuse::new(Box::new(Solid::new(Vec3::new(
            0.48, 0.83, 0.53,
        ))))),
        Box::new(Light::new(Box::new(Solid::new(Vec3::from_scalar(12))))),
        Box::new(Specular::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
        Box::new(Specular::new(Vec3::new(0.9, 0.5, 0.5), 0.25)),
        Box::new(Dielectric::new(1.5)),
        Box::new(Light::new(Box::new(Perlin::new(
            0.05,
            Vec3::new(1.6, 0.6, 0.2),
            Rock,
        )))),
        Box::new(Isotropic::new(Box::new(Solid::new(Vec3::new(
            0.2, 0.4, 0.9,
        ))))),
        Box::new(Diffuse::new(Box::new(
            image::open("earth.png").unwrap().to_rgb(),
        ))),
    ];
    let (white, ground, light, mirror, metal, glass, glow_rock, smoke, earth) =
        (0, 1, 2, 3, 4, 5, 6, 7, 8);
    let internal_reflection =
        Box::new(Sphere::new(Vec3::new(360, 150, 145), 70.0, glass));
    let objects = BVHNode::from_items(&mut vec![
        // floor
        Box::new(BVHNode::from_items(
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
                        ground,
                    )) as Box<Hitable>
                })
                .collect::<Vec<Box<Hitable>>>(),
        )) as Box<Hitable>,
        Box::new(XZRect::new(123.0, 147.0, 423.0, 412.0, 554.0, light)),
        Box::new(Sphere::new(Vec3::new(220, 280, 300), 70.0, mirror)),
        Box::new(Sphere::new(Vec3::new(400, 375, 200), 70.0, metal)),
        Box::new(Sphere::new(Vec3::new(260, 150, 45), 50.0, glass)),
        Box::new(Sphere::new(Vec3::new(0, 150, 145), 80.0, glow_rock)),
        Box::new((*internal_reflection).clone()),
        Box::new(Volume::new(0.2, internal_reflection, smoke)),
        // globe
        Box::new(Translate::new(
            Box::new(RotateY::new(
                Box::new(Sphere::new(Vec3::default(), 100.0, earth)),
                60.0,
            )),
            Vec3::new(400, 200, 400),
        )),
        // many tiny spheres
        Box::new(Translate::new(
            Box::new(RotateY::new(
                Box::new(BVHNode::from_items(
                    &mut (0..1000)
                        .map(|_| {
                            Box::new(Sphere::new(random::<Vec3>() * 165.0, 10.0, white))
                                as Box<Hitable>
                        })
                        .collect::<Vec<Box<Hitable>>>(),
                )),
                15.0,
            )),
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
        materials,
        camera,
        width,
        height,
        samples: 500,
        bounces: 100,
        background: Box::new(Solid::new(Color::default())),
    }
    .render_to_file("the_next_week.png")
    .unwrap();
}
