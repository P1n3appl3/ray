use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::random;
use std::sync::Arc;

#[derive(Debug)]
pub struct Volume {
    density: f32,
    boundary: Box<dyn Hitable>,
    phase_function: Arc<dyn Material>,
}

impl Volume {
    pub fn new(
        density: f32,
        boundary: Box<dyn Hitable>,
        phase_function: Arc<dyn Material>,
    ) -> Self {
        Volume {
            density,
            boundary,
            phase_function,
        }
    }
}

impl Hitable for Volume {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut hit1) = self.boundary.hit(r, std::f32::MIN, std::f32::MAX) {
            if let Some(mut hit2) = self.boundary.hit(r, hit1.t + 0.0001, std::f32::MAX) {
                hit1.t = hit1.t.max(t_min);
                hit2.t = hit2.t.min(t_max);
                if hit1.t > hit2.t {
                    return None;
                }
                hit1.t = hit1.t.max(0.0);
                let dist_inside_boundary = (hit2.t - hit1.t) * r.dir.len();
                let hit_dist = -random::<f32>().log10() / self.density;
                if hit_dist < dist_inside_boundary {
                    return Some(HitRecord {
                        t: hit1.t + hit_dist / r.dir.len(),
                        u: 0.0, // arbitrary
                        v: 0.0, // arbitrary
                        point: r.point_at_param(hit1.t),
                        normal: Vec3::default(), // arbitrary
                        material: self.phase_function.as_ref(),
                    });
                }
            }
        }
        None
    }
    fn get_bb(&self) -> AABB {
        self.boundary.get_bb()
    }
}
