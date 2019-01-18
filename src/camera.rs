use super::ray::Ray;
use super::vec3::Vec3;
use rand::random;

fn rand_in_unit_disk() -> Vec3 {
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
            lower_left: look_from
                - (u.scale(half_width) + v.scale(half_height) + w).scale(focus_dist),
            horizontal: u.scale(2.0 * half_width * focus_dist),
            vertical: v.scale(2.0 * half_height * focus_dist),
            lens_radius: aperture / 2.0,
            u: u,
            v: v,
        }
    }

    pub fn get_ray(&self, h: f32, v: f32) -> Ray {
        let rand = rand_in_unit_disk().scale(self.lens_radius);
        let offset = self.u.scale(rand.x) + self.v.scale(rand.y);
        Ray::new(
            self.origin + offset,
            self.lower_left + self.horizontal.scale(h) + self.vertical.scale(v)
                - self.origin
                - offset,
        )
    }
}
