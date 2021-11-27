#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms
)]
#![doc = include_str!("../README.md")]

use std::io::{self, Write as _};
use std::time::Instant;

use indicatif::{HumanDuration, ProgressIterator as _};

use sidewinder::ray::Ray;
use sidewinder::vec3::Vec3;

fn main() -> io::Result<()> {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_width_f = f64::from(image_width);
    let image_height_f = f64::from(image_width) / aspect_ratio;
    #[allow(clippy::cast_possible_truncation)]
    let image_height = image_height_f as i32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::default();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    // Render

    let timer = Instant::now();

    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut buf = io::BufWriter::new(lock); // TODO: make faster with_capacity?

    // Write header information.
    writeln!(&mut buf, "P3\n{} {}\n255", image_width, image_height)?;

    // Write pixel information.
    for j in (0..image_height).rev().progress() {
        for i in 0..image_width {
            let u = f64::from(i) / (image_width_f - 1.0);
            let v = f64::from(j) / (image_height_f - 1.0);

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let color = ray.color();

            color.write_rgb(&mut buf)?;
        }
    }

    buf.flush()?;
    eprintln!("PPM written in {}", HumanDuration(timer.elapsed()));

    Ok(())
}
