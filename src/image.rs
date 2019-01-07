use super::vec3::Vec3;
use png::HasParameters;
use std::fs;
use std::io::BufWriter;

type Pixel = Vec3;

impl From<Pixel> for String {
    fn from(item: Pixel) -> Self {
        format!(
            "{} {} {}",
            (item.x * 255.99) as u8,
            (item.y * 255.99) as u8,
            (item.z * 255.99) as u8
        )
    }
}

pub struct Image {
    pub w: usize,
    pub h: usize,
    pub content: Vec<Vec<Pixel>>,
}

impl Image {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w: w,
            h: h,
            content: vec![vec![Pixel::default(); w]; h],
        }
    }

    pub fn output_ppm(&self, filename: &str) {
        let header = format!("P3\n{} {}\n255\n", self.w, self.h);
        let content = self
            .content
            .iter()
            .rev()
            .map(|row| {
                row.iter()
                    // sqrt for gamma 2 correction
                    .map(|p| Pixel::new(p.x.sqrt(), p.y.sqrt(), p.z.sqrt()))
                    .map(Into::into)
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        fs::write(filename, header + &content).expect("Unable to write to file");
    }

    pub fn output_png(&self, filename: &str) {
        let file = fs::File::create(filename).unwrap();
        let ref mut writer = BufWriter::new(file);
        let mut encoder = png::Encoder::new(writer, self.w as u32, self.h as u32);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let data = self
            .content
            .iter()
            .rev()
            .flatten()
            .map(|p| {
                vec![
                    // sqrt for gamma 2 correction
                    (p.x.sqrt() * 255.99) as u8,
                    (p.y.sqrt() * 255.99) as u8,
                    (p.z.sqrt() * 255.99) as u8,
                ]
            })
            .flatten()
            .collect::<Vec<u8>>();
        writer.write_image_data(&data).unwrap(); // Save
    }
}
