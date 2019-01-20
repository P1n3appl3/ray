use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use crate::ray::Ray;

#[derive(Debug)]
pub struct FlipNormal {
    pub obj: Box<dyn Hitable>,
}

impl Clone for FlipNormal {
    fn clone(&self) -> FlipNormal {
        FlipNormal {
            obj: self.obj.clone_box(),
        }
    }
}

impl Hitable for FlipNormal {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut temp) = self.obj.hit(r, t_min, t_max) {
            temp.normal = -temp.normal;
            Some(temp)
        } else {
            None
        }
    }
    fn get_bb(&self) -> Option<AABB> {
        self.obj.get_bb()
    }
    fn get_mat(&self) -> Option<&Box<dyn Material>> {
        self.obj.get_mat()
    }
    fn clone_box(&self) -> Box<dyn Hitable> {
        Box::new(self.clone())
    }
}
