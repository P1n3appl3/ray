use crate::background::Background;
use crate::camera::Camera;
use crate::model::bvh::BVHNode;
use crate::model::hitable::Hitable;
use crate::ray::Ray;
use crate::vec3::{ToF32, Vec3};
use image::{ImageBuffer, Pixel, Rgb, RgbImage};
use itertools::iproduct;
use rand::random;
use rayon::prelude::*;
use std::f32;
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use termion::cursor;

pub type Color = Vec3;
pub type MatID = u16;

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color::new(r.to(), g.to(), b.to())
    }
}

fn color(
    r: Ray,
    world: &impl Hitable,
    bg: &dyn Background,
    show_bg: bool,
    depth: u16,
    max_bounces: u16,
) -> Color {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        let emited = hit.material.emit(hit.u, hit.v, hit.point);
        if let Some((attenuation, scattered)) =
            hit.material.scatter(r, hit.normal, hit.point, hit.u, hit.v)
        {
            if depth < max_bounces {
                return emited
                    + attenuation
                        * color(scattered, world, bg, show_bg, depth + 1, max_bounces);
            }
        }
        emited
    } else if show_bg || depth > 0 {
        bg.get_color(r)
    } else {
        Color::default()
    }
}

pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub objects: BVHNode,
    pub camera: Camera,
    pub samples: u16,
    pub bounces: u16,
    pub show_bg: bool,
    pub background: Box<dyn Background>,
}

static PROGRESS_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Scene {
    pub fn render(&self) -> Vec<Rgb<f32>> {
        if cfg!(feature = "single_thread") {
            iproduct!((0..self.height).rev(), 0..self.width)
                .map(|(y, x)| self.render_pixel(x, y))
                .collect::<Vec<Rgb<f32>>>()
        } else {
            iproduct!((0..self.height).rev(), 0..self.width)
                .collect::<Vec<(usize, usize)>>() // TODO: might not need this?
                .into_par_iter()
                .map(|(y, x)| self.render_pixel(x, y))
                .collect::<Vec<Rgb<f32>>>()
        }
    }

    fn render_pixel(&self, x: usize, y: usize) -> Rgb<f32> {
        PROGRESS_COUNTER.fetch_add(1, Ordering::Relaxed);
        let col = (0..self.samples)
            .map(|_| {
                color(
                    self.camera.get_ray(
                        // TODO: add jitter sampling
                        (x as f32 + random::<f32>()) / self.width as f32,
                        (y as f32 + random::<f32>()) / self.height as f32,
                    ),
                    &self.objects,
                    self.background.as_ref(),
                    self.show_bg,
                    0,
                    self.bounces,
                )
            })
            .fold(Color::default(), |a, b| a + b)
            / f32::from(self.samples);
        Rgb {
            data: [col.x, col.y, col.z],
        }
    }

    fn show_progress(start: Instant, goal: usize, current: usize) {
        let percent = current as f32 / goal as f32;
        let elapsed = (Instant::now() - start).as_millis() as f32 / 1000.0;
        print!("{}", cursor::Up(4));
        println!("progress: {:.1}%    ", percent * 100.0);
        println!("elapsed: {:.1}s    ", elapsed);
        println!("remaining: {:.1}s    ", elapsed / percent - elapsed);
        println!("speed: {} rays/s    ", (current as f32 / elapsed) as usize);
    }

    pub fn render_to_file(&self, filename: &str) -> std::io::Result<()> {
        let samples = self.samples as usize;
        let goal = self.width * self.height * samples;
        let start_time = Instant::now();
        println!("\n\n\n");
        thread::spawn(move || {
            let mut prog = 0;
            while prog < goal {
                thread::sleep(Duration::from_millis(100));
                Scene::show_progress(start_time, goal, prog);
                prog = PROGRESS_COUNTER.load(Ordering::Relaxed) * samples;
            }
        });
        let data = self.render();
        Scene::show_progress(start_time, goal, goal);
        #[cfg(feature = "hdr_output")]
        {
            let buffer = File::create(PathBuf::from(filename).with_extension("hdr"))?;
            let encoder = image::hdr::HDREncoder::new(buffer);
            encoder.encode(data.as_slice(), self.width, self.height)?;
        }
        let data = data
            .iter()
            .map(|p| {
                p.channels()
                    .iter()
                    // cap pixel brightness at 1 and sqrt for gamma 2 correction
                    .map(|n| (n.min(1.0).sqrt() * 255.999) as u8)
                    .collect::<Vec<u8>>()
            })
            .flatten()
            .collect::<Vec<u8>>();
        let output: RgbImage =
            ImageBuffer::from_vec(self.width as u32, self.height as u32, data).unwrap();
        output.save(filename)?;
        Ok(())
    }
}
