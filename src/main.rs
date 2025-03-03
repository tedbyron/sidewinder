#![warn(clippy::all, clippy::cargo, clippy::nursery, rust_2018_idioms)]
#![doc = include_str!("../README.md")]

use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufWriter, Write},
    path::Path,
    time::Instant,
};

use anyhow::{Result, bail};
use clap::Parser;
use image::ImageBuffer;
use indicatif::{HumanDuration, ProgressBar};
use rand::distr::Distribution;
use rayon::prelude::*;
use sidewinder::{
    camera::Camera,
    math::{Point, Rgb, Vec3},
    rng::CLOSED_OPEN_01,
};

mod scene_1;
mod scene_2;
mod scene_3;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Image width.
    #[clap(short = 'w', long = "width", default_value_t = 450)]
    image_width: u32,

    /// Image aspect ratio.
    #[clap(short = 'r', long, default_value_t = 1.5)]
    aspect_ratio: f64,

    /// Antialiasing samples per pixel.
    #[clap(short, long = "samples", default_value_t = 50)]
    samples_per_pixel: u32,

    /// Diffuse reflection recursion depth.
    #[clap(short = 'd', long = "depth", default_value_t = 100)]
    max_depth: usize,

    /// Output path.
    #[clap(value_name = "PATH")]
    output_path: Option<String>,

    /// Overwrite existing files.
    #[clap(short, long)]
    force: bool,
}

fn main() -> Result<()> {
    let world = scene_3::two_perlin_spheres();

    let Args {
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        output_path,
        force,
    } = Args::parse();
    if let Some(ref path) = output_path {
        OpenOptions::new()
            .write(true)
            .create_new(!force)
            .create(force)
            .open(path)?;
    }

    let image_width_f = f64::from(image_width);
    let image_height_f = if (image_width_f / aspect_ratio).fract() < f64::EPSILON {
        image_width_f / aspect_ratio
    } else {
        panic!("Error: image_width {image_width} is not valid for aspect_ratio {aspect_ratio}");
    };
    let image_height = image_height_f as u32;

    let camera = Camera::new(
        Point::newi(13, 2, 3),
        Point::newi(0, 0, 0),
        Vec3::newi(0, 1, 0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
        0.0,
        1.0,
    );

    let bar = ProgressBar::new(u64::from(image_height));
    let timer = Instant::now();

    let pixels = (0..image_height * image_width)
        .into_par_iter()
        .map(|i| (i % image_width, image_height - i / image_width - 1))
        .map(|(x, y)| {
            if x == 0 {
                bar.inc(1);
            }

            let mut rng = rand::rng();
            let mut pixel = Rgb::ZERO;

            for _ in 0..samples_per_pixel {
                let u = (f64::from(x) + CLOSED_OPEN_01.sample(&mut rng)) / (image_width_f - 1.0);
                let v = (f64::from(y) + CLOSED_OPEN_01.sample(&mut rng)) / (image_height_f - 1.0);

                let r = camera.ray(u, v, &mut rng);
                pixel += r.color(&world, max_depth, &mut rng);
            }

            pixel
        })
        .collect::<Vec<Rgb>>(); // TODO: avoid intermediate allocation
    // may require a parallel to sequential adapter
    // https://github.com/rayon-rs/rayon/issues/210

    bar.finish_and_clear();
    let bar = ProgressBar::new_spinner().with_message(format!(
        "Writing to {}...",
        output_path.as_ref().map_or("stdout", |path| path)
    ));

    // Write the image to a PPM file.
    let write_ppm = |buf: &mut dyn Write| -> Result<()> {
        writeln!(buf, "P3\n{image_width} {image_height}\n255")?;
        for pixel in &pixels {
            pixel.write(buf, samples_per_pixel)?;
        }
        buf.flush()?;
        Ok(())
    };

    // Write the image to the specified output.
    if let Some(ref path) = output_path {
        let path = Path::new(path);
        if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("png"))
        {
            if force && path.exists() {
                fs::remove_file(path)?;
            }
            let Some(buf) = ImageBuffer::<image::Rgb<u8>, _>::from_raw(
                image_width,
                image_height,
                pixels
                    .iter()
                    .flat_map(|p| p.to_rgb8(samples_per_pixel))
                    .collect::<Box<[_]>>(),
            ) else {
                bail!("Error: couldn't create image buffer");
            };
            buf.save(path)?;
        } else {
            let file = OpenOptions::new().write(true).truncate(force).open(path)?;
            let mut buf = BufWriter::new(file);
            write_ppm(&mut buf)?;
        }
    } else {
        let stdout = io::stdout();
        let lock = stdout.lock();
        let mut buf = BufWriter::new(lock);
        write_ppm(&mut buf)?;
    }

    let elapsed = HumanDuration(timer.elapsed());
    bar.finish_with_message(format!("Done in {elapsed}"));

    Ok(())
}
