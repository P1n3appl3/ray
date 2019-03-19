use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use crate::axis::Axis;
use crate::ray::Ray;
use crate::vec3::Vec3;
use itertools::iproduct;

#[derive(Debug)]
pub struct FlipNormal<T: Hitable> {
    obj: T,
}

impl<T: Hitable> FlipNormal<T> {
    pub fn new(obj: T) -> Self {
        FlipNormal { obj }
    }
}

impl<T: Hitable> Hitable for FlipNormal<T> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut temp) = self.obj.hit(r, t_min, t_max) {
            temp.normal = -temp.normal;
            Some(temp)
        } else {
            None
        }
    }
    fn get_bb(&self) -> AABB {
        self.obj.get_bb()
    }
}

#[derive(Debug)]
pub struct Translate<T: Hitable> {
    obj: T,
    offset: Vec3,
}

impl<T: Hitable> Translate<T> {
    pub fn new(obj: T, offset: Vec3) -> Self {
        Translate { obj, offset }
    }
}

impl<T: Hitable> Hitable for Translate<T> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut moved_r = r;
        moved_r.origin -= self.offset;
        if let Some(mut temp) = self.obj.hit(moved_r, t_min, t_max) {
            temp.point += self.offset;
            Some(temp)
        } else {
            None
        }
    }
    fn get_bb(&self) -> AABB {
        let temp = self.obj.get_bb();
        AABB::new(temp.min + self.offset, temp.max + self.offset)
    }
}

#[derive(Debug)]
pub struct Rotate<T: Hitable> {
    obj: T,
    axis: Axis,
    sin_theta: f32,
    cos_theta: f32,
    bb: AABB,
}

impl<T: Hitable> Rotate<T> {
    /// angle is in degrees
    pub fn new(obj: T, axis: Axis, angle: f32) -> Self {
        let rad = angle * std::f32::consts::PI / 180.0;
        let mut temp = Rotate {
            obj,
            axis,
            sin_theta: rad.sin(),
            cos_theta: rad.cos(),
            bb: AABB::default(),
        };
        // go through every point in the bounding box and compute its rotated position
        // then form a new bounding box by expanding one to fit all those points
        iproduct!(0..2, 0..2, 0..2).for_each(|(i, j, k)| {
            let point = Vec3::new(
                i as f32 * temp.bb.max.x + (1 - i) as f32 * temp.bb.min.x,
                j as f32 * temp.bb.max.y + (1 - j) as f32 * temp.bb.min.y,
                k as f32 * temp.bb.max.z + (1 - k) as f32 * temp.bb.min.z,
            );
            let v = point.rotate(axis, temp.cos_theta, temp.sin_theta);
            temp.bb = temp.bb.combine(&AABB::new(v, v));
        });
        temp
    }
}

impl<T: Hitable> Hitable for Rotate<T> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let rotated_r = Ray::new(
            r.origin.rotate(self.axis, self.cos_theta, -self.sin_theta),
            r.dir.rotate(self.axis, self.cos_theta, -self.sin_theta),
        );
        if let Some(mut rec) = self.obj.hit(rotated_r, t_min, t_max) {
            rec.point = rec.point.rotate(self.axis, self.cos_theta, self.sin_theta);
            rec.normal = rec.normal.rotate(self.axis, self.cos_theta, self.sin_theta);
            Some(rec)
        } else {
            None
        }
    }
    fn get_bb(&self) -> AABB {
        self.bb
    }
}
