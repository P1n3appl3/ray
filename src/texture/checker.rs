use super::Texture;
use crate::scene::Color;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Checkered<Ta: Texture, Tb: Texture> {
    a: Ta,
    b: Tb,
    size: f32,
}

impl<Ta: Texture, Tb: Texture> Checkered<Ta, Tb> {
    pub fn new(a: Ta, b: Tb, size: f32) -> Self {
        Checkered { a, b, size }
    }
}

impl<Ta: Texture, Tb: Texture> Texture for Checkered<Ta, Tb> {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Color {
        if (self.size * u).sin() * (self.size * v).sin() < 0.0 {
            self.a.value(u, v, p)
        } else {
            self.b.value(u, v, p)
        }
    }
}
#[derive(Debug)]

pub struct Checkered3D<Ta: Texture, Tb: Texture> {
    a: Ta,
    b: Tb,
    size: f32,
}

impl<Ta: Texture, Tb: Texture> Checkered3D<Ta, Tb> {
    pub fn new(a: Ta, b: Tb, size: f32) -> Self {
        Checkered3D { a, b, size }
    }
}

impl<Ta: Texture, Tb: Texture> Texture for Checkered3D<Ta, Tb> {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Color {
        if (self.size * p.x).sin() * (self.size * p.y).sin() * (self.size * p.z).sin()
            < 0.0
        {
            self.a.value(u, v, p)
        } else {
            self.b.value(u, v, p)
        }
    }
}
