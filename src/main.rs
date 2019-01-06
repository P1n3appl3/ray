mod ray;
mod vec3;
use std::fs;

type Pixel = vec3::Vec3;
type Color = vec3::Vec3;

impl From<&Pixel> for String {
    fn from(item: &Pixel) -> Self {
        format!(
            "{} {} {}",
            (item.x * 255.99) as u8,
            (item.y * 255.99) as u8,
            (item.z * 255.99) as u8
        )
    }
}

struct Image {
    w: usize,
    h: usize,
    content: Vec<Vec<Pixel>>,
}

impl Image {
    fn new(w: usize, h: usize) -> Self {
        Self {
            w: w,
            h: h,
            content: vec![vec![Pixel::default(); w]; h],
        }
    }

    fn output_ppm(&self, filename: &str) {
        let header = format!("P3\n{} {}\n255\n", self.w, self.h);
        let content = self
            .content
            .iter()
            .map(|row| {
                row.iter()
                    .map(Into::into)
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        fs::write(filename, header + &content).expect("Unable to write to file");
    }
}

fn color(r: ray::Ray) -> Color {
    if hit_sphere(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_dir = r.dir.normalize();
    let t = (unit_dir.y + 1.0) / 2.0;
    vec3::Vec3::from_scalar(1.0 - t) * vec3::Vec3::from_scalar(1.0)
        + vec3::Vec3::from_scalar(t) * vec3::Vec3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: vec3::Vec3, radius: f32, r: ray::Ray) -> bool {
    let origin_to_center = r.origin - center;
    let a = r.dir.dot(r.dir);
    let b = 2.0 * origin_to_center.dot(r.dir);
    let c = origin_to_center.dot(origin_to_center) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn main() {
    let width = 200;
    let height = 100;
    let mut img = Image::new(width, height);

    let lower_left_corner = vec3::Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = vec3::Vec3::new(4.0, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, 2.0, 0.0);
    let origin = vec3::Vec3::new(0.0, 0.0, 0.0);

    for x in 0..width {
        for y in 0..height {
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;
            let r = ray::Ray::new(
                origin,
                lower_left_corner
                    + vec3::Vec3::from_scalar(u) * horizontal
                    + vec3::Vec3::from_scalar(v) * vertical,
            );
            img.content[y][x] = color(r);
        }
    }
    img.output_ppm("test.ppm");
}
