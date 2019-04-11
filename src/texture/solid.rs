use super::Texture;
use crate::scene::Color;
use crate::vec3::Vec3;

#[macro_export]
macro_rules! solid {
    ($x:expr) => {
        Solid::new($x.into())
    };
    ($x:expr, $y:expr, $z: expr) => {
        Solid::new(Color::new($x, $y, $z))
    };
}

#[derive(Debug, Clone)]
pub struct Solid {
    color: Color,
}

impl Solid {
    pub fn new(color: Vec3) -> Self {
        Solid { color }
    }
}

impl Texture for Solid {
    fn value(&self, _u: f32, _v: f32, _p: Vec3) -> Color {
        self.color
    }
}
