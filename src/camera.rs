use super::ray::Ray;
use super::vec3::Vec3;
use rand::distributions::UnitCircle;
use rand::prelude::*;
use rand::random;

pub fn rand_in_unit_disk() -> Vec3 {
    let mut p;
    while {
        p = Vec3::new(
            random::<f32>() * 2.0 - 1.0,
            random::<f32>() * 2.0 - 1.0,
            0.0,
        );
        p.dot(&p) >= 1.0
    } {}
    p
}

pub fn almost_faster_rand() -> Vec3 {
    let r = UnitCircle::new().sample(&mut thread_rng());
    Vec3::new(r[0] as f32, r[1] as f32, 0) * random::<f32>().sqrt()
}

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    /// vfov is specified in degrees
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
    ) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);
        let focus_dist = (look_from - look_at).len();
        Camera {
            origin: look_from,
            lower_left: look_from - (u * half_width + v * half_height + w) * focus_dist,
            horizontal: u * (2.0 * half_width * focus_dist),
            vertical: v * (2.0 * half_height * focus_dist),
            lens_radius: aperture / 2.0,
            u,
            v,
        }
    }

    pub fn get_ray(&self, h: f32, v: f32) -> Ray {
        let rand = rand_in_unit_disk() * self.lens_radius;
        let offset = self.u * rand.x + self.v * rand.y;
        Ray::new(
            self.origin + offset,
            self.lower_left + self.horizontal * h + self.vertical * v
                - self.origin
                - offset,
        )
    }
}
