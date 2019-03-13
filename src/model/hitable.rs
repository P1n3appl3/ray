use super::aabb::AABB;
use crate::ray::Ray;
use crate::scene::MatID;
use crate::vec3::Vec3;

/// The relevant information for a ray collision with an object
#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: MatID,
}

pub trait Hitable: std::fmt::Debug + Send + Sync {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn get_bb(&self) -> AABB;
}
