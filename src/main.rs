#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms
)]
#![windows_subsystem = "console"]
#![doc = include_str!("../README.md")]

use std::io::{self, Write as _};
use std::time::Instant;

use indicatif::{HumanDuration, ProgressBar};
use rand::distributions::Uniform;
use rand::Rng;
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};

use sidewinder::graphics::HitList;
use sidewinder::math::{Point, Rgb};
use sidewinder::object::Sphere;
use sidewinder::util::{Camera, RngDist};

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;

    let image_width: u32 = 400;
    let image_width_f = f64::from(image_width);

    let image_height_f = if (image_width_f / aspect_ratio).fract() == 0.0 {
        image_width_f / aspect_ratio
    } else {
        panic!(
            "image_width {} is not valid for aspect_ratio {}",
            image_width, aspect_ratio
        );
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let image_height = image_height_f as u32;

    // Subtract one now so we don't need to in loops.
    let image_width_f = image_width_f - 1.0;
    let image_height_f = image_height_f - 1.0;

    // Antialiasing samples.
    let samples_per_pixel = 100;
    // Diffuse reflection depth.
    let max_depth = 50;

    let world = sidewinder::hitlist![
        Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0),
    ];

    let camera = Camera::new(aspect_ratio);

    // Uniform distribution for unit types.
    let dist_n11 = Uniform::from(-1.0..1.0);
    // Progress bar for pixel calculation.
    #[allow(clippy::cast_sign_loss)]
    let bar = ProgressBar::new(u64::from(image_height)); // TODO: with_style(..)

    // Measure elapsed time during render and writing.
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
                let u = (f64::from(x) + rng.gen::<f64>()) / image_width_f;
                let v = (f64::from(y) + rng.gen::<f64>()) / image_height_f;

                let r = camera.ray(u, v);
                pixel += r.color(&world, max_depth, &mut RngDist::new(&mut rng, &dist_n11));
            }

            pixel
        })
        .collect::<Vec<Rgb>>();

    bar.finish_and_clear();
    let bar = ProgressBar::new_spinner().with_message("Writing to stdout...");

    // Lock stdout until done writing.
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut buf = io::BufWriter::new(lock); // TODO: with_capacity(..)

    // Write header information.
    writeln!(&mut buf, "P3\n{} {}\n255", image_width, image_height)?;

    // Write pixel information.
    for pixel in pixels {
        pixel.write(&mut buf, samples_per_pixel)?;
    }

    buf.flush()?;
    bar.finish_with_message(format!("Done in {}", HumanDuration(timer.elapsed())));

    Ok(())
}
