use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    /// vfov is specified in degrees
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
    ) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            origin: look_from,
            lower_left: look_from - u.scale(half_width) - v.scale(half_height) - w,
            horizontal: u.scale(2.0 * half_width),
            vertical: v.scale(2.0 * half_height),
        }
    }

    pub fn get_ray(&self, h: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + self.horizontal.scale(h) + self.vertical.scale(v)
                - self.origin,
        )
    }
}
