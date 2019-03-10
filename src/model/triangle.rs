use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    material: Box<dyn Material>,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, mat: Box<dyn Material>) -> Self {
        Triangle {
            v0,
            v1,
            v2,
            material: mat,
        }
    }
}

impl Hitable for Triangle {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        let normal = v0v1.cross(&v0v2);
        let area = normal.len() / 2.0;
        if normal.dot(&r.dir) <= std::f32::EPSILON {
            // ray is parallel to plane
            return None;
        }
        let d = normal.dot(&self.v0);
        let t = (normal.dot(&r.origin) + d) / normal.dot(&r.dir);
        if t < t_min || t > t_max {
            // ray starts or ends before the triangles plane
            return None;
        }

        let point = r.origin + r.dir * t;

        let edge0 = self.v1 - self.v0;
        let vp0 = point - self.v0;
        let cross = edge0.cross(&vp0);
        if normal.dot(&cross) < 0.0 {
            // point of intersection is outside this edge
            return None;
        }

        let edge1 = self.v2 - self.v1;
        let vp1 = point - self.v1;
        let cross = edge1.cross(&vp1);
        let u = normal.dot(&cross);
        if u < 0.0 {
            // point of intersection is outside this edge
            return None;
        }

        let edge2 = self.v0 - self.v2;
        let vp2 = point - self.v2;
        let cross = edge2.cross(&vp2);
        let v = normal.dot(&cross);
        if v < 0.0 {
            // point of intersection is outside this edge
            return None;
        }

        dbg!(u / area);
        dbg!(v / area);
        Some(HitRecord {
            t,
            u: u / area,
            v: v / area,
            point,
            normal, // TODO: interpolate this
            material: self.material.as_ref(),
        })
    }
    fn get_bb(&self) -> AABB {
        let min = self.v0.piecewise_min(self.v1).piecewise_min(self.v2);
        let max = self.v0.piecewise_max(self.v1).piecewise_max(self.v2);
        AABB::new(min, max)
    }
    fn get_mat(&self) -> Option<&dyn Material> {
        Some(self.material.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::aabb::AABB;
    use crate::model::material::Specular;
    use lazy_static::lazy_static;
    use std::f32::EPSILON;

    lazy_static! {
        static ref TRI: Triangle = Triangle::new(
            Vec3::new(0, 0, 0),
            Vec3::new(0, 4, 0),
            Vec3::new(4, 0, 0),
            Box::new(Specular::new(Vec3::default(), 0.0)),
        );
    }

    #[test]
    fn test_bb() {
        assert_eq!(
            TRI.get_bb(),
            AABB::new(Vec3::new(0, 0, 0), Vec3::new(4, 4, 0))
        );
    }
    #[test]
    fn test_hit() {
        let hit = TRI.hit(
            Ray::new(Vec3::new(1, 1, -2), Vec3::new(0, 0, 1)),
            0.0,
            std::f32::MAX,
        ).unwrap();
        assert!(hit.t - 1.0 <= EPSILON);
        assert_eq!(hit.normal, Vec3::new(0, 0, -1));
        dbg!(hit.u);
        dbg!(hit.v);
        panic!();
    }
    #[test]
    fn test_miss() {
        let hit = TRI.hit(
            Ray::new(Vec3::new(1, 1, -2), Vec3::new(0, 0, 1)),
            0.0,
            std::f32::MAX,
        );
        assert!(hit.is_none());
    }
}
