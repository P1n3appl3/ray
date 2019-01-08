use std::fmt;
use std::ops::*;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x: x, y: y, z: z }
    }
    pub fn square_len(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
    pub fn len(&self) -> f32 {
        self.square_len().sqrt()
    }
    pub fn scale(&self, amt: f32) -> Self {
        *self * Self::from_scalar(amt)
    }
    pub fn normalize(self) -> Self {
        self / Self::from_scalar(self.len())
    }
    pub fn reflect(&self, normal: &Self) -> Self {
        *self - normal.scale(self.dot(*normal) * 2.0)
    }
    pub fn refract(&self, normal: &Self, index_ratio: f32) -> Option<Self> {
        let unit = self.normalize();
        let dt = unit.dot(*normal);
        let discriminant = 1.0 - index_ratio * index_ratio * (1.0 - dt * dt);
        if discriminant > 0.0 {
            Some(
                (unit - normal.scale(dt)).scale(index_ratio)
                    - normal.scale(discriminant.sqrt()),
            )
        } else {
            None
        }
    }
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn from_scalar(n: f32) -> Self {
        Vec3 { x: n, y: n, z: n }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.4}, {:.4}, {:.4})", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;
    use std::f32::EPSILON;
    #[test]
    fn test_construct() {
        assert_eq!(
            Vec3 {
                x: 1.5,
                y: 0.0,
                z: -0.5
            },
            Vec3::new(1.5, 0.0, -0.5)
        );
        assert_eq!(
            Vec3 {
                x: 0.25,
                y: 0.25,
                z: 0.25
            },
            Vec3::from_scalar(0.25)
        );
    }

    #[test]
    fn test_arithmetic() {
        let a = Vec3::new(-4.0, 1.25, 16.5);
        let b = Vec3::new(4.0, 5.0, 4.0);
        assert_eq!(a + b, Vec3::new(0.0, 6.25, 20.5));
        assert_eq!(a - b, Vec3::new(-8.0, -3.75, 12.5));
        assert_eq!(a / b, Vec3::new(-1.0, 0.25, 4.125));
        assert_eq!(a * b, Vec3::new(-16.0, 6.25, 66.0));
    }

    #[test]
    fn test_len() {
        assert!(Vec3::new(1.0, 0.0, 0.0).len() - 1.0 <= EPSILON);
        assert!(Vec3::new(4.0, 4.0, 4.0).len() - (48.0f32).sqrt() <= EPSILON);
        assert!(Vec3::new(-3.0, 0.0, 4.0).len() - 5.0 <= EPSILON);
    }

    #[test]
    fn test_norm() {
        assert_eq!(
            Vec3::new(8.0, 4.0, 8.0).normalize(),
            Vec3::new(2.0, 1.0, 2.0) / Vec3::from_scalar(3.0)
        );
        for _ in 0..10 {
            assert!(Vec3::new(random(), random(), random()).normalize().len() <= 1.0);
        }
    }

    #[test]
    fn test_scale() {
        assert_eq!(
            Vec3::new(3.0, 4.0, 5.0).scale(1.5),
            Vec3::new(4.5, 6.0, 7.5)
        );
        assert_eq!(
            Vec3::new(3.0, 4.0, 5.0).scale(0.5),
            Vec3::new(1.5, 2.0, 2.5)
        );
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).dot(Vec3::new(4.0, -5.0, 6.0)),
            12.0
        );
        let v = Vec3::new(0.0, 2.0, 4.0);
        assert_eq!(v.dot(v), 20.0);
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vec3::new(3.0, -3.0, 1.0).cross(Vec3::new(4.0, 9.0, 2.0)),
            Vec3::new(-15.0, -2.0, 39.0)
        );
        assert_eq!(
            Vec3::new(3.0, -3.0, 1.0).cross(Vec3::new(-12.0, 12.0, -4.0)),
            Vec3::new(0.0, 0.0, 0.0)
        );
    }
}
