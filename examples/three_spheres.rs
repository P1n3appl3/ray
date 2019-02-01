extern crate ray;
use ray::background;
use ray::camera::Camera;
use ray::model::group::HitableGroup;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::sphere::Sphere;
use ray::model::texture::*;
use ray::scene::*;
use ray::vec3::Vec3;

pub fn three_spheres() -> Scene {
    let spheres: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere::new(
            Vec3::new(0, -100.5, -1),
            100.0,
            Box::new(Diffuse {
                texture: Box::new(Solid {
                    color: Vec3::new(0.8, 0.8, 0),
                }),
            }),
        )),
        Box::new(Sphere::new(
            Vec3::new(0, 0, -1),
            0.5,
            Box::new(Diffuse {
                texture: Box::new(Solid {
                    color: Vec3::new(0.1, 0.2, 0.5),
                }),
            }),
        )),
        Box::new(Sphere::new(
            Vec3::new(1, 0, -1),
            0.5,
            Box::new(Specular {
                albedo: Vec3::new(0.8, 0.6, 0.2),
                fuzz: 0.1,
            }),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1, 0, -1),
            0.5,
            Box::new(Dielectric {
                refractive_index: 1.5,
            }),
        )),
        /* Box::new(Sphere::new(
         *     Vec3::new(-1, 0, -1),
         *     -0.45,
         *     Dielectric(1.5),
         * )), */
    ];
    let width = 200;
    let height = 100;
    let cam = Camera::new(
        Vec3::new(0, 0, 0),
        Vec3::new(0, 0, -1),
        Vec3::new(0, 1, 0),
        90.0,
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
    three_spheres().render_to_file("three_spheres.png").unwrap();
}
