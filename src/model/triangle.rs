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
    /*
    Solving ray triangle intersection with Möller-Trumbore algorithm:

    Triangle verticies A B and C
    Triangle edges E1 = B - A and E2 = C - A
    Ray origin O and direction D
    Barycentric coords u and v express P = (1-u-v)A * uB * vC
    Relative position of ray origin to vertex A is called T = (O-A)
    Solving the system using Cramers rule:

    t        1      |T E1 E2|        (note that scalar tripple product
    u  = ---------  |D T  E2|         |A B C| is equal to AxB.C and
    v    |D E1 E2|  |D E1 T |         also A.BxC)

    if 0 <= u <= 1 and 0 <= u + v <= 1 then the collision is valid
        */
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let d_cross_e2 = r.dir.cross(&edge2);
        let det = edge1.dot(&d_cross_e2);
        if det.abs() <= std::f32::EPSILON {
            // ray is parallel to plane
            return None;
        }
        if det <= 0.0 {
            // intersection is with the back-face of the triangle
            return None;
        }
        let tvec = r.origin - self.v0;
        let u = tvec.dot(&d_cross_e2) / det;
        if u <= 0.0 || u >= 1.0 {
            return None;
        }
        let t_cross_e1 = tvec.cross(&edge1);
        let v = r.dir.dot(&t_cross_e1) / det;
        if v <= 0.0 || u + v >= 1.0 {
            return None;
        }
        let t = edge2.dot(&t_cross_e1) / det;
        if t < t_min || t > t_max {
            return None;
        }
        Some(HitRecord {
            t,
            u,
            v,
            // TODO: figure out if I can safely remove (1-u-v) below
            point: self.v0 * (1.0 - u - v) + edge1 * u + edge2 * v,
            normal: edge1.cross(&edge2).normalize(), // TODO: interpolate this
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
        let hit = TRI
            .hit(
                Ray::new(Vec3::new(1, 1, -2), Vec3::new(0, 0, 1)),
                0.0,
                std::f32::MAX,
            )
            .unwrap();
        assert_eq!(hit.t, 2.0);
        assert_eq!(hit.normal, Vec3::new(0, 0, -1));
        assert_eq!(hit.point, Vec3::new(1, 1, 0));
        assert_eq!(hit.u, 0.25);
        assert_eq!(hit.v, 0.25);
        let hit = TRI
            .hit(
                Ray::new(Vec3::new(1, 0, -1), Vec3::new(0, 1, 1)),
                0.0,
                std::f32::MAX,
            )
            .unwrap();
        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.normal, Vec3::new(0, 0, -1));
        assert_eq!(hit.point, Vec3::new(1, 1, 0));
        assert_eq!(hit.u, 0.25);
        assert_eq!(hit.v, 0.25);
    }
    #[test]
    fn test_back_hit() {
        assert!(TRI
            .hit(
                Ray::new(Vec3::new(1, 1, 2), Vec3::new(0, 0, -1)),
                0.0,
                std::f32::MAX,
            )
            .is_none());
    }
    #[test]
    fn test_miss() {
        let hit = TRI.hit(
            Ray::new(Vec3::new(1, 1, 2), Vec3::new(0, 0, -1)),
            0.0,
            std::f32::MAX,
        );
        assert!(hit.is_none());
    }
}
