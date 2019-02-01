use crate::ray::Ray;
use crate::vec3::Vec3;

type Color = Vec3;

pub trait Background: Send + Sync {
    fn get_color(&self, r: Ray) -> Color;
}

// TODO: add environment map (cube or sphere)

pub struct Solid {
    pub color: Color,
}

impl Background for Solid {
    fn get_color(&self, _r: Ray) -> Color {
        self.color
    }
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
