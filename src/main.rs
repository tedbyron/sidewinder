#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    rust_2018_idioms
)]
#![windows_subsystem = "console"]
#![doc = include_str!("../README.md")]

use std::io::{self, Write};
use std::time::Instant;

use indicatif::{HumanDuration, ProgressBar};
use once_cell::sync::Lazy;
use rand::distributions::Uniform;
use rand::Rng;
use rayon::prelude::*;

use sidewinder::graphics::{HitList, Lambertian, MatList, Material, Metallic};
use sidewinder::math::{Point, Rgb};
use sidewinder::object::Sphere;
use sidewinder::util::{Camera, RngDist};

static MATS: Lazy<MatList> = Lazy::new(|| {
    sidewinder::matlist![
        "ground": Lambertian::from(Rgb::new(0.8, 0.8, 0.0)),
        "center": Lambertian::from(Rgb::new(0.7, 0.3, 0.3)),
        "left": Metallic::from(Rgb::new(0.8, 0.8, 0.8)),
        "right": Metallic::from(Rgb::new(0.8, 0.6, 0.2)),
    ]
});

static WORLD: Lazy<HitList> = Lazy::new(|| {
    sidewinder::hitlist![
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, &*MATS["ground"]),
        Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, &*MATS["center"]),
        Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, &*MATS["left"]),
        Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, &*MATS["right"]),
    ]
});

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;

    let image_width: u32 = 400;
    let image_width_f = f64::from(image_width);

    let image_height_f = if (image_width_f / aspect_ratio).fract() == 0.0 {
        image_width_f / aspect_ratio
    } else {
        panic!("image_width {image_width} is not valid for aspect_ratio {aspect_ratio}");
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let image_height = image_height_f as u32;

    // Antialiasing samples.
    let samples_per_pixel = 100;
    // Diffuse reflection depth.
    let max_depth = 50;

    let camera = Camera::new(aspect_ratio);
    let dist_n11 = Uniform::from(-1.0..1.0);
    #[allow(clippy::cast_sign_loss)]
    let bar = ProgressBar::new(u64::from(image_height)); // TODO: with_style(..)

    let timer = Instant::now();

    // Write pixel information to memory.
    let pixels = (0..image_height * image_width)
        .into_par_iter()
        .map(|i| (i % image_width, image_height - i / image_width - 1))
        .map(|(x, y)| {
            if x == 0 {
                bar.inc(1);
            }

            let mut rng = rand::thread_rng();
            let mut pixel = Rgb::default();

            for _ in 0..samples_per_pixel {
                let u = (f64::from(x) + rng.gen::<f64>()) / (image_width_f - 1.0);
                let v = (f64::from(y) + rng.gen::<f64>()) / (image_height_f - 1.0);

                let r = camera.ray(u, v);
                pixel += r.color(&WORLD, max_depth, &mut RngDist::new(&mut rng, &dist_n11));
            }

            pixel
        })
        .collect::<Vec<Rgb>>(); // TODO: avoid intermediate allocation

    bar.finish_and_clear();
    let bar = ProgressBar::new_spinner().with_message("Writing to stdout...");

    // Lock stdout until done writing.
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut buf = io::BufWriter::new(lock);

    // Write header information.
    writeln!(&mut buf, "P3\n{} {}\n255", image_width, image_height)?;

    // Write pixel information.
    for pixel in pixels {
        pixel.write(&mut buf, samples_per_pixel)?;
    }

    buf.flush()?;

    let elapsed = HumanDuration(timer.elapsed());
    bar.finish_with_message(format!("Done in {elapsed}"));

    Ok(())
}
