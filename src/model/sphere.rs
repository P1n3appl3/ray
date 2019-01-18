use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

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
            let mut temp = (-b - discriminant.sqrt()) / (2.0 * a);
            for _ in 0..2 {
                if temp < t_max && temp > t_min {
                    let p = r.point_at_param(temp);
                    return Some(HitRecord {
                        t: temp,
                        point: p,
                        normal: (p - self.center).scale(1.0 / self.radius),
                        material: &self.material,
                    });
                }
                // retry with the other quadratic formula solution
                temp = (-b + discriminant.sqrt()) / (2.0 * a);
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
    fn get_mat(&self) -> Option<&Box<dyn Material>> {
        Some(&self.material)
    }
    fn clone_box(&self) -> Box<dyn Hitable> {
        Box::new(self.clone())
    }
}
