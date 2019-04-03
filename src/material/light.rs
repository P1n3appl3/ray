use super::Material;
use crate::ray::Ray;
use crate::scene::Color;
use crate::texture::Texture;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Light<T: Texture> {
    texture: T,
}

impl<T: Texture> Light<T> {
    pub fn new(texture: T) -> Self {
        Light { texture }
    }
}

impl<T: Texture> Material for Light<T> {
    fn scatter(
        &self,
        _r: Ray,
        _normal: Vec3,
        _point: Vec3,
        _u: f32,
        _v: f32,
    ) -> Option<(Color, Ray)> {
        None
    }
    fn emit(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.texture.value(u, v, p)
    }
}
