extern crate ray;
use ray::*;
use_all!();

pub fn main() {
    let checker = diffuse!(checker!(solid!(0.4), solid!(0.8), 20));
    let white = diffuse!(solid!(0.7));
    let red = specular!(rgb!(240, 17, 24), 0.35);
    let gold = specular!(rgb!(244, 212, 66), 0.6);
    let glass = dielectric!(rgb!(255, 200, 200), 1.5);
    let objects = BVHNode::from(&mut vec![
        rect!(xy, (-100, -100), (100, 100), 100, checker.clone()) as Box<dyn Hitable>,
        rect!(xz, (-100, -100), (100, 100), 0, checker.clone()),
        translate!(
            rotate!(Y, mesh!("lucy.obj", 200, glass), 180),
            vec3!(5, 0, 0)
        ),
    ]);
    let width = 225;
    let height = 300;
    let camera = camera!((15, 30, -100), (2, 18, 0), width, height, 30.0);
    Scene {
        objects,
        camera,
        width,
        height,
        samples: 50,
        bounces: 50,
        background: Gradient {
            a: Color::from(1),
            b: Color::new(0.5, 0.7, 1.0),
        },
        show_bg: true,
    }
    .render_to_file("test.png")
    .unwrap();
}
