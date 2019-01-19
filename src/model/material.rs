use crate::model::texture::Texture;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::random;

pub trait Material: Send + Sync + std::fmt::Debug {
    fn clone_box(&self) -> Box<dyn Material>;
    fn scatter(
        &self,
        r: Ray,
        normal: Vec3,
        point: Vec3,
        u: f32,
        v: f32,
    ) -> Option<(Vec3, Ray)>;
}

pub fn rand_in_unit_sphere() -> Vec3 {
    let mut p;
    while {
        p = Vec3::rand().scale(2.0) - Vec3::from_scalar(1.0);
        p.square_len() > 1.0
    } {}
    p
}

/// Aproximates the way that glass reflectivity varies with viewing angle
fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[derive(Debug)]
pub struct Diffuse {
    pub texture: Box<dyn Texture>,
}

impl Material for Diffuse {
    fn scatter(
        &self,
        _r: Ray,
        normal: Vec3,
        point: Vec3,
        u: f32,
        v: f32,
    ) -> Option<(Vec3, Ray)> {
        let target = point + normal + rand_in_unit_sphere();
        let new_ray = Ray::new(point, target - point);
        Some((self.texture.value(u, v, point), new_ray))
    }
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(Diffuse {
            texture: self.texture.clone_box(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(
        &self,
        r: Ray,
        normal: Vec3,
        point: Vec3,
        _u: f32,
        _v: f32,
    ) -> Option<(Vec3, Ray)> {
        let reflected = r.dir.normalize().reflect(&normal);
        let scattered =
            Ray::new(point, reflected + rand_in_unit_sphere().scale(self.fuzz));
        if scattered.dir.dot(&normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub refractive_index: f32,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r: Ray,
        normal: Vec3,
        point: Vec3,
        _u: f32,
        _v: f32,
    ) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::from_scalar(1.0);
        let reflected = r.dir.reflect(&normal);
        let outward_normal;
        let index_ratio;
        let cosine;
        if r.dir.dot(&normal) > 0.0 {
            outward_normal = -normal;
            index_ratio = self.refractive_index;
            cosine = self.refractive_index * r.dir.dot(&normal) / r.dir.len();
        } else {
            outward_normal = normal;
            index_ratio = 1.0 / self.refractive_index;
            cosine = -r.dir.dot(&normal) / r.dir.len();
        };
        let (refracted, reflect_prob) = match r.dir.refract(&outward_normal, index_ratio)
        {
            Some(x) => (x, schlick(cosine, self.refractive_index)),
            // If none, the refracted ray is never used
            None => (Vec3::default(), 1.0),
        };
        if random::<f32>() < reflect_prob {
            // TODO: figure out why i'm not seeing reflections in glass
            Some((attenuation, Ray::new(point, reflected)))
        } else {
            Some((attenuation, Ray::new(point, refracted)))
        }
    }
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
