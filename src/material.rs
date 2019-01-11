use super::hitable::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;
use rand::random;

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Diffuse(Vec3),
    Metal(Vec3, f32),
    Dielectric(f32),
}

fn rand_in_unit_sphere() -> Vec3 {
    let mut p;
    while {
        p = (Vec3::new(random(), random(), random()) - Vec3::from_scalar(1.0)).scale(2.0);
        p.square_len() > 1.0
    } {}
    p
}

fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub fn scatter(r: Ray, hit: HitRecord) -> Option<(Vec3, Ray)> {
    match hit.material {
        Material::Diffuse(color) => {
            let target = hit.point + hit.normal + rand_in_unit_sphere();
            Some((color, Ray::new(hit.point, target - hit.point)))
        }
        Material::Metal(reflectance, fuzz) => {
            let reflected = r.dir.normalize().reflect(&hit.normal);
            let scattered =
                Ray::new(hit.point, reflected + rand_in_unit_sphere().scale(fuzz));
            if scattered.dir.dot(&hit.normal) > 0.0 {
                Some((reflectance, scattered))
            } else {
                None
            }
        }
        Material::Dielectric(refractive_index) => {
            let attenuation = Vec3::from_scalar(1.0);
            let reflected = r.dir.reflect(&hit.normal);
            let outward_normal;
            let index_ratio;
            let cosine;
            if r.dir.dot(&hit.normal) > 0.0 {
                outward_normal = -hit.normal;
                index_ratio = refractive_index;
                cosine = refractive_index * r.dir.dot(&hit.normal) / r.dir.len();
            } else {
                outward_normal = hit.normal;
                index_ratio = 1.0 / refractive_index;
                cosine = -r.dir.dot(&hit.normal) / r.dir.len();
            };
            let (refracted, reflect_prob) =
                match r.dir.refract(&outward_normal, index_ratio) {
                    Some(x) => (x, schlick(cosine, refractive_index)),
                    // If none, the refracted ray is never used
                    None => (Vec3::default(), 1.0),
                };
            if random::<f32>() < reflect_prob {
                Some((attenuation, Ray::new(hit.point, reflected)))
            } else {
                Some((attenuation, Ray::new(hit.point, refracted)))
            }
        }
    }
}
