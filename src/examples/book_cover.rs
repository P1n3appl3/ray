//TODO: FIX
fn main() {
    let width = 150;
    let height = 100;
    let mut textures = vec![
        Checker(Vec3::from_scalar(0.3), Vec3::new(0.8, 0.0, 0.0)),
        Solid(Vec3::new(0.2, 0.3, 0.7)),
    ];
    let mut spheres: Vec<Box<Hitable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Diffuse(textures.first().unwrap()),
        }),
        Box::new(Sphere {
            center: Vec3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Diffuse(textures.last().unwrap()),
        }),
        Box::new(Sphere {
            center: Vec3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Dielectric(1.5),
        }),
    ];

    for a in -11..11 {
        for b in -11..11 {
            let pos = Vec3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );
            if (pos - Vec3::new(4.0, 0.2, 0.0)).len() < 0.9 {
                continue;
            }
            spheres.push(Box::new(Sphere {
                center: pos,
                radius: 0.2,
                material: match (random::<f32>() * 100.0) as u8 {
                    0...5 => Dielectric(1.5),
                    5...20 => Metal(
                        (Vec3::new(1.0, 1.0, 1.0)
                            + Vec3::new(random(), random(), random()))
                        .scale(0.5),
                        random::<f32>() / 2.0,
                    ),
                    _ => {
                        textures.push(Solid(Vec3::new(
                            random::<f32>() * random::<f32>(),
                            random::<f32>() * random::<f32>(),
                            random::<f32>() * random::<f32>(),
                        )));
                        Diffuse(textures.last().unwrap())
                    }
                },
            }))
        }
    }

    let cam = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        width as f32 / height as f32,
        0.0,
    );
    let file = fs::File::create("test.png").unwrap();
    let ref mut writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, width as u32, height as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let data = render(&obj, cam, width, height, samples, bounces);
    writer.write_image_data(&data).unwrap();
}
