use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use crate::ray::Ray;

#[derive(Debug)]
pub struct BVHNode {
    pub bb: AABB,
    pub left: Option<Box<dyn Hitable>>,
    pub right: Option<Box<dyn Hitable>>,
}

impl BVHNode {
    pub fn new(
        bb: AABB,
        left: Option<Box<dyn Hitable>>,
        right: Option<Box<dyn Hitable>>,
    ) -> Self {
        BVHNode {
            bb: bb,
            left: left,
            right: right,
        }
    }
    pub fn from_items(items: &mut [Box<dyn Hitable>]) -> Self {
        if items.len() == 1 {
            return BVHNode::new(
                items[0].get_bb().unwrap(),
                Some(items[0].clone_box()),
                Some(items[0].clone_box()),
            );
        } else if items.len() == 2 {
            return BVHNode::new(
                items[0]
                    .get_bb()
                    .unwrap()
                    .combine(&items[1].get_bb().unwrap()),
                Some(items[0].clone_box()),
                Some(items[1].clone_box()),
            );
        }
        items.sort_unstable_by(|a, b| {
            // TODO: do something smarter than always choosing x axis (see site)
            a.get_bb()
                .unwrap()
                .min
                .x
                .partial_cmp(&b.get_bb().unwrap().min.x)
                .unwrap()
        });
        let half = items.len() / 2;
        // TODO: split while taking ownership instead of cloning
        let left = Box::new(BVHNode::from_items(
            &mut items[..half]
                .iter()
                .map(|x| x.clone_box())
                .collect::<Vec<Box<dyn Hitable>>>(),
        ));
        let right = Box::new(BVHNode::from_items(
            &mut items[half..]
                .iter()
                .map(|x| x.clone_box())
                .collect::<Vec<Box<dyn Hitable>>>(),
        ));
        BVHNode::new(left.bb.combine(&right.bb), Some(left), Some(right))
    }
}
impl Hitable for BVHNode {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bb.hit(r, t_min, t_max) {
            return None;
        }
        // both sides are always populated
        let hit_left = self.left.as_ref().unwrap().hit(r, t_min, t_max);
        let hit_right = self.right.as_ref().unwrap().hit(r, t_min, t_max);

        match (hit_left, hit_right) {
            (None, None) => None,
            (Some(hit), None) | (None, Some(hit)) => Some(hit),
            (Some(lhit), Some(rhit)) => {
                if lhit.t < rhit.t {
                    Some(lhit)
                } else {
                    Some(rhit)
                }
            }
        }
    }
    fn get_bb(&self) -> Option<AABB> {
        Some(self.bb)
    }
    fn get_mat(&self) -> Option<&Material> {
        None
    }
    fn clone_box(&self) -> Box<dyn Hitable> {
        unreachable!();
    }
}
