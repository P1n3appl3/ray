pub mod checker;
pub mod gradient;
pub mod image;
pub mod perlin;
pub mod solid;

use crate::scene::Color;
use crate::vec3::Vec3;

pub trait Texture: Send + Sync + std::fmt::Debug {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Color;
}
