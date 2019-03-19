use super::aabb::AABB;
use super::bvh::BVHNode;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use super::rect::Rect;
use super::transform::FlipNormal;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

#[derive(Debug)]
pub struct Prism {
    faces: BVHNode,
}

impl Prism {
    pub fn new(p0: Vec3, p1: Vec3, mat: Arc<dyn Material>) -> Self {
        Prism {
            faces: BVHNode::from(&mut vec![
                Box::new(Rect::xy(p0.x, p0.y, p1.x, p1.y, p1.z, mat.clone()))
                    as Box<dyn Hitable>,
                Box::new(FlipNormal::new(Rect::xy(
                    p0.x,
                    p0.y,
                    p1.x,
                    p1.y,
                    p0.z,
                    mat.clone(),
                ))),
                Box::new(Rect::xz(p0.x, p0.z, p1.x, p1.z, p1.y, mat.clone())),
                Box::new(FlipNormal::new(Rect::xz(
                    p0.x,
                    p0.z,
                    p1.x,
                    p1.z,
                    p0.y,
                    mat.clone(),
                ))),
                Box::new(Rect::yz(p0.y, p0.z, p1.y, p1.z, p1.x, mat.clone())),
                Box::new(FlipNormal::new(Rect::yz(
                    p0.y,
                    p0.z,
                    p1.y,
                    p1.z,
                    p0.x,
                    mat.clone(),
                ))),
            ]),
        }
    }
}

impl Hitable for Prism {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.faces.hit(r, t_min, t_max)
    }
    fn get_bb(&self) -> AABB {
        self.faces.get_bb()
    }
}
