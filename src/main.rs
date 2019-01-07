mod hitable;
mod ray;
mod vec3;
use self::hitable::{HitableGroup, Sphere};
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

fn color(r: Ray, world: &impl hitable::Hitable) -> Color {
    if let Some(rec) = world.hit(r, 0.0, std::f32::MAX) {
        (Color::from_scalar(1.0) + rec.normal).scale(0.5)
    } else {
        let unit_dir = r.dir.normalize();
        let t = (unit_dir.y + 1.0) * 0.5;
        Color::from_scalar(1.0 - t) + Color::new(0.5, 0.7, 1.0).scale(t)
    }
}

fn main() {
    let width = 200;
    let height = 100;
    let mut img = Image::new(width, height);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let mut world = HitableGroup::default();
    world
        .items
        .push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world
        .items
        .push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

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
            img.content[y][x] = color(r, &world);
        }
    }
    img.output_ppm("test.ppm");
}
