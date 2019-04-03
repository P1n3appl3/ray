use crate::scene::Color;
use crate::vec3::{ToF32, Vec3};
use itertools::iproduct;
use lazy_static::lazy_static;
use rand::{random, seq::SliceRandom, thread_rng};
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
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
}
