use super::aabb::AABB;
use super::bvh::BVHNode;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use crate::ray::Ray;

#[derive(Debug, Default)]
pub struct HitableGroup {
    pub items: Vec<Box<dyn Hitable>>,
}

impl HitableGroup {
    pub fn new(items: Vec<Box<dyn Hitable>>) -> Self {
        let mut temp = HitableGroup { items: items };
        // make sure that group has no elements without bounding boxes
        let mut items_with_bb: Vec<_> =
            temp.items.drain_filter(|n| n.get_bb().is_some()).collect();
        temp.items
            .insert(0, Box::new(BVHNode::from_items(&mut items_with_bb)));
        temp
    }
}

impl Hitable for HitableGroup {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.items
            .iter()
            .filter_map(|n| n.hit(r, t_min, t_max))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap()) // because floating point
    }
    fn get_bb(&self) -> Option<AABB> {
        if self.items.is_empty() {
            return None;
        }
        Some(
            self.items
                .iter()
                .filter_map(|group| group.get_bb())
                .fold(AABB::default(), |acc, bb| acc.combine(&bb)),
        )
    }
    fn get_mat(&self) -> Option<&Box<dyn Material>> {
        None
    }
    fn clone_box(&self) -> Box<dyn Hitable> {
        unreachable!();
    }
}
