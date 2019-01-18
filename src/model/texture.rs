use crate::vec3::Vec3;

pub trait Texture: Send + Sync + std::fmt::Debug {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
    fn clone_box(&self) -> Box<dyn Texture>;
}

#[derive(Debug, Clone)]
pub struct Solid {
    pub color: Vec3,
}

impl Texture for Solid {
    fn value(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        self.color
    }
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct Checkered {
    pub a: Box<dyn Texture>,
    pub b: Box<dyn Texture>,
    pub size: f32,
}

impl Texture for Checkered {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        // TODO: use u and v instead to make 2d instead of 3d checkers
        if (self.size * p.x).sin() * (self.size * p.y).sin() * (self.size * p.z).sin()
            < 0.0
        {
            self.a.value(u, v, p)
        } else {
            self.b.value(u, v, p)
        }
    }
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(Checkered {
            a: self.a.clone_box(),
            b: self.b.clone_box(),
            size: self.size,
        })
    }
}
