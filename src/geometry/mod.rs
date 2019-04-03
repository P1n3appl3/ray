pub mod aabb;
pub mod mesh;
pub mod prism;
pub mod rect;
pub mod sphere;
pub mod transform;
pub mod volume;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use aabb::AABB;

/// The relevant geometric information for a ray collision with an object
#[derive(Clone)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub trait Hitable: std::fmt::Debug + Send + Sync {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn get_bb(&self) -> AABB;
}
