use ray::scene_descriptor::load_scene;
use std::env;
use std::path::PathBuf;

fn main() {
    let name = env::args()
        .skip(1)
        .next()
        .expect("Pass a scene file or run an example");
    load_scene(&name)
        .render_to_file(PathBuf::from(name).with_extension("png").to_str().unwrap())
        .unwrap();
}
