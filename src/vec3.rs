use crate::axis::Axis;
use packed_simd::{f32x4, shuffle};
use rand::distributions::{Distribution, Standard, UnitSphereSurface};
use rand::prelude::*;
use serde::Deserialize;
use std::f32;
use std::fmt;
use std::ops::*;

pub trait ToF32: Copy {
    fn to(&self) -> f32;
}

impl ToF32 for f32 {
    fn to(&self) -> f32 {
        *self
    }
}

impl ToF32 for i32 {
    fn to(&self) -> f32 {
        *self as f32
    }
}

impl ToF32 for u8 {
    fn to(&self) -> f32 {
        f32::from(*self) / 255.0
    }
}

// TODO: replace with Deserialize_tuple
#[derive(Copy, Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new<T: ToF32, U: ToF32, V: ToF32>(x: T, y: U, z: V) -> Self {
        Vec3 {
            x: x.to(),
            y: y.to(),
            z: z.to(),
        }
    }

    pub fn rand_in_unit_sphere() -> Self {
        let mut p;
        while {
            p = random::<Vec3>() * 2.0 - Vec3::from(1.0);
            p.square_len() > 1.0
        } {}
        p
    }

    pub fn almost_faster_rand() -> Self {
        let r = UnitSphereSurface::new().sample(&mut thread_rng());
        Vec3::new(r[0] as f32, r[1] as f32, r[2] as f32) * random::<f32>().cbrt()
    }

    pub fn get_axis(&self, a: Axis) -> f32 {
        match a {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn set_axis(&self, a: Axis, val: f32) -> Self {
        let mut temp = *self;
        match a {
            Axis::X => temp.x = val,
            Axis::Y => temp.y = val,
            Axis::Z => temp.z = val,
        }
        temp
    }

    // Each axis rotation just uses a solution based on the rotation matrix
    // stackoverflow.com/questions/14607640/rotating-a-vector-in-3d-space
    pub fn rotate_x(&self, cos_theta: f32, sin_theta: f32) -> Vec3 {
        Vec3::new(
            self.x,
            cos_theta * self.y - sin_theta * self.z,
            sin_theta * self.y + cos_theta * self.z,
        )
    }

    pub fn rotate_y(&self, cos_theta: f32, sin_theta: f32) -> Vec3 {
        Vec3::new(
            cos_theta * self.x + sin_theta * self.z,
            self.y,
            -sin_theta * self.x + cos_theta * self.z,
        )
    }

    pub fn rotate_z(&self, cos_theta: f32, sin_theta: f32) -> Vec3 {
        Vec3::new(
            cos_theta * self.x - sin_theta * self.y,
            sin_theta * self.x + cos_theta * self.y,
            self.z,
        )
    }

    pub fn rotate(&self, a: Axis, cos_theta: f32, sin_theta: f32) -> Self {
        match a {
            Axis::X => self.rotate_x(cos_theta, sin_theta),
            Axis::Y => self.rotate_y(cos_theta, sin_theta),
            Axis::Z => self.rotate_z(cos_theta, sin_theta),
        }
    }

    pub fn square_len(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn len(&self) -> f32 {
        self.square_len().sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / Self::from(self.len())
    }

    pub fn piecewise_max(&self, other: Self) -> Self {
        Vec3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    pub fn piecewise_min(&self, other: Self) -> Self {
        Vec3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - (*normal * self.dot(normal) * 2.0)
    }

    pub fn refract(&self, normal: &Self, index_ratio: f32) -> Option<Self> {
        let unit = self.normalize();
        let dt = unit.dot(normal);
        let discriminant = 1.0 - index_ratio * index_ratio * (1.0 - dt * dt);
        if discriminant > 0.0 {
            Some((unit - *normal * dt) * index_ratio - *normal * discriminant.sqrt())
        } else {
            None
        }
    }

    pub fn fast_cross(&self, other: &Self) -> Self {
        let a = f32x4::from(*self);
        let b = f32x4::from(*other);
        let a_yzx = shuffle!(a, [3, 0, 2, 1]);
        let b_yzx = shuffle!(b, [3, 0, 2, 1]);
        let c = a * b_yzx - a_yzx * b;
        shuffle!(c, [3, 0, 2, 1]).into()
    }

    pub fn fast_add(&self, other: &Self) -> Self {
        (f32x4::from(*self) + f32x4::from(*other)).into()
    }

    pub fn fast_dot(&self, other: &Self) -> Self {
        (f32x4::from(*self) * f32x4::from(*other)).sum().into()
    }
}

impl From<Vec3> for f32x4 {
    fn from(v: Vec3) -> Self {
        f32x4::new(v.x, v.y, v.z, 0.0)
    }
}

impl From<f32x4> for Vec3 {
    fn from(v: f32x4) -> Self {
        Vec3::new(v.extract(0), v.extract(1), v.extract(2))
    }
}

impl<T: ToF32> From<T> for Vec3 {
    fn from(n: T) -> Self {
        Vec3::new(n, n, n)
    }
}

impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
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
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
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
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

// TODO: make this a blanket impl
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Vec3::new(self.x / other, self.y / other, self.z / other)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
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
            Vec3::new(1.5, 0, -0.5),
            Vec3 {
                x: 1.5,
                y: 0.0,
                z: -0.5,
            }
        );
        assert_eq!(
            Vec3 {
                x: 0.25,
                y: 0.25,
                z: 0.25
            },
            Vec3::from(0.25)
        );
    }

    #[test]
    fn test_arithmetic() {
        let a = Vec3::new(-4, 1.25, 16.5);
        let b = Vec3::new(4, 5, 4);
        assert_eq!(a + b, Vec3::new(0, 6.25, 20.5));
        assert_eq!(a - b, Vec3::new(-8, -3.75, 12.5));
        assert_eq!(a / b, Vec3::new(-1, 0.25, 4.125));
        assert_eq!(a * b, Vec3::new(-16, 6.25, 66));
    }

    #[test]
    fn test_len() {
        assert!(Vec3::new(1, 0, 0).len() - 1.0 <= EPSILON);
        assert!(Vec3::new(4, 4, 4).len() - (48.0f32).sqrt() <= EPSILON);
        assert!(Vec3::new(-3, 0, 4).len() - 5.0 <= EPSILON);
    }

    #[test]
    fn test_norm() {
        assert_eq!(
            Vec3::new(8, 4, 8).normalize(),
            Vec3::new(2, 1, 2) / Vec3::from(3)
        );
        for _ in 0..10 {
            assert!(
                (random::<Vec3>() * random::<f32>() * 2.0).normalize().len()
                    <= 1.0 + EPSILON
            );
        }
    }

    #[test]
    fn test_dot() {
        assert_eq!(Vec3::new(1, 2, 3).dot(&Vec3::new(4, -5, 6)), 12.0);
        let v = Vec3::new(0, 2, 4);
        assert_eq!(v.dot(&v), 20.0);
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vec3::new(3, -3, 1).cross(&Vec3::new(4, 9, 2)),
            Vec3::new(-15, -2, 39)
        );
        assert_eq!(
            Vec3::new(3, -3, 1).cross(&Vec3::new(-12, 12, -4)),
            Vec3::new(0, 0, 0)
        );
    }
}
