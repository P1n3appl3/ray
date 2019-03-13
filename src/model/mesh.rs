use super::aabb::AABB;
use super::bvh::BVHNode;
use super::hitable::{HitRecord, Hitable};
use super::triangle::Triangle;
use crate::ray::Ray;
use crate::scene::MatID;
use crate::vec3::Vec3;
use tobj;

#[derive(Debug)]
pub struct Mesh {
    pub faces: BVHNode,
}

impl Mesh {
    pub fn new(filename: &str, material: MatID) -> Self {
        let teapot = tobj::load_obj(&std::path::Path::new(filename));
        let (model, _) = teapot.unwrap();
        let mesh = &model[0].mesh;
        let points = mesh
            .positions
            .chunks(3)
            .map(|pos| Vec3::new(pos[0], pos[1], pos[2]))
            .collect::<Vec<_>>();
        let mut triangles = mesh
            .indices
            .chunks(3)
            .map(|i| {
                Box::new(Triangle::new(
                    points[i[0] as usize],
                    points[i[1] as usize],
                    points[i[2] as usize],
                    material,
                )) as Box<dyn Hitable>
            })
            .collect();
        Mesh {
            faces: BVHNode::from_items(&mut triangles),
        }
    }
}

impl Hitable for Mesh {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.faces.hit(r, t_min, t_max)
    }
    fn get_bb(&self) -> AABB {
        self.faces.get_bb()
    }
}
