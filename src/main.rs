#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    rust_2018_idioms
)]
#![doc = include_str!("../README.md")]

use std::io::{self, BufWriter, Write};
use std::time::Instant;

use indicatif::{HumanDuration, ProgressBar};
use rand::distributions::Uniform;
use rand::Rng;
use rayon::prelude::*;

use sidewinder::graphics::{HitList, Lambertian, Material, Metallic};
use sidewinder::math::{Point, Rgb};
use sidewinder::object::Sphere;
use sidewinder::util::{Camera, RngDist};

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;

    let image_width: u32 = 1920;
    let image_width_f = f64::from(image_width);

    let image_height_f = if (image_width_f / aspect_ratio).fract() == 0.0 {
        image_width_f / aspect_ratio
    } else {
        panic!("image_width {image_width} is not valid for aspect_ratio {aspect_ratio}");
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let image_height = image_height_f as u32;

    // Antialiasing samples.
    let samples_per_pixel = 1000;
    // Diffuse reflection depth.
    let max_depth = 100;

    let mats = sidewinder::matlist![
        "ground": Lambertian::new(Rgb::new(0.8, 0.8, 0.0)),
        "lambertian": Lambertian::new(Rgb::new(0.7, 0.3, 0.3)),
        "metal": Metallic::new(Rgb::new(0.8, 0.8, 0.8), 0.3),
        "metal-opaque": Metallic::new(Rgb::new(0.8, 0.6, 0.2), 1.0),
    ];

    let world = sidewinder::hitlist![
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, mats["ground"].clone()),
        Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, mats["lambertian"].clone()),
        Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, mats["metal"].clone()),
        Sphere::new(
            Point::new(1.0, 0.0, -1.0),
            0.5,
            mats["metal-opaque"].clone()
        ),
    ];

    let camera = Camera::new(aspect_ratio);
    let dist_n11 = Uniform::from(-1.0..1.0);
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
            let mut pixel = Rgb::ZERO;

            for _ in 0..samples_per_pixel {
                let u = (f64::from(x) + rng.gen::<f64>()) / (image_width_f - 1.0);
                let v = (f64::from(y) + rng.gen::<f64>()) / (image_height_f - 1.0);

                let r = camera.ray(u, v);
                pixel += r.color(&world, max_depth, &mut RngDist::new(&mut rng, &dist_n11));
            }

            pixel
        })
        .collect::<Vec<Rgb>>(); // TODO: avoid intermediate allocation

    bar.finish_and_clear();
    let bar = ProgressBar::new_spinner().with_message("Writing to stdout...");

    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut buf = BufWriter::new(lock);

    // Header information.
    writeln!(&mut buf, "P3\n{} {}\n255", image_width, image_height)?;

    // Pixel information.
    for pixel in pixels {
        pixel.write(&mut buf, samples_per_pixel)?;
    }

    buf.flush()?;

    let elapsed = HumanDuration(timer.elapsed());
    bar.finish_with_message(format!("Done in {elapsed}"));

    Ok(())
}
