use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        AABB {
            min: a.piecewise_min(b),
            max: a.piecewise_max(b),
        }
    }

    pub fn default() -> Self {
        AABB::new(
            Vec3::from_scalar(std::f32::MAX),
            Vec3::from_scalar(std::f32::MIN),
        )
    }

    // TODO: early escape is a potential optimization
    /// True if ray intersects AABB between t_min and t_max
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> bool {
        let temp0 = (self.min - r.origin) / r.dir;
        let temp1 = (self.max - r.origin) / r.dir;
        let t0 = temp0
            .piecewise_min(temp1)
            .piecewise_max(Vec3::from_scalar(t_min));
        let t1 = temp0
            .piecewise_max(temp1)
            .piecewise_min(Vec3::from_scalar(t_max));
        !(t1.x <= t0.y
            || t1.x <= t0.z
            || t1.y <= t0.x
            || t1.y <= t0.z
            || t1.z <= t0.x
            || t1.z <= t0.y)
    }

    /// Generates an enclosing AABB which encloses two others
    pub fn combine(&self, other: &Self) -> Self {
        AABB {
            min: self.min.piecewise_min(other.min),
            max: self.max.piecewise_max(other.max),
        }
    }

    pub fn surface_area(&self) -> f32 {
        let len = self.max.z - self.min.z;
        let width = self.max.x - self.min.x;
        let height = self.max.y - self.min.y;
        2.0 * len * width + 2.0 * len * height + 2.0 * width * height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::MAX;
    #[test]
    fn test_hit() {
        let r = Ray::new(Vec3::new(0.0, 0.0, -10.0), Vec3::new(0.0, 0.0, 1.0));
        let on_origin = AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        assert!(on_origin.hit(r, 0.0, MAX));
    }

    fn test_miss() {
        let off_origin = AABB::new(Vec3::new(2.0, 3.0, 4.0), Vec3::new(3.0, 4.0, 5.0));
        let r = Ray::new(Vec3::new(0.0, 0.0, -10.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(!off_origin.hit(r, 0.0, MAX));
    }

    #[test]
    fn test_time() {
        let r = Ray::new(Vec3::new(0.0, 0.0, -10.0), Vec3::new(0.0, 0.0, 1.0));
        let on_origin = AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        assert!(!on_origin.hit(r, 0.0, 5.0));
        assert!(on_origin.hit(r, 0.0, 15.0));
    }

    #[test]
    fn test_surface_area() {
        let unit = AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(unit.surface_area(), 24.0);
        let oblong = AABB::new(Vec3::new(-1.0, -2.0, -3.0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(oblong.surface_area(), 88.0);
    }
}
