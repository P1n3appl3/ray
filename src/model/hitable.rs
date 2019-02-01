use super::aabb::AABB;
use super::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// The relevant information for a ray collision with an object
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
    fn clone_box(&self) -> Box<Hitable>;
    fn get_mat(&self) -> Option<&dyn Material> {
        None
    }
    // TODO: just use an actuall aabb here since there's no infinite surfaces
    fn get_bb(&self) -> Option<AABB> {
        None
    }
}
