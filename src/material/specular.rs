use super::Material;
use crate::ray::Ray;
use crate::scene::Color;
use crate::vec3::Vec3;

#[macro_export]
macro_rules! specular {
    ($color:expr, $fuzz:expr) => {
        Arc::new(Specular::new($color.into(), $fuzz as f32))
    };
}

#[derive(Debug, Clone)]
pub struct Specular {
    albedo: Color,
    fuzz: f32,
}

impl Specular {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Specular { albedo, fuzz }
    }
}

impl Material for Specular {
    fn scatter(
        &self,
        r: Ray,
        normal: Vec3,
        point: Vec3,
        _u: f32,
        _v: f32,
    ) -> Option<(Color, Ray)> {
        let reflected = r.dir.normalize().reflect(&normal);
        let scattered =
            Ray::new(point, reflected + Vec3::rand_in_unit_sphere() * self.fuzz);
        if scattered.dir.dot(&normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
