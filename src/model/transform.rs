use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
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
pub struct RotateY<T: Hitable> {
    obj: T,
    sin_theta: f32,
    cos_theta: f32,
    bb: AABB,
}

impl<T: Hitable> RotateY<T> {
    /// angle is in degrees
    pub fn new(obj: T, angle: f32) -> Self {
        let rad = angle * std::f32::consts::PI / 180.0;
        let mut temp = RotateY {
            obj,
            sin_theta: rad.sin(),
            cos_theta: rad.cos(),
            bb: AABB::default(),
        };
        iproduct!(0..2, 0..2, 0..2).for_each(|(i, j, k)| {
            let x = i as f32 * temp.bb.max.x + (1 - i) as f32 * temp.bb.min.x;
            let y = j as f32 * temp.bb.max.y + (1 - j) as f32 * temp.bb.min.y;
            let z = k as f32 * temp.bb.max.z + (1 - k) as f32 * temp.bb.min.z;
            let newx = temp.cos_theta * x + temp.sin_theta * z;
            let newz = -temp.sin_theta * x + temp.cos_theta * z;
            let v = Vec3::new(newx, y, newz);
            temp.bb = temp.bb.combine(&AABB::new(v, v));
        });
        temp
    }
}

impl<T: Hitable> Hitable for RotateY<T> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let rotated_r = Ray::new(
            Vec3::new(
                self.cos_theta * r.origin.x - self.sin_theta * r.origin.z,
                r.origin.y,
                self.sin_theta * r.origin.x + self.cos_theta * r.origin.z,
            ),
            Vec3::new(
                self.cos_theta * r.dir.x - self.sin_theta * r.dir.z,
                r.dir.y,
                self.sin_theta * r.dir.x + self.cos_theta * r.dir.z,
            ),
        );
        if let Some(mut rec) = self.obj.hit(rotated_r, t_min, t_max) {
            rec.point = Vec3::new(
                self.cos_theta * rec.point.x + self.sin_theta * rec.point.z,
                rec.point.y,
                -self.sin_theta * rec.point.x + self.cos_theta * rec.point.z,
            );
            rec.normal = Vec3::new(
                self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
                rec.normal.y,
                -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
            );
            Some(rec)
        } else {
            None
        }
    }
    fn get_bb(&self) -> AABB {
        self.bb
    }
}
