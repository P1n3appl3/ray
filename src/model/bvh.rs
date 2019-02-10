use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use crate::ray::Ray;

#[derive(Debug)]
pub struct BVHNode {
    pub bb: AABB,
    // TODO: consider if option is needed here since we're guarenteeing it's full
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
    pub fn from_items(mut items: &mut Vec<Box<dyn Hitable>>) -> Self {
        if items.len() == 1 {
            return BVHNode::new(items[0].get_bb(), Some(items.remove(0)), None);
        } else if items.len() == 2 {
            return BVHNode::new(
                items[0].get_bb().combine(&items[1].get_bb()),
                Some(items.remove(0)),
                Some(items.remove(0)),
            );
        }
        items.sort_unstable_by(|a, b| {
            // TODO: do something smarter than always choosing x axis (see site)
            a.get_bb().min.x.partial_cmp(&b.get_bb().min.x).unwrap()
        });
        let half = items.len() / 2;
        // TODO: split while taking ownership instead of cloning
        let left = Box::new(BVHNode::from_items(&mut items.drain(..half).collect()));
        let right = Box::new(BVHNode::from_items(&mut items));
        BVHNode::new(left.bb.combine(&right.bb), Some(left), Some(right))
    }
    pub fn from_items_sah(mut items: &mut Vec<Box<dyn Hitable>>) -> Self {
        let main_box = items
            .iter()
            .map(|i| i.get_bb())
            .fold(items[0].get_bb(), |a, b| a.combine(&b));
        let x = main_box.max.x - main_box.min.x;
        let y = main_box.max.y - main_box.min.y;
        let z = main_box.max.z - main_box.min.z;
        if x > y && x > z {
            items.sort_unstable_by(|a, b| {
                a.get_bb().min.x.partial_cmp(&b.get_bb().min.x).unwrap()
            });
        } else if y > x && y > z {
            items.sort_unstable_by(|a, b| {
                a.get_bb().min.y.partial_cmp(&b.get_bb().min.y).unwrap()
            });
        } else {
            items.sort_unstable_by(|a, b| {
                a.get_bb().min.z.partial_cmp(&b.get_bb().min.z).unwrap()
            });
        }
        let mut left_area = vec![0.0; items.len()];
        let mut right_area = vec![0.0; items.len()];
        let boxes: Vec<AABB> = items.iter().map(|i| i.get_bb()).collect();
        left_area[0] = boxes[0].surface_area();
        let mut left_box = boxes[0];
        for i in 1..items.len() - 1 {
            left_box = left_box.combine(&boxes[i]);
            left_area[i] = left_box.surface_area();
        }
        right_area[0] = boxes[0].surface_area();
        let mut right_box = boxes[0];
        for i in (1..items.len() - 1).rev() {
            right_box = right_box.combine(&boxes[i]);
            right_area[i] = right_box.surface_area();
        }
        let mut min_sah = std::f32::MAX;
        let mut min_sah_idx = 0;
        for i in 0..items.len() - 1 {
            let sah = i as f32 * left_area[i]
                + (items.len() - i - 1) as f32 * right_area[i + 1];
            if sah < min_sah {
                min_sah = sah;
                min_sah_idx = i;
            }
        }
        let left = if min_sah_idx == 0 {
            items.remove(0)
        } else {
            (Box::new(BVHNode::from_items_sah(
                &mut items.drain(..min_sah_idx + 1).collect(),
            )) as Box<Hitable>)
        };
        let right = if min_sah_idx == items.len() - 2 {
            items.remove(min_sah_idx + 1)
        } else {
            Box::new(BVHNode::from_items_sah(&mut items)) as Box<Hitable>
        };
        BVHNode::new(main_box, Some(left), Some(right))
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
    fn get_bb(&self) -> AABB {
        self.bb
    }
}
