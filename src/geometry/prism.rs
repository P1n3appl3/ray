use super::aabb::AABB;
use super::rect::Rect;
use super::transform::FlipNormal;
use super::{HitRecord, Hitable};
use crate::bvh::BVHNode;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

#[macro_export]
macro_rules! prism {
    ($p0:expr, $p1:expr, $mat:expr) => {
        Box::new(Prism::new($p0.into(), $p1.into(), $mat))
    };
}

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
                Box::new(FlipNormal::new(Rect::yz(p0.y, p0.z, p1.y, p1.z, p0.x, mat))),
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
