use crate::background::Background;
use crate::model::texture::Texture;
use crate::ray::Ray;
use crate::scene::Color;
use crate::vec3::Vec3;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Image {
    pub fn new(filename: &str) -> Self {
        let decoder = png::Decoder::new(File::open(filename).unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();
        Image {
            width: info.width as usize,
            height: info.height as usize,
            data: buf,
        }
    }
}

impl Texture for Image {
    fn value(&self, u: f32, v: f32, _p: Vec3) -> Vec3 {
        let i = (u * self.width as f32)
            .max(0.0)
            .min((self.width - 1) as f32) as usize;
        let j = ((1.0 - v) * self.height as f32 - 0.001)
            .max(0.0)
            .min((self.height - 1) as f32) as usize;
        Vec3::new(
            self.data[3 * i + 3 * self.width * j] as f32 / 255.0,
            self.data[3 * i + 3 * self.width * j + 1] as f32 / 255.0,
            self.data[3 * i + 3 * self.width * j + 2] as f32 / 255.0,
        )
    }
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

impl Background for Image {
    fn get_color(&self, r: Ray) -> Color {
        use std::f32::consts::PI;
        let v = r.dir.normalize();
        let phi = v.z.atan2(v.x);
        let theta = v.y.asin();
        self.value((1.0 + phi / PI) / 2.0, theta / PI + 0.5, Vec3::default())
    }
}
