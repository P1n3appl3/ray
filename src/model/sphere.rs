use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(c: Vec3, r: f32, mat: Box<dyn Material>) -> Self {
        Sphere {
            center: c,
            radius: r,
            material: mat,
        }
    }
    pub fn get_uv(&self, p: Vec3) -> (f32, f32) {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        (1.0 - (phi + PI) / (2.0 * PI), (theta + PI / 2.0) / PI)
    }
}

impl Clone for Sphere {
    fn clone(&self) -> Sphere {
        Sphere {
            center: self.center,
            radius: self.radius,
            material: self.material.clone_box(),
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin_center = r.origin - self.center;
        let a = r.dir.dot(&r.dir);
        let b = 2.0 * origin_center.dot(&r.dir);
        let c = origin_center.dot(&origin_center) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let mut t = (-b - discriminant.sqrt()) / (2.0 * a);
            for _ in 0..2 {
                if t < t_max && t > t_min {
                    let point = r.point_at_param(t);
                    let normal = (point - self.center) / self.radius;
                    let (u, v) = self.get_uv(normal);
                    return Some(HitRecord {
                        t,
                        u,
                        v,
                        point,
                        normal,
                        material: &*self.material,
                    });
                }
                // retry with the other quadratic formula solution
                t = (-b + discriminant.sqrt()) / (2.0 * a);
            }
        }
        None
    }
    fn get_bb(&self) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::from_scalar(self.radius),
            self.center + Vec3::from_scalar(self.radius),
        ))
    }
    fn get_mat(&self) -> Option<&dyn Material> {
        Some(&*self.material)
    }
    fn clone_box(&self) -> Box<dyn Hitable> {
        Box::new(self.clone())
    }
}
