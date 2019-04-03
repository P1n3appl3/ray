use super::Material;
use crate::ray::Ray;
use crate::scene::Color;
use crate::vec3::Vec3;
use rand::random;

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
