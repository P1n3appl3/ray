pub mod dielectric;
pub mod diffuse;
pub mod isotropic;
pub mod light;
pub mod specular;

use crate::ray::Ray;
use crate::scene::Color;
use crate::vec3::Vec3;

pub trait Material: Send + Sync + std::fmt::Debug {
    /// Once a ray hits an object, it uses it's material to determine 2 things
    ///     1. How much light/color was absorbed by the surface (if any)
    ///     2. Which direction the ray bounces off (aka scatters)
    fn scatter(
        &self,
        r: Ray,
        normal: Vec3,
        point: Vec3,
        u: f32,
        v: f32,
    ) -> Option<(Color, Ray)>;
    /// Some materials could also be "emissive" meaning that they actively
    /// give off light instead of just reflecting/absorbing it
    fn emit(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        Vec3::zero()
    }
}
