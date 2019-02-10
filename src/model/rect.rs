use super::aabb::AABB;
use super::bvh::BVHNode;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use super::transform::FlipNormal;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct XYRect {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: Box<dyn Material>,
}

impl XYRect {
    pub fn new(
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        k: f32,
        mat: Box<dyn Material>,
    ) -> Self {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            material: mat,
        }
    }
}

impl Clone for XYRect {
    fn clone(&self) -> XYRect {
        XYRect {
            x0: self.x0,
            x1: self.x1,
            y0: self.y0,
            y1: self.y1,
            k: self.k,
            material: self.material.clone_box(),
        }
    }
}

impl Hitable for XYRect {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.z) / r.dir.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin.x + t * r.dir.x;
        let y = r.origin.y + t * r.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        Some(HitRecord {
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            point: r.point_at_param(t),
            normal: Vec3::new(0.0, 0.0, 1.0),
            material: &*self.material,
        })
    }
    fn get_bb(&self) -> AABB {
        AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        )
    }
    fn get_mat(&self) -> Option<&dyn Material> {
        Some(&*self.material)
    }
}

#[derive(Debug)]
pub struct XZRect {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Box<dyn Material>,
}

impl XZRect {
    pub fn new(
        x0: f32,
        z0: f32,
        x1: f32,
        z1: f32,
        k: f32,
        mat: Box<dyn Material>,
    ) -> Self {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            material: mat,
        }
    }
}

impl Clone for XZRect {
    fn clone(&self) -> XZRect {
        XZRect {
            x0: self.x0,
            x1: self.x1,
            z0: self.z0,
            z1: self.z1,
            k: self.k,
            material: self.material.clone_box(),
        }
    }
}

impl Hitable for XZRect {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.y) / r.dir.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin.x + t * r.dir.x;
        let z = r.origin.z + t * r.dir.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord {
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            point: r.point_at_param(t),
            normal: Vec3::new(0.0, 1.0, 0.0),
            material: &*self.material,
        })
    }
    fn get_bb(&self) -> AABB {
        AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        )
    }
    fn get_mat(&self) -> Option<&dyn Material> {
        Some(&*self.material)
    }
}

#[derive(Debug)]
pub struct YZRect {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Box<dyn Material>,
}

impl YZRect {
    pub fn new(
        y0: f32,
        z0: f32,
        y1: f32,
        z1: f32,
        k: f32,
        mat: Box<dyn Material>,
    ) -> Self {
        YZRect {
            y0,
            y1,
            z0,
            z1,
            k,
            material: mat,
        }
    }
}

impl Clone for YZRect {
    fn clone(&self) -> YZRect {
        YZRect {
            y0: self.y0,
            y1: self.y1,
            z0: self.z0,
            z1: self.z1,
            k: self.k,
            material: self.material.clone_box(),
        }
    }
}

impl Hitable for YZRect {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.x) / r.dir.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.origin.y + t * r.dir.y;
        let z = r.origin.z + t * r.dir.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord {
            t,
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            point: r.point_at_param(t),
            normal: Vec3::new(1.0, 0.0, 0.0),
            material: &*self.material,
        })
    }
    fn get_bb(&self) -> AABB {
        AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        )
    }
    fn get_mat(&self) -> Option<&dyn Material> {
        Some(&*self.material)
    }
}

#[derive(Debug)]
pub struct Prism {
    faces: BVHNode,
}

impl Prism {
    pub fn new(p0: Vec3, p1: Vec3, mat: Box<dyn Material>) -> Self {
        Prism {
            faces: BVHNode::from_items(&mut vec![
                Box::new(XYRect::new(p0.x, p0.y, p1.x, p1.y, p1.z, mat.clone_box()))
                    as Box<dyn Hitable>,
                Box::new(FlipNormal::new(Box::new(XYRect::new(
                    p0.x,
                    p0.y,
                    p1.x,
                    p1.y,
                    p0.z,
                    mat.clone_box(),
                )))),
                Box::new(XZRect::new(p0.x, p0.z, p1.x, p1.z, p1.y, mat.clone_box())),
                Box::new(FlipNormal::new(Box::new(XZRect::new(
                    p0.x,
                    p0.z,
                    p1.x,
                    p1.z,
                    p0.y,
                    mat.clone_box(),
                )))),
                Box::new(YZRect::new(p0.y, p0.z, p1.y, p1.z, p1.x, mat.clone_box())),
                Box::new(FlipNormal::new(Box::new(YZRect::new(
                    p0.y,
                    p0.z,
                    p1.y,
                    p1.z,
                    p0.x,
                    mat.clone_box(),
                )))),
            ]),
        }
    }
}

impl Hitable for Prism {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.faces.hit(r, t_min, t_max)
    }
    fn get_bb(&self) -> AABB {
        self.faces.get_bb()
    }
    fn get_mat(&self) -> Option<&dyn Material> {
        None
    }
}
