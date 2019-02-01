extern crate ray;
use ray::background;
use ray::camera::Camera;
use ray::model::group::HitableGroup;
use ray::model::hitable::Hitable;
use ray::model::material::*;
use ray::model::rect::*;
use ray::model::texture::*;
use ray::model::transform::*;
use ray::scene::*;
use ray::vec3::Vec3;

pub fn cornell_box() -> Scene {
    let red = Box::new(Diffuse {
        texture: Box::new(Solid {
            color: Vec3::new(0.65, 0.05, 0.05),
        }),
    });
    let white = Box::new(Diffuse {
        texture: Box::new(Solid {
            color: Vec3::new(0.73, 0.73, 0.73),
        }),
    });
    let green = Box::new(Diffuse {
        texture: Box::new(Solid {
            color: Vec3::new(0.12, 0.45, 0.15),
        }),
    });
    let light = Box::new(Light {
        texture: Box::new(Solid {
            // TODO: figure out why mine is so much brighter
            color: Vec3::from_scalar(1.3),
        }),
    });
    let objects: Vec<Box<dyn Hitable>> = vec![
        // left
        Box::new(FlipNormal {
            obj: Box::new(YZRect::new(0.0, 0.0, 555.0, 555.0, 555.0, green)),
        }),
        // right
        Box::new(YZRect::new(0.0, 0.0, 555.0, 555.0, 0.0, red)),
        // big light
        // Box::new(XZRect::new(113.0, 127.0, 443.0, 432.0, 554.0, light)),
        // light
        Box::new(XZRect::new(213.0, 227.0, 343.0, 332.0, 554.0, light)),
        // ceiling
        Box::new(FlipNormal {
            obj: Box::new(XZRect::new(
                0.0,
                0.0,
                555.0,
                555.0,
                555.0,
                white.clone_box(),
            )),
        }),
        // floor
        Box::new(XZRect::new(0.0, 0.0, 555.0, 555.0, 0.0, white.clone_box())),
        // back
        Box::new(FlipNormal {
            obj: Box::new(XYRect::new(
                0.0,
                0.0,
                555.0,
                555.0,
                555.0,
                white.clone_box(),
            )),
        }),
        // right box
        Box::new(Translate {
            //     obj: Box::new(RotateY::new(
            obj: Box::new(Prism::new(
                Vec3::default(),
                Vec3::new(165, 165, 165),
                white.clone_box(),
            )),
            //         -18.0,
            //     )),
            offset: Vec3::new(130, 0, 65),
        }),
        // left box
        Box::new(Translate {
            // obj: Box::new(RotateY::new(
            obj: Box::new(Prism::new(
                Vec3::default(),
                Vec3::new(165, 330, 165),
                white.clone_box(),
            )),
            // 15.0,
            // )),
            offset: Vec3::new(265, 0, 295),
        }),
    ];
    let width = 150;
    let height = 150;
    let cam = Camera::new(
        Vec3::new(278, 278, -760),
        Vec3::new(278, 278, 0),
        Vec3::new(0, 1, 0),
        40.0,
        width as f32 / height as f32,
        0.0,
    );
    Scene {
        objects: HitableGroup::new(objects),
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
    cornell_box().render_to_file("cornell_box.png").unwrap();
}
