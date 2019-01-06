mod ray;
mod vec3;
use self::ray::Ray;
use self::vec3::Vec3;
use std::fs;

type Pixel = Vec3;
type Color = Vec3;

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

fn hit_sphere(center: Vec3, radius: f32, r: Ray) -> f32 {
    let origin_to_center = r.origin - center;
    let a = r.dir.dot(r.dir);
    let b = 2.0 * origin_to_center.dot(r.dir);
    let c = origin_to_center.dot(origin_to_center) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn color(r: Ray) -> Color {
    let sphere_pos = Vec3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(sphere_pos, 0.5, r);
    if t > 0.0 {
        let surface_normal = (r.point_at_param(t) - sphere_pos).normalize();
        return (Color::from_scalar(1.0) + surface_normal).scale(0.5);
    }
    let unit_dir = r.dir.normalize();
    let t = (unit_dir.y + 1.0) * 0.5;
    Color::from_scalar(1.0 - t) + Color::new(0.5, 0.7, 1.0).scale(t)
}

fn main() {
    let width = 200;
    let height = 100;
    let mut img = Image::new(width, height);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for y in 0..height {
        for x in 0..width {
            if !(width / 2 - 7 < x && width / 2 + 8 > x) {
                // continue;
            }
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal.scale(u) + vertical.scale(v),
            );
            img.content[y][x] = color(r);
        }
    }
    img.output_ppm("test.ppm");
}
