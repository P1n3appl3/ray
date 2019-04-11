extern crate ray;
use ray::*;
use_all!();

pub fn main() {
    let mut objects: Vec<Box<dyn Hitable>> = (0..4)
        .map(|n| {
            sphere!((8 * n - 12, 3, -20), 3, specular!(0.92, 0.2 * n as f32))
                as Box<dyn Hitable>
        })
        .collect();
    objects.push(sphere!(
        (0, -10000, 0),
        10000,
        diffuse!(checker3d!(solid!(0.1), solid!(0.8), 1.75))
    ));
    let width = 500;
    let height = 120;
    let camera = camera!((0, 150, 500), (0, 3, -20), width, height, 0.85);
    Scene {
        objects: BVHNode::from(&mut objects),
        camera,
        width,
        height,
        samples: 50,
        bounces: 50,
        background: hdr_image("outside.hdr"),
        show_bg: true,
    }
    .render_to_file("sphere_row.png")
    .unwrap();
}
