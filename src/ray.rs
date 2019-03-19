use super::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray { origin: a, dir: b }
    }

    pub fn point_at_param(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_at_param() {
        let r = Ray::new(Vec3::new(-1, -1, -1), Vec3::new(0, 2, 3));
        assert_eq!(r.point_at_param(1.0), Vec3::new(-1, 1, 2))
    }
}
