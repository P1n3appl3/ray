use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(c: Vec3, r: f32) -> Self {
        Sphere {
            center: c,
            radius: r,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin_center = r.origin - self.center;
        let a = r.dir.dot(r.dir);
        let b = 2.0 * origin_center.dot(r.dir);
        let c = origin_center.dot(origin_center) - self.radius * self.radius;
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
                    });
                }
                // retry with the other quadratic formula solution
                temp = (-b + discriminant.sqrt()) / (2.0 * a);
            }
        }
        None
    }
}

#[derive(Default)]
pub struct HitableGroup {
    pub items: Vec<Box<Hitable>>,
}

impl Hitable for HitableGroup {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.items
            .iter()
            .filter_map(|n| n.hit(r, t_min, t_max)) // heheheheheheheh
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap()) // because floating point
    }
}
