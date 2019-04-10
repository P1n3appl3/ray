use crate::ray::Ray;
use crate::vec3::Vec3;
use packed_simd::{f32x4, shuffle};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        AABB {
            min: a.piecewise_min(&b),
            max: a.piecewise_max(&b),
        }
    }

    pub fn default() -> Self {
        AABB::new(Vec3::from(std::f32::MAX), Vec3::from(std::f32::MIN))
    }

    // TODO: early escape is a potential optimization
    /// True if ray intersects AABB between t_min and t_max
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> bool {
        let temp0 = (self.min - r.origin) / r.dir;
        let temp1 = (self.max - r.origin) / r.dir;
        let t0 = temp0
            .piecewise_min(&temp1)
            .piecewise_max(&Vec3::from(t_min));
        let t1 = temp0
            .piecewise_max(&temp1)
            .piecewise_min(&Vec3::from(t_max));
        let t0 = f32x4::from(t0);
        let t1 = f32x4::new(t1.x, t1.y, t1.z, std::f32::MAX);
        !(t1.le(shuffle!(t0, [1, 2, 0, 3])).any()
            || t1.le(shuffle!(t0, [2, 0, 1, 3])).any())
    }

    /// Generates an enclosing AABB which encloses two others
    pub fn combine(&self, other: &Self) -> Self {
        AABB {
            min: self.min.piecewise_min(&other.min),
            max: self.max.piecewise_max(&other.max),
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

    #[test]
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
        let cube = AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(cube.surface_area(), 24.0);
        let oblong = AABB::new(Vec3::new(-1.0, -2.0, -3.0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(oblong.surface_area(), 88.0);
    }

    #[test]
    fn test_combine() {
        let cube = AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let offset_cube =
            AABB::new(Vec3::new(-2.0, -2.0, -2.0), Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(
            cube.combine(&offset_cube),
            AABB::new(Vec3::new(-2.0, -2.0, -2.0), Vec3::new(1.0, 1.0, 1.0))
        );
    }
}
