#![feature(drain_filter)]
mod camera;
mod model;
mod ray;
mod scene;
mod vec3;
use self::scene::Scene;
use png::HasParameters;
use std::fs;
use std::io::BufWriter;

fn main() {
    let world = Scene::book_cover();
    let file = fs::File::create("output.png").unwrap();
    let ref mut writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, world.width as u32, world.height as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let data = world.render();
    writer.write_image_data(&data).unwrap();
}
