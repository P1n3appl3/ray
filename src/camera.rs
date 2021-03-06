use super::ray::Ray;
use super::vec3::Vec3;
use rand::distributions::UnitCircle;
use rand::prelude::*;
use rand::random;

#[macro_export]
macro_rules! camera {
    ($pos:expr, $at:expr, $width:expr, $height:expr, $fov:expr) => {
        Camera::new(
            $pos.into(),
            $at.into(),
            (0, 1, 0).into(),
            $fov as f32,
            $width as f32 / $height as f32,
            0.0,
        )
    };
}

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
    /// fov is specified in degrees
    pub fn new(
        position: Vec3,
        look_at: Vec3,
        up_dir: Vec3,
        fov: f32,
        aspect: f32,
        aperture: f32,
    ) -> Self {
        let theta = fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (position - look_at).normalize();
        let u = up_dir.cross(&w).normalize();
        let v = w.cross(&u);
        let focus_dist = (position - look_at).len();
        Camera {
            origin: position,
            lower_left: position - (u * half_width + v * half_height + w) * focus_dist,
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
