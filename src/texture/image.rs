use super::Texture;
use crate::scene::Color;
use crate::vec3::{ToF32, Vec3};
use image::hdr::HDRDecoder;
use image::{ImageBuffer, Pixel, Primitive, Rgb, RgbImage};
use std::fs::File;
use std::io::BufReader;

impl<T> Texture for ImageBuffer<Rgb<T>, Vec<T>>
where
    T: 'static + Send + Sync + Primitive + ToF32 + std::fmt::Debug,
{
    fn value(&self, u: f32, v: f32, _p: Vec3) -> Color {
        let x = (u * self.width() as f32) as u32;
        let y = ((1.0 - v) * self.height() as f32 - 0.001) as u32;
        let channels = self.get_pixel(x, y).channels();
        Color::new(channels[0].to(), channels[1].to(), channels[2].to())
    }
}

pub fn load_ldr_image(filename: &str) -> RgbImage {
    image::open(filename).unwrap().to_rgb()
}

pub fn load_hdr_image(filename: &str) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    let hdr_decoder =
        HDRDecoder::new(BufReader::new(File::open(filename).unwrap())).unwrap();
    let metadata = hdr_decoder.metadata();
    ImageBuffer::from_vec(
        metadata.width,
        metadata.height,
        hdr_decoder
            .read_image_hdr()
            .unwrap()
            .into_iter()
            .map(|p| p.channels().to_vec())
            .flatten()
            .collect::<Vec<f32>>(),
    )
    .unwrap()
}
