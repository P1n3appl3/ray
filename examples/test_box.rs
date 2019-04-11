extern crate ray;
use ray::*;
use_all!();

pub fn main() {
    let red = diffuse!(solid!(0.65, 0.05, 0.05));
    let blue = diffuse!(solid!(rgb!(32, 32, 237)));
    let white = diffuse!(solid!(0.73));
    let light = light!(solid!(5));
    let metal = specular!((0.91, 0.91, 0.92), 0);
    let objects = BVHNode::from(&mut vec![
        // left wall
        flip_normal!(rect!(xz, (0, -555), (555, 0), 555, blue)) as Box<dyn Hitable>,
        rect!(yz, (0, -555), (555, 0), 0, red), // right wall
        rect!(yz, (113, -432), (443, -127), 554, light), // light
        flip_normal!(rect!(xz, (0, -555), (555, 0), 555, white.clone())), // ceiling
        rect!(xz, (0, -555), (555, 0), 0, white.clone()), // floor
        rect!(xy, (0, 0), (555, 555), -555, white.clone()), // back wall
        translate!(mesh!("teapot.obj", 80, metal), (275, 0, -275)),
    ]);
    let width = 500;
    let height = 500;
    let camera = Camera::simple(
        Vec3::new(278, 278, 760),
        Vec3::new(278, 278, 0),
        width,
        height,
        40.0,
    );
    Scene {
        objects,
        camera,
        width,
        height,
        samples: 500,
        bounces: 50,
        background: solid!(0),
        show_bg: false,
    }
    .render_to_file("test_box.png")
    .unwrap();
}
