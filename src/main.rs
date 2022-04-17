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
use std::sync::Arc;
use std::time::Instant;

use clap::Parser;
use indicatif::{HumanDuration, ProgressBar};
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rayon::prelude::*;

use sidewinder::camera::Camera;
use sidewinder::graphics::{Dielectric, HitList, Lambertian, Material, Metallic};
use sidewinder::math::{Point, Rgb, Vec3};
use sidewinder::object::{MovingSphere, Sphere};
use sidewinder::rng::CLOSED_OPEN_01;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Image width.
    #[clap(short = 'w', long = "width", default_value_t = 400)]
    image_width: u32,

    /// Image aspect ratio.
    #[clap(short = 'r', long, default_value_t = 16.0 / 9.0)]
    aspect_ratio: f64,

    /// Antialiasing samples per pixel.
    #[clap(short, long = "samples", default_value_t = 100)]
    samples_per_pixel: u32,

    /// Diffuse reflection recursion depth.
    #[clap(short = 'd', long = "depth", default_value_t = 50)]
    max_depth: usize,

    /// Output path.
    #[clap(value_name = "PATH")]
    output_path: Option<String>,

    /// Overwrite existing files.
    #[clap(short, long)]
    force: bool,
}

#[allow(clippy::too_many_lines)]
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
            .create(force)
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

    let mut mats = sidewinder::matlist![
        "ground": Lambertian::new(Rgb::newf(0.5, 0.5, 0.5)),
        "dielectric": Dielectric::new(1.5),
        "lambertian": Lambertian::new(Rgb::newf(0.4, 0.2, 0.1)),
        "metallic": Metallic::new(Rgb::newf(0.7, 0.6, 0.5), 0.0),
    ];
    let mut world = sidewinder::hitlist![
        Sphere::new(Point::newi(0, -1000, 0), 1000.0, mats["ground"].clone()),
        Sphere::new(Point::newi(0, 1, 0), 1.0, mats["dielectric"].clone()),
        Sphere::new(Point::newi(-4, 1, 0), 1.0, mats["lambertian"].clone()),
        Sphere::new(Point::newi(4, 1, 0), 1.0, mats["metallic"].clone()),
    ];

    let mut rng = rand::thread_rng();
    let uniform_0_p5 = Uniform::<f64>::from(0.0..0.5);
    let uniform_p5_1 = Uniform::<f64>::from(0.5..1.0);
    let offset = Point::newf(4.0, 0.2, 0.0);

    for a in -11..11 {
        let a = f64::from(a);

        for b in -11..11 {
            let b = f64::from(b);

            let choose_mat = CLOSED_OPEN_01.sample(&mut rng);
            let center = Point::newf(
                0.9_f64.mul_add(CLOSED_OPEN_01.sample(&mut rng), a),
                0.2,
                0.9_f64.mul_add(CLOSED_OPEN_01.sample(&mut rng), b),
            );

            if (center - offset).len() > 0.9 {
                match choose_mat {
                    n if n < 0.8 => {
                        let albedo = Rgb::random(&mut rng) * Rgb::random(&mut rng);
                        let name = format!("lambertian {}, {}, {}", albedo.x, albedo.y, albedo.z);
                        let name_clone = name.clone();

                        mats.entry(name)
                            .or_insert_with(|| Arc::new(Lambertian::new(albedo)));

                        let center_end =
                            center + Vec3::newf(0.0, uniform_0_p5.sample(&mut rng), 0.0);
                        let sphere = MovingSphere::new(
                            center,
                            center_end,
                            0.0,
                            1.0,
                            0.2,
                            mats[&name_clone].clone(),
                        );
                        world.push(Box::new(sphere));
                    }
                    n if n < 0.95 => {
                        let albedo = Rgb::random_in(&uniform_p5_1, &mut rng);
                        let blur = uniform_0_p5.sample(&mut rng);
                        let name = format!(
                            "metallic {}, {}, {}, {}",
                            albedo.x, albedo.y, albedo.z, blur
                        );
                        let name_clone = name.clone();

                        mats.entry(name)
                            .or_insert_with(|| Arc::new(Metallic::new(albedo, blur)));

                        let sphere = Sphere::new(center, 0.2, mats[&name_clone].clone());
                        world.push(Box::new(sphere));
                    }
                    _ => {
                        let sphere = Sphere::new(center, 0.2, mats["dielectric"].clone());
                        world.push(Box::new(sphere));
                    }
                };
            }
        }
    }

    let from = Point::newi(13, 2, 3);
    let to = Point::newi(0, 0, 0);
    let v_up = Vec3::newi(0, 1, 0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        from,
        to,
        v_up,
        20.0,
        aspect_ratio,
        aperture,
        focus_dist,
        0.0,
        1.0,
    );

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
    let bar = ProgressBar::new_spinner().with_message("Writing to stdout...");

    let write_output = |buf: &mut dyn Write| -> io::Result<()> {
        // Header information.
        writeln!(buf, "P3\n{image_width} {image_height}\n255")?;

        // Pixel information.
        for pixel in pixels {
            pixel.write(buf, samples_per_pixel)?;
        }

        Ok(())
    };

    // The `BufWriter` can have different types, call `write_output` in each case.
    if let Some(ref path) = output_path {
        let file = OpenOptions::new().write(true).truncate(force).open(path)?;
        let mut buf = BufWriter::new(file);
        write_output(&mut buf)?;
    } else {
        let stdout = io::stdout();
        let lock = stdout.lock();
        let mut buf = BufWriter::new(lock);
        write_output(&mut buf)?;
    };

    let elapsed = HumanDuration(timer.elapsed());
    bar.finish_with_message(format!("Done in {elapsed}"));

    Ok(())
}
