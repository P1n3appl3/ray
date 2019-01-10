use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        AABB { min: a, max: b }
    }

    pub fn default() -> Self {
        AABB {
            min: Vec3::from_scalar(std::f32::MAX),
            max: Vec3::from_scalar(std::f32::MIN),
        }
    }

    // TODO: implement Andrew Kensler's faster solution
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> bool {
        let temp_min = (self.min - r.origin) / r.dir;
        let temp_max = (self.min - r.origin) / r.dir;
        let t0 = temp_min
            .piecewise_min(temp_max)
            .piecewise_min(Vec3::from_scalar(t_min));
        let t1 = temp_min
            .piecewise_max(temp_max)
            .piecewise_max(Vec3::from_scalar(t_max));
        !(t1.x <= t0.x || t1.y <= t0.y || t1.z <= t0.z)
    }

    pub fn combine(&self, other: &Self) -> Self {
        AABB::new(
            self.min.piecewise_min(other.min),
            self.max.piecewise_max(other.max),
        )
    }
}
