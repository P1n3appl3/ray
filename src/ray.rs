use super::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray { origin: a, dir: b }
    }

    pub fn point_at_param(&self, t: f32) -> Vec3{
        self.origin + self.dir.scale(t)
    }
}
