use super::material::{Material, Material::*};
use super::ray::Ray;
use super::vec3::Vec3;
use rand::random;

/// The relevant information for a ray collision with an object
pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait Hitable: Send + Sync {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn get_material(&self) -> Material;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(c: Vec3, r: f32, mat: Material) -> Self {
        Sphere {
            center: c,
            radius: r,
            material: mat,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin_center = r.origin - self.center;
        let a = r.dir.dot(r.dir);
        let b = 2.0 * origin_center.dot(r.dir);
        let c = origin_center.dot(origin_center) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / (2.0 * a);
            for _ in 0..2 {
                if temp < t_max && temp > t_min {
                    let p = r.point_at_param(temp);
                    return Some(HitRecord {
                        t: temp,
                        point: p,
                        normal: (p - self.center).scale(1.0 / self.radius),
                        material: self.get_material(),
                    });
                }
                // retry with the other quadratic formula solution
                temp = (-b + discriminant.sqrt()) / (2.0 * a);
            }
        }
        None
    }
    fn get_material(&self) -> Material {
        self.material
    }
}

#[derive(Default)]
pub struct HitableGroup {
    pub items: Vec<Box<Hitable>>,
}

impl HitableGroup {
    pub fn new(items: Vec<Box<Hitable>>) -> Self {
        HitableGroup { items: items }
    }
    pub fn random_scene() -> Self {
        let mut world = HitableGroup::new(vec![
            Box::new(Sphere::new(
                Vec3::new(0.0, -1000.0, 0.0),
                1000.0,
                Diffuse(Vec3::new(0.5, 0.5, 0.5)),
            )),
            Box::new(Sphere::new(
                Vec3::new(-4.0, 1.0, 0.0),
                1.0,
                Diffuse(Vec3::new(0.2, 0.3, 0.7)),
            )),
            Box::new(Sphere::new(
                Vec3::new(4.0, 1.0, 0.0),
                1.0,
                Metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
            )),
            Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Dielectric(1.5))),
        ]);

        for a in -11..11 {
            for b in -11..11 {
                let pos = Vec3::new(
                    a as f32 + 0.9 * random::<f32>(),
                    0.2,
                    b as f32 + 0.9 * random::<f32>(),
                );
                if (pos - Vec3::new(4.0, 0.2, 0.0)).len() < 0.9 {
                    continue;
                }
                world.items.push(Box::new(Sphere::new(
                    pos,
                    0.2,
                    match (random::<f32>() * 100.0) as u8 {
                        0...5 => Dielectric(1.5),
                        5...20 => Metal(
                            (Vec3::new(1.0, 1.0, 1.0)
                                + Vec3::new(random(), random(), random()))
                            .scale(0.5),
                            random::<f32>() / 2.0,
                        ),
                        _ => Diffuse(Vec3::new(
                            random::<f32>() * random::<f32>(),
                            random::<f32>() * random::<f32>(),
                            random::<f32>() * random::<f32>(),
                        )),
                    },
                )))
            }
        }

        world
    }
}

impl Hitable for HitableGroup {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.items
            .iter()
            .filter_map(|n| n.hit(r, t_min, t_max)) // heheheheheheheh
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap()) // because floating point
    }
    fn get_material(&self) -> Material {
        Material::default()
    }
}
