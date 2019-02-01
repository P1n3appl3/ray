extern crate ray;
use rand::random;
use ray::background;
use ray::camera::Camera;
use ray::model::group::HitableGroup;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::sphere::Sphere;
use ray::model::texture::*;
use ray::scene::*;
use ray::vec3::Vec3;

pub fn book_cover() -> Scene {
    let mut spheres: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere::new(
            Vec3::new(0, -1000, 0),
            1000.0,
            Box::new(Diffuse {
                texture: Box::new(Checkered {
                    a: Box::new(Solid {
                        color: Vec3::new(0.6, 0.1, 0.1),
                    }),
                    b: Box::new(Solid {
                        color: Vec3::new(0.9, 0.9, 0.9),
                    }),
                    size: 3.0,
                }),
            }),
        )),
        Box::new(Sphere::new(
            Vec3::new(-4, 1, 0),
            1.0,
            Box::new(Diffuse {
                texture: Box::new(Solid {
                    color: Vec3::new(0.2, 0.3, 0.7),
                }),
            }),
        )),
        Box::new(Sphere::new(
            Vec3::new(4, 1, 0),
            1.0,
            Box::new(Specular {
                albedo: Vec3::new(0.7, 0.6, 0.5),
                fuzz: 0.0,
            }),
        )),
        Box::new(Sphere::new(
            Vec3::new(0, 1, 0),
            1.0,
            Box::new(Dielectric {
                refractive_index: 1.5,
            }),
        )),
    ];

    for a in -11..11 {
        for b in -11..11 {
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
                    0...5 => Box::new(Dielectric {
                        refractive_index: 1.5,
                    }),
                    5...20 => Box::new(Specular {
                        albedo: (Vec3::new(1, 1, 1)
                            + Vec3::new(
                                random::<f32>(),
                                random::<f32>(),
                                random::<f32>(),
                            ))
                            / 2.0,
                        fuzz: random::<f32>().powi(4),
                    }),
                    _ => Box::new(Diffuse {
                        texture: Box::new(Solid {
                            color: Vec3::new(
                                random::<f32>() * random::<f32>(),
                                random::<f32>() * random::<f32>(),
                                random::<f32>() * random::<f32>(),
                            ),
                        }),
                    }),
                },
            )))
        }
    }

    let width = 150;
    let height = 100;
    let cam = Camera::new(
        Vec3::new(13, 2, 3),
        Vec3::new(0, 0, 0),
        Vec3::new(0, 1, 0),
        20.0,
        width as f32 / height as f32,
        0.0,
    );
    Scene {
        objects: HitableGroup::new(spheres),
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
