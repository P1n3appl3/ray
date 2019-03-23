use crate::model::texture::*;
use crate::ray::Ray;
use crate::scene::Color;
use crate::vec3::Vec3;
use rand::random;

pub trait Material: Send + Sync + std::fmt::Debug {
    /// Once a ray hits an object, it uses it's material to determine 2 things
    ///     1. How much light/color was absorbed by the surface (if any)
    ///     2. Which direction the ray bounces off (aka scatters)
    fn scatter(
        &self,
        r: Ray,
        normal: Vec3,
        point: Vec3,
        u: f32,
        v: f32,
    ) -> Option<(Color, Ray)>;
    /// Some materials could also be "emissive" meaning that they actively
    /// give off light instead of just reflecting/absorbing it
    fn emit(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        Vec3::zero()
    }
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
    ) -> Option<(Color, Ray)> {
        let target = point + normal + Vec3::rand_in_unit_sphere();
        let scattered = Ray::new(point, target - point);
        Some((self.texture.value(u, v, point), scattered))
    }
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

/// Dielectrics have a specular component, meaning that they can sometimes
/// reflect rays instead of transmitting them. The only catch is that the
/// reflectivity vs refractivity is dependent on the angle of incomming light
/// (google Fresnel equations for the physics). Christophe Schlick figured out
/// an equation to approximate this effect. It returns a probability in [0..1]
/// of reflection occurring.
fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    tint: Color,
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(tint: Color, refractive_index: f32) -> Self {
        Dielectric {
            tint,
            refractive_index,
        }
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
    ) -> Option<(Color, Ray)> {
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
            None => (Vec3::zero(), 1.0),
        };
        Some((
            self.tint,
            Ray::new(
                point,
                if random::<f32>() < reflect_prob {
                    reflected
                } else {
                    refracted
                },
            ),
        ))
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
    fn scatter(
        &self,
        _r: Ray,
        _normal: Vec3,
        _point: Vec3,
        _u: f32,
        _v: f32,
    ) -> Option<(Color, Ray)> {
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
    ) -> Option<(Color, Ray)> {
        Some((
            self.texture.value(u, v, point),
            Ray::new(point, Vec3::rand_in_unit_sphere()),
        ))
    }
}
