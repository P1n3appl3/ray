use crate::model::texture::Texture;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::random;

pub trait Material: Send + Sync + std::fmt::Debug {
    fn scatter(
        &self,
        r: Ray,
        normal: Vec3,
        point: Vec3,
        u: f32,
        v: f32,
    ) -> Option<(Vec3, Ray)>;
    fn emit(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        Vec3::default()
    }
}

/// Aproximates the way that glass reflectivity varies with viewing angle
fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[derive(Debug)]
pub struct Diffuse<T: Texture> {
    texture: T,
}

impl<T: Texture> Diffuse<T> {
    pub fn new(texture: T) -> Self {
        Diffuse { texture }
    }
}

impl<T: Texture> Material for Diffuse<T> {
    fn scatter(
        &self,
        _r: Ray,
        normal: Vec3,
        point: Vec3,
        u: f32,
        v: f32,
    ) -> Option<(Vec3, Ray)> {
        let target = point + normal + Vec3::rand_in_unit_sphere();
        let scattered = Ray::new(point, target - point);
        Some((self.texture.value(u, v, point), scattered))
    }
}

#[derive(Debug, Clone)]
pub struct Specular {
    albedo: Vec3,
    fuzz: f32,
}

impl Specular {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
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
    ) -> Option<(Vec3, Ray)> {
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

#[derive(Debug, Clone)]
pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Dielectric { refractive_index }
    }
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
        let attenuation = Vec3::from(1.0);
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
            // TODO: maybe adjust to make it more reflective
            Some((attenuation, Ray::new(point, reflected)))
        } else {
            Some((attenuation, Ray::new(point, refracted)))
        }
    }
}

#[derive(Debug)]
pub struct Light<T: Texture> {
    texture: T,
}

impl<T: Texture> Light<T> {
    pub fn new(texture: T) -> Self {
        Light { texture }
    }
}

impl<T: Texture> Material for Light<T> {
    fn scatter(&self, _: Ray, _: Vec3, _: Vec3, _: f32, _: f32) -> Option<(Vec3, Ray)> {
        None
    }
    fn emit(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.texture.value(u, v, p)
    }
}

#[derive(Debug)]
pub struct Isotropic<T: Texture> {
    texture: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(texture: T) -> Self {
        Isotropic { texture }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(
        &self,
        _r: Ray,
        _normal: Vec3,
        point: Vec3,
        u: f32,
        v: f32,
    ) -> Option<(Vec3, Ray)> {
        Some((
            self.texture.value(u, v, point),
            Ray::new(point, Vec3::rand_in_unit_sphere()),
        ))
    }
}
