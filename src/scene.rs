use crate::background::Background;
use crate::camera::Camera;
use crate::model::bvh::BVHNode;
use crate::model::hitable::Hitable;
use crate::ray::Ray;
use crate::vec3::Vec3;
use itertools::iproduct;
use rand::random;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use termion::cursor;

pub type Color = Vec3;

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color::new(
            f32::from(r) / 255.0,
            f32::from(g) / 255.0,
            f32::from(b) / 255.0,
        )
    }
}

fn color(
    r: Ray,
    world: &impl Hitable,
    bg: &dyn Background,
    depth: u16,
    bounces: u16,
) -> Color {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        let emited = hit.material.emit(hit.u, hit.v, hit.point);
        if let Some((attenuation, scattered)) =
            hit.material.scatter(r, hit.normal, hit.point, hit.u, hit.v)
        {
            if depth < bounces {
                return emited
                    + attenuation * color(scattered, world, bg, depth + 1, bounces);
            }
        }
        emited
    } else {
        bg.get_color(r)
    }
}

pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub objects: BVHNode,
    pub camera: Camera,
    pub samples: u16,
    pub bounces: u16,
    pub background: Box<dyn Background>,
}

static PROGRESS_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Scene {
    pub fn render(&self) -> Vec<u8> {
        iproduct!((0..self.height).rev(), 0..self.width)
            .collect::<Vec<(usize, usize)>>() // TODO: might not need this?
            .into_par_iter()
            .map(|(y, x)| {
                PROGRESS_COUNTER.fetch_add(1, Ordering::Relaxed);
                let col = ((0..self.samples)
                    .map(|_| {
                        color(
                            self.camera.get_ray(
                                // TODO: add jitter sampling
                                (x as f32 + random::<f32>()) / self.width as f32,
                                (y as f32 + random::<f32>()) / self.height as f32,
                            ),
                            &self.objects,
                            self.background.as_ref(),
                            0,
                            self.bounces,
                        )
                    })
                    .fold(Color::default(), |a, b| a + b)
                    / self.samples as f32)
                    .piecewise_min(Vec3::from_scalar(1));
                vec![
                    // sqrt for gamma 2 correction to brighten image
                    (col.x.sqrt() * 255.999) as u8,
                    (col.y.sqrt() * 255.999) as u8,
                    (col.z.sqrt() * 255.999) as u8,
                ]
            })
            .flatten()
            .collect::<Vec<u8>>()
    }

    fn show_progress(start: Instant, goal: usize, current: usize) {
        let percent = current as f32 / goal as f32;
        let elapsed = (Instant::now() - start).as_nanos() as f32 / 1_000_000_000.0;
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
        let output: image::RgbImage =
            image::ImageBuffer::from_vec(self.width as u32, self.height as u32, data)
                .unwrap();
        output.save(filename)?;
        Ok(())
    }
}
