use super::Material;
use crate::ray::Ray;
use crate::scene::Color;
use crate::texture::Texture;
use crate::vec3::Vec3;

#[macro_export]
macro_rules! diffuse {
    ($texture:expr) => {
        Arc::new(Diffuse::new($texture))
    };
}

#[derive(Debug)]
pub struct Diffuse<T: Texture> {
    texture: T,
}

impl<T: Texture> Diffuse<T> {
    pub fn new(texture: T) -> Self {
        Diffuse { texture }
    }
}

impl<T: Texture> Material for Diffuse<T> {
    fn scatter(
        &self,
        _r: Ray,
        normal: Vec3,
        point: Vec3,
        u: f32,
        v: f32,
    ) -> Option<(Color, Ray)> {
        let target = point + normal + Vec3::rand_in_unit_sphere();
        let scattered = Ray::new(point, target - point);
        Some((self.texture.value(u, v, point), scattered))
    }
}
