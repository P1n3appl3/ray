use crate::vec3::Vec3;
use itertools::iproduct;
use rand::{random, seq::SliceRandom, thread_rng};

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

#[derive(Clone)]
pub struct Perlin {
    scale: f32,
    // TODO: make these static
    rand_vec: [Vec3; 256],
    perm_x: [u8; 256],
    perm_y: [u8; 256],
    perm_z: [u8; 256],
}

impl std::fmt::Debug for Perlin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "perlin with scale: {}", self.scale)
    }
}

impl Texture for Perlin {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        // Vec3::from_scalar(0.5 * (1.0 + self.noise(p.scale(self.scale))))
        // Vec3::from_scalar(self.turb(p.scale(self.scale)))
        Vec3::from_scalar(0.5 * (1.0 + (p.z * self.scale + 10.0 * self.turb(p)).sin()))
    }
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

impl Perlin {
    pub fn new(scale: f32) -> Self {
        let mut temp = Perlin {
            scale: scale,
            rand_vec: [Vec3::default(); 256],
            perm_x: [0; 256],
            perm_y: [0; 256],
            perm_z: [0; 256],
        };
        temp.rand_vec.copy_from_slice(
            &(0..256)
                .map(|_| (Vec3::rand().scale(2.0) - Vec3::from_scalar(1.0)).normalize())
                .collect::<Vec<_>>(),
        );
        let mut perm: Vec<u8> = (0..256).map(|_| random::<u8>()).collect();
        let mut rng = thread_rng();
        perm.shuffle(&mut rng);
        temp.perm_x.copy_from_slice(&perm);
        perm.shuffle(&mut rng);
        temp.perm_y.copy_from_slice(&perm);
        perm.shuffle(&mut rng);
        temp.perm_z.copy_from_slice(&perm);
        temp
    }
    pub fn noise(&self, p: Vec3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        iproduct!(0..2, 0..2, 0..2)
            .map(|(i, j, k)| {
                (i as f32 * uu + (1 - i) as f32 * (1.0 - uu))
                    * (j as f32 * vv + (1 - j) as f32 * (1.0 - vv))
                    * (k as f32 * ww + (1 - k) as f32 * (1.0 - ww))
                    * self.rand_vec[(self.perm_x[(p.x.floor() as usize + i) % 256]
                        ^ self.perm_y[(p.y.floor() as usize + j) % 256]
                        ^ self.perm_z[(p.z.floor() as usize + k) % 256])
                        as usize]
                        .dot(&Vec3::new(u - i as f32, v - j as f32, w - k as f32))
            })
            .sum()
    }
    pub fn turb(&self, p: Vec3) -> f32 {
        let depth = 7;
        let mut weight = 2.0;
        p.scale(0.5);
        (0..depth)
            .map(|_| {
                p.scale(2.0);
                weight /= 2.0;
                weight * self.noise(p)
            })
            .sum::<f32>()
            .abs()
    }
}
