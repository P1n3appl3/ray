extern crate ray;
use rand::random;
use ray::background;
use ray::camera::Camera;
use ray::model::bvh::BVHNode;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::sphere::Sphere;
use ray::model::texture::*;
use ray::scene::*;
use ray::vec3::Vec3;

pub fn book_cover() -> Scene {
    let mut spheres = vec![
        Box::new(Sphere::new(
            Vec3::new(0, -1000, 0),
            1000.0,
            Box::new(Diffuse::new(Box::new(Checkered3D::new(
                Box::new(Solid::new(Vec3::new(0.6, 0.1, 0.1))),
                Box::new(Solid::new(Vec3::from_scalar(0.7))),
                10.0,
            )))),
        )) as Box<dyn Hitable>,
        Box::new(Sphere::new(
            Vec3::new(-4, 1, 0),
            1.0,
            Box::new(Diffuse::new(Box::new(Solid::new(Vec3::new(0.2, 0.3, 0.7))))),
        )),
        Box::new(Sphere::new(
            Vec3::new(4, 1, 0),
            1.0,
            Box::new(Specular::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0, 1, 0),
            1.0,
            Box::new(Dielectric::new(1.5)),
        )),
    ];
    for a in -16..11 {
        for b in -16..11 {
            let pos = Vec3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );
            if (pos - Vec3::new(4, 0.2, 0)).len() < 0.9 {
                continue;
            }
            spheres.push(Box::new(Sphere::new(
                pos,
                0.2,
                match (random::<f32>() * 100.0) as u8 {
                    0...5 => Box::new(Dielectric::new(1.5)),
                    5...30 => Box::new(Specular::new(
                        (Vec3::new(1, 1, 1)
                            + Vec3::new(
                                random::<f32>(),
                                random::<f32>(),
                                random::<f32>(),
                            ))
                            / 2.0,
                        random::<f32>().powi(4),
                    )),
                    _ => Box::new(Diffuse::new(Box::new(Solid::new(Vec3::new(
                        random::<f32>() * random::<f32>(),
                        random::<f32>() * random::<f32>(),
                        random::<f32>() * random::<f32>(),
                    ))))),
                },
            )))
        }
    }

    let width = 300;
    let height = 200;
    let cam = Camera::new(
        Vec3::new(13, 2, 3),
        Vec3::new(0, 0, 0),
        Vec3::new(0, 1, 0),
        20.0,
        width as f32 / height as f32,
        0.0,
    );
    Scene {
        objects: BVHNode::from_items_sah(&mut spheres),
        camera: cam,
        width: width,
        height: height,
        samples: 50,
        bounces: 50,
        background: Box::new(background::Gradient {
            a: Color::new(1.0, 1.0, 1.0),
            b: Color::new(0.5, 0.7, 1.0),
        }),
    }
}

fn main() {
    book_cover().render_to_file("book_cover.png").unwrap();
}
