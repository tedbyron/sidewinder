#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    rust_2018_idioms
)]
#![doc = include_str!("../README.md")]

use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
use std::time::Instant;

use clap::Parser;
use indicatif::{HumanDuration, ProgressBar};
use rand::distributions::Uniform;
use rand::Rng;
use rayon::prelude::*;

use sidewinder::graphics::{Dialectric, HitList, Lambertian, Material, Metallic};
use sidewinder::math::{Point, Rgb};
use sidewinder::object::Sphere;
use sidewinder::util::{Camera, RngDist};

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Image width.
    #[clap(short = 'w', long = "width", default_value_t = 400)]
    image_width: u32,

    /// Image aspect ratio.
    #[clap(short = 'r', long, default_value_t = 16.0 / 9.0)]
    aspect_ratio: f64,

    /// Antialiasing Samples.
    #[clap(short, long = "samples", default_value_t = 100)]
    samples_per_pixel: u32,

    /// Diffuse reflection depth.
    #[clap(short = 'd', long = "depth", default_value_t = 50)]
    max_depth: usize,

    /// Output path.
    #[clap(value_name = "PATH")]
    output_path: Option<String>,

    /// Overwrite existing files.
    #[clap(short, long)]
    force: bool,
}

fn main() -> io::Result<()> {
    let Args {
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        output_path,
        force,
    } = Args::parse();
    if let Some(ref path) = output_path {
        // Create the file if it doesn't exist, or exit with an error.
        OpenOptions::new()
            .write(true)
            .create_new(!force)
            .open(path)?;
    }

    let image_width_f = f64::from(image_width);

    let image_height_f = if (image_width_f / aspect_ratio).fract() < f64::EPSILON {
        image_width_f / aspect_ratio
    } else {
        panic!("Error: image_width {image_width} is not valid for aspect_ratio {aspect_ratio}");
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let image_height = image_height_f as u32;

    let mats = sidewinder::matlist![
        "ground": Lambertian::new(Rgb::new(0.8, 0.8, 0.0)),
        "center": Dialectric::new(1.5),
        "left": Dialectric::new(1.5),
        "right": Metallic::new(Rgb::new(0.8, 0.6, 0.2), 1.0),
    ];

    let world = sidewinder::hitlist![
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, mats["ground"].clone()),
        Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, mats["center"].clone()),
        Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, mats["left"].clone()),
        Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, mats["right"].clone()),
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
                                // may require a parallel to sequential adapter
                                // https://github.com/rayon-rs/rayon/issues/210

    bar.finish_and_clear();
    let bar = ProgressBar::new_spinner().with_message("Writing to stdout...");

    // The `BufWriter` can have different types, so we must call `write_output` in each case.
    if let Some(ref path) = output_path {
        let file = OpenOptions::new().write(true).truncate(force).open(path)?;
        let mut buf = BufWriter::new(file);

        write_output(
            &mut buf,
            pixels,
            image_width,
            image_height,
            samples_per_pixel,
        )?;
    } else {
        let stdout = io::stdout();
        let lock = stdout.lock();
        let mut buf = BufWriter::new(lock);

        write_output(
            &mut buf,
            pixels,
            image_width,
            image_height,
            samples_per_pixel,
        )?;
    };

    let elapsed = HumanDuration(timer.elapsed());
    bar.finish_with_message(format!("Done in {elapsed}"));

    Ok(())
}

fn write_output(
    buf: &mut dyn Write,
    pixels: Vec<Rgb>,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
) -> io::Result<()> {
    // Header information.
    writeln!(buf, "P3\n{image_width} {image_height}\n255")?;

    // Pixel information.
    for pixel in pixels {
        pixel.write(buf, samples_per_pixel)?;
    }

    Ok(())
}
