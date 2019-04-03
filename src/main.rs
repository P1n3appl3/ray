use ray::scene_descriptor::load_scene;
use std::env;
use std::path::PathBuf;

fn main() {
    let name = env::args()
        .nth(1)
        .expect("Pass a scene file or run an example");
    //TODO: load from file and render scene
}
