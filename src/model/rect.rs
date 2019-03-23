use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use crate::axis::Axis;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

#[derive(Debug)]
pub struct Rect {
    axis_a: Axis,
    axis_b: Axis,
    a0: f32,
    a1: f32,
    b0: f32,
    b1: f32,
    k: f32,
    material: Arc<dyn Material>,
}

impl Rect {
    pub fn xy(
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        k: f32,
        material: Arc<dyn Material>,
    ) -> Self {
        Rect {
            axis_a: Axis::X,
            axis_b: Axis::Y,
            a0: x0,
            a1: x1,
            b0: y0,
            b1: y1,
            k,
            material,
        }
    }
    pub fn yz(
        y0: f32,
        z0: f32,
        y1: f32,
        z1: f32,
        k: f32,
        material: Arc<dyn Material>,
    ) -> Self {
        Rect {
            axis_a: Axis::Y,
            axis_b: Axis::Z,
            a0: y0,
            a1: y1,
            b0: z0,
            b1: z1,
            k,
            material,
        }
    }
    pub fn xz(
        x0: f32,
        z0: f32,
        x1: f32,
        z1: f32,
        k: f32,
        material: Arc<dyn Material>,
    ) -> Self {
        Rect {
            axis_a: Axis::X,
            axis_b: Axis::Z,
            a0: x0,
            a1: x1,
            b0: z0,
            b1: z1,
            k,
            material,
        }
    }
}

impl Hitable for Rect {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let other_axis = Axis::other(self.axis_a, self.axis_b);
        let t = (self.k - r.origin.get_axis(other_axis)) / r.dir.get_axis(other_axis);
        if t < t_min || t > t_max {
            return None;
        }
        let a = r.origin.get_axis(self.axis_a) + t * r.dir.get_axis(self.axis_a);
        let b = r.origin.get_axis(self.axis_b) + t * r.dir.get_axis(self.axis_b);
        if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
            return None;
        }
        Some(HitRecord {
            t,
            u: (a - self.a0) / (self.a1 - self.a0),
            v: (b - self.b0) / (self.b1 - self.b0),
            point: r.point_at_param(t),
            normal: Vec3::zero().set_axis(other_axis, 1.0),
            material: self.material.as_ref(),
        })
    }

    fn get_bb(&self) -> AABB {
        let other_axis = Axis::other(self.axis_a, self.axis_b);
        AABB::new(
            Vec3::zero()
                .set_axis(self.axis_a, self.a0)
                .set_axis(self.axis_b, self.b0)
                .set_axis(other_axis, self.k - 0.0001),
            Vec3::zero()
                .set_axis(self.axis_a, self.a1)
                .set_axis(self.axis_b, self.b1)
                .set_axis(other_axis, self.k + 0.0001),
        )
    }
}
