use crate::ray::Ray;
use crate::scene::Color;
use crate::texture::Texture;
use crate::vec3::Vec3;

pub trait Background: Send + Sync {
    fn get_color(&self, r: Ray) -> Color;
}

pub struct Gradient {
    pub a: Color,
    pub b: Color,
}

impl Background for Gradient {
    fn get_color(&self, r: Ray) -> Color {
        let t = (r.dir.normalize().y + 1.0) * 0.5;
        self.a * (1.0 - t) + self.b * t
    }
}

impl<T: Texture> Background for T {
    fn get_color(&self, r: Ray) -> Color {
        use std::f32::consts::PI;
        let v = r.dir.normalize();
        let phi = v.z.atan2(v.x);
        let theta = v.y.asin();
        self.value((1.0 + phi / PI) / 2.0, theta / PI + 0.5, Vec3::zero())
    }
}
