use super::Texture;
use crate::scene::Color;
use crate::vec3::Vec3;

#[derive(Clone, Debug)]
pub struct SimpleGradient {}

impl Texture for SimpleGradient {
    fn value(&self, u: f32, v: f32, _p: Vec3) -> Color {
        Color::new(u, v, 1.0 - u - v)
    }
}
