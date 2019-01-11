use super::aabb::AABB;
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;
use rand::random;

/// The relevant information for a ray collision with an object
#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait Hitable: Send + Sync + std::fmt::Debug {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn clone_box(&self) -> Box<Hitable>;
    fn get_bounding_box(&self) -> Option<AABB>;
    fn get_material(&self) -> Material;
}

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(c: Vec3, r: f32, mat: Material) -> Self {
        Sphere {
            center: c,
            radius: r,
            material: mat,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // println!("lmao");
        let origin_center = r.origin - self.center;
        let a = r.dir.dot(&r.dir);
        let b = 2.0 * origin_center.dot(&r.dir);
        let c = origin_center.dot(&origin_center) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / (2.0 * a);
            for _ in 0..2 {
                if temp < t_max && temp > t_min {
                    let p = r.point_at_param(temp);
                    return Some(HitRecord {
                        t: temp,
                        point: p,
                        normal: (p - self.center).scale(1.0 / self.radius),
                        material: self.get_material(),
                    });
                }
                // retry with the other quadratic formula solution
                temp = (-b + discriminant.sqrt()) / (2.0 * a);
            }
        }
        None
    }
    fn clone_box(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
    fn get_bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::from_scalar(self.radius),
            self.center + Vec3::from_scalar(self.radius),
        ))
    }
    fn get_material(&self) -> Material {
        self.material
    }
}

#[derive(Default, Debug)]
pub struct HitableGroup {
    pub items: Vec<Box<Hitable>>,
}

impl HitableGroup {
    pub fn new(items: Vec<Box<Hitable>>) -> Self {
        let mut temp = HitableGroup { items: items };
        // make sure that group has no elements without bounding boxes
        let mut items_with_bb: Vec<_> = temp
            .items
            .drain_filter(|n| n.get_bounding_box().is_some())
            .collect();
        temp.items
            .insert(0, Box::new(BVHNode::from_items(&mut items_with_bb)));
        println!("{}", temp.items.len());
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
    fn get_bounding_box(&self) -> Option<AABB> {
        if self.items.is_empty() {
            return None;
        }
        Some(
            self.items
                .iter()
                .filter_map(|group| group.get_bounding_box())
                .fold(AABB::default(), |acc, bb| acc.combine(&bb)),
        )
    }
    fn clone_box(&self) -> Box<Hitable> {
        unreachable!();
    }
    fn get_material(&self) -> Material {
        unreachable!();
    }
}

#[derive(Debug)]
pub struct BVHNode {
    pub bb: AABB,
    pub left: Option<Box<Hitable>>,
    pub right: Option<Box<Hitable>>,
}

impl BVHNode {
    pub fn new(
        bb: AABB,
        left: Option<Box<Hitable>>,
        right: Option<Box<Hitable>>,
    ) -> Self {
        BVHNode {
            bb: bb,
            left: left,
            right: right,
        }
    }
    pub fn from_items(items: &mut [Box<Hitable>]) -> Self {
        if items.len() == 1 {
            return BVHNode::new(
                items[0].get_bounding_box().unwrap(),
                Some(items[0].clone_box()),
                Some(items[0].clone_box()),
            );
        } else if items.len() == 2 {
            return BVHNode::new(
                items[0]
                    .get_bounding_box()
                    .unwrap()
                    .combine(&items[1].get_bounding_box().unwrap()),
                Some(items[0].clone_box()),
                Some(items[1].clone_box()),
            );
        }
        items.sort_unstable_by(|a, b| {
            // TODO: do something smarter than always choosing x axis
            a.get_bounding_box()
                .unwrap()
                .min
                .x
                .partial_cmp(&b.get_bounding_box().unwrap().min.x)
                .unwrap()
        });
        let half = items.len() / 2;
        let left = Box::new(BVHNode::from_items(
            &mut items[..half]
                .iter()
                .map(|x| x.clone_box())
                .collect::<Vec<Box<Hitable>>>(),
        ));
        let right = Box::new(BVHNode::from_items(
            &mut items[half..]
                .iter()
                .map(|x| x.clone_box())
                .collect::<Vec<Box<Hitable>>>(),
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
    fn clone_box(&self) -> Box<Hitable> {
        unreachable!();
    }
    fn get_bounding_box(&self) -> Option<AABB> {
        Some(self.bb)
    }
    fn get_material(&self) -> Material {
        unreachable!();
    }
}
