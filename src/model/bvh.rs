use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
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
        BVHNode { bb, left, right }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bb.hit(r, t_min, t_max) {
            return None;
        }
        // left side is always populated
        let hit_left = self.left.as_ref().unwrap().hit(r, t_min, t_max);
        let hit_right = if self.right.is_some() {
            self.right.as_ref().unwrap().hit(r, t_min, t_max)
        } else {
            None
        };
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
    fn get_bb(&self) -> AABB {
        self.bb
    }
}

/// Recursively split the list of objects in two such that the sum of
/// surface area * number of contained objects is maximized
impl From<&mut Vec<Box<dyn Hitable>>> for BVHNode {
    fn from(objects: &mut Vec<Box<dyn Hitable>>) -> BVHNode {
        if objects.len() == 1 {
            return BVHNode::new(objects[0].get_bb(), Some(objects.remove(0)), None);
        } else if objects.len() == 2 {
            return BVHNode::new(
                objects[0].get_bb().combine(&objects[1].get_bb()),
                Some(objects.remove(0)),
                Some(objects.remove(0)),
            );
        }
        let main_box = objects
            .iter()
            .map(|i| i.get_bb())
            .fold(objects[0].get_bb(), |a, b| a.combine(&b));
        let x = main_box.max.x - main_box.min.x;
        let y = main_box.max.y - main_box.min.y;
        let z = main_box.max.z - main_box.min.z;
        if x > y && x > z {
            objects.sort_unstable_by(|a, b| {
                a.get_bb().min.x.partial_cmp(&b.get_bb().min.x).unwrap()
            });
        } else if y > x && y > z {
            objects.sort_unstable_by(|a, b| {
                a.get_bb().min.y.partial_cmp(&b.get_bb().min.y).unwrap()
            });
        } else {
            objects.sort_unstable_by(|a, b| {
                a.get_bb().min.z.partial_cmp(&b.get_bb().min.z).unwrap()
            });
        }
        objects.reverse();
        let mut left_area = vec![0.0; objects.len()];
        let mut right_area = vec![0.0; objects.len()];
        let boxes: Vec<AABB> = objects.iter().map(|i| i.get_bb()).collect();
        left_area[0] = boxes[0].surface_area();
        let mut left_box = boxes[0];
        for i in 1..objects.len() - 1 {
            left_box = left_box.combine(&boxes[i]);
            left_area[i] = left_box.surface_area();
        }
        right_area[0] = boxes[0].surface_area();
        let mut right_box = boxes[0];
        for i in (1..objects.len() - 1).rev() {
            right_box = right_box.combine(&boxes[i]);
            right_area[i] = right_box.surface_area();
        }
        let mut min_sah = std::f32::MAX;
        let mut min_sah_idx = 0;
        for i in 0..objects.len() - 1 {
            let sah = i as f32 * left_area[i]
                + (objects.len() - i - 1) as f32 * right_area[i + 1];
            if sah < min_sah {
                min_sah = sah;
                min_sah_idx = i;
            }
        }
        let left = Box::new(BVHNode::from(&mut objects.drain(..=min_sah_idx).collect()))
            as Box<Hitable>;
        let right = Box::new(BVHNode::from(objects)) as Box<Hitable>;
        BVHNode::new(main_box, Some(left), Some(right))
    }
}
