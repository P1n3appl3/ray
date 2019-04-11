use super::Material;
use crate::ray::Ray;
use crate::scene::Color;
use crate::texture::Texture;
use crate::vec3::Vec3;

#[macro_export]
macro_rules! isotropic {
    ($texture:expr) => {
        Arc::new(Isotropic::new($texture))
    };
}

#[derive(Debug)]
pub struct Isotropic<T: Texture> {
    texture: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(texture: T) -> Self {
        Isotropic { texture }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(
        &self,
        _r: Ray,
        _normal: Vec3,
        point: Vec3,
        u: f32,
        v: f32,
    ) -> Option<(Color, Ray)> {
        Some((
            self.texture.value(u, v, point),
            Ray::new(point, Vec3::rand_in_unit_sphere()),
        ))
    }
}
