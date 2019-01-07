use super::hitable::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;
use rand::random;

#[derive(Copy, Clone)]
pub enum Material {
    Diffuse(Vec3),
    Metal(Vec3),
}

impl Material {
    pub fn default() -> Self {
        Material::Diffuse(Vec3::new(0.5, 0.5, 0.5))
    }
}

fn rand_in_sphere() -> Vec3 {
    let mut p;
    while {
        p = (Vec3::new(random(), random(), random()) - Vec3::from_scalar(1.0)).scale(2.0);
        p.square_len() > 1.0
    } {}
    p
}

pub fn scatter(r: Ray, hit: HitRecord) -> Option<(Vec3, Ray)> {
    match hit.material {
        Material::Metal(reflectance) => {
            let reflected = r.dir.normalize().reflect(&hit.normal);
            let scattered = Ray::new(hit.point, reflected);
            if scattered.dir.dot(hit.normal) > 0.0 {
                Some((reflectance, scattered))
            } else {
                None
            }
        }
        Material::Diffuse(color) => {
            let target = hit.point + hit.normal + rand_in_sphere();
            Some((color, Ray::new(hit.point, target - hit.point)))
        }
    }
}
