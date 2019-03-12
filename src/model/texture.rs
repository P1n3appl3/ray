use crate::scene::Color;
use crate::vec3::{ToF32, Vec3};
use image::hdr::HDRDecoder;
use image::{ImageBuffer, Pixel, Primitive, Rgb};
use itertools::iproduct;
use lazy_static::lazy_static;
use rand::{random, seq::SliceRandom, thread_rng};
use std::fs::File;
use std::io::BufReader;

pub trait Texture: Send + Sync + std::fmt::Debug {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Color;
    fn clone_box(&self) -> Box<dyn Texture>;
}

#[derive(Debug, Clone)]
pub struct Solid {
    color: Color,
}

impl Solid {
    pub fn new(color: Vec3) -> Self {
        Solid { color }
    }
}

impl Texture for Solid {
    fn value(&self, _u: f32, _v: f32, _p: Vec3) -> Color {
        self.color
    }
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct Checkered {
    a: Box<dyn Texture>,
    b: Box<dyn Texture>,
    size: f32,
}

impl Checkered {
    pub fn new(a: Box<dyn Texture>, b: Box<dyn Texture>, size: f32) -> Self {
        Checkered { a, b, size }
    }
}

impl Texture for Checkered {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Color {
        if (self.size * u).sin() * (self.size * v).sin() < 0.0 {
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
#[derive(Debug)]

pub struct Checkered3D {
    a: Box<dyn Texture>,
    b: Box<dyn Texture>,
    size: f32,
}

impl Checkered3D {
    pub fn new(a: Box<dyn Texture>, b: Box<dyn Texture>, size: f32) -> Self {
        Checkered3D { a, b, size }
    }
}

impl Texture for Checkered3D {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Color {
        if (self.size * p.x).sin() * (self.size * p.y).sin() * (self.size * p.z).sin()
            < 0.0
        {
            self.a.value(u, v, p)
        } else {
            self.b.value(u, v, p)
        }
    }
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(Checkered3D {
            a: self.a.clone_box(),
            b: self.b.clone_box(),
            size: self.size,
        })
    }
}

#[derive(Clone)]
pub enum PerlinVariant {
    Noise,
    Rock,
    Marble,
}

lazy_static! {
    static ref RAND_VEC: [Vec3; 256] = {
        let mut temp = [Vec3::default(); 256];
        temp.copy_from_slice(
            &(0..256)
                .map(|_| (random::<Vec3>() * 2.0 - Vec3::from_scalar(1.0)).normalize())
                .collect::<Vec<_>>(),
        );
        temp
    };
    static ref PERM_X: [u8; 256] = {
        let mut temp = [0; 256];
        let mut perm: Vec<u8> = (0..=255).collect();
        let mut rng = thread_rng();
        perm.shuffle(&mut rng);
        temp.copy_from_slice(&perm);
        temp
    };
    // TODO: dry (nested macro?!?!)
    static ref PERM_Y: [u8; 256] = {
        let mut temp = [0; 256];
        let mut perm: Vec<u8> = (0..=255).collect();
        let mut rng = thread_rng();
        perm.shuffle(&mut rng);
        temp.copy_from_slice(&perm);
        temp
    };
    static ref PERM_Z: [u8; 256] = {
        let mut temp = [0; 256];
        let mut perm: Vec<u8> = (0..=255).collect();
        let mut rng = thread_rng();
        perm.shuffle(&mut rng);
        temp.copy_from_slice(&perm);
        temp
    };
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
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
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

#[derive(Clone, Debug)]
pub struct Gradient {}

impl Texture for Gradient {
    fn value(&self, u: f32, v: f32, _p: Vec3) -> Color {
        Color::new(u, v, 1.0 - u - v)
    }
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

impl<T> Texture for ImageBuffer<Rgb<T>, Vec<T>>
where
    T: 'static + Send + Sync + Primitive + ToF32 + std::fmt::Debug,
{
    fn value(&self, u: f32, v: f32, _p: Vec3) -> Color {
        let x = (u * self.width() as f32) as u32;
        let y = ((1.0 - v) * self.height() as f32 - 0.001) as u32;
        let channels = self.get_pixel(x, y).channels();
        Color::new(channels[0].to(), channels[1].to(), channels[2].to())
    }
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

pub fn load_hdr_image(filename: &str) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    let hdr_decoder =
        HDRDecoder::new(BufReader::new(File::open(filename).unwrap())).unwrap();
    let metadata = hdr_decoder.metadata();
    ImageBuffer::from_vec(
        metadata.width,
        metadata.height,
        hdr_decoder
            .read_image_hdr()
            .unwrap()
            .into_iter()
            .map(|p| p.channels().iter().cloned().collect::<Vec<f32>>())
            .flatten()
            .collect::<Vec<f32>>(),
    )
    .unwrap()
}
