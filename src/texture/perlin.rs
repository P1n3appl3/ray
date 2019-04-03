use super::Texture;
use crate::scene::Color;
use crate::vec3::Vec3;
use itertools::iproduct;
use lazy_static::lazy_static;
use rand::{random, seq::SliceRandom, thread_rng};

#[derive(Clone)]
pub enum PerlinVariant {
    Noise,
    Rock,
    Marble,
}

fn rand_arr() -> [u8; 256] {
    let mut temp = [0; 256];
    let mut perm: Vec<u8> = (0..=255).collect();
    let mut rng = thread_rng();
    perm.shuffle(&mut rng);
    temp.copy_from_slice(&perm);
    temp
}

lazy_static! {
    static ref RAND_VEC: [Vec3; 256] = {
        let mut temp = [Vec3::zero(); 256];
        temp.copy_from_slice(
            &(0..256)
                .map(|_| (random::<Vec3>() * 2.0 - Vec3::from(1.0)).normalize())
                .collect::<Vec<_>>(),
        );
        temp
    };
    static ref PERM_X: [u8; 256] = rand_arr();
    static ref PERM_Y: [u8; 256] = rand_arr();
    static ref PERM_Z: [u8; 256] = rand_arr();
}

/// scale is inverted, smaller numbers make the pattern larger
#[derive(Clone)]
pub struct Perlin {
    scale: f32,
    color: Vec3,
    kind: PerlinVariant,
}

impl std::fmt::Debug for Perlin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "perlin with scale: {}", self.scale)
    }
}

impl Texture for Perlin {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Color {
        use PerlinVariant::*;
        match self.kind {
            Noise => self.color * (0.5 * (1.0 + self.noise(p * self.scale))),
            Marble => {
                self.color
                    * (0.5 * (1.0 + (p.z * self.scale + 10.0 * self.turb(p)).sin()))
            }
            Rock => self.color * (self.turb(p * self.scale)),
        }
    }
}

impl Perlin {
    pub fn new(scale: f32, color: Vec3, kind: PerlinVariant) -> Self {
        Perlin { scale, color, kind }
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
                    * RAND_VEC[(PERM_X[(p.x.floor() as usize + i) % 256]
                        ^ PERM_Y[(p.y.floor() as usize + j) % 256]
                        ^ PERM_Z[(p.z.floor() as usize + k) % 256])
                        as usize]
                        .dot(&Vec3::new(u - i as f32, v - j as f32, w - k as f32))
            })
            .sum()
    }

    pub fn turb(&self, p: Vec3) -> f32 {
        let depth = 7;
        let mut weight = 1.0;
        let mut accum = 0.0;
        let mut temp_p = p;
        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            temp_p *= 2.0;
            weight /= 2.0;
        }
        accum.abs()
    }
}
