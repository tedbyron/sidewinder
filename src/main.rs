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

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            ray_color(&r).write_rgb(&mut buf)?;
        }
    }

    buf.flush()?;
    eprintln!("PPM written in {}", HumanDuration(timer.elapsed()));

    Ok(())
}

#[inline]
#[must_use]
fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

#[inline]
#[must_use]
fn ray_color(r: &Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0).mul_add(1.0 - t, Vec3::new(0.5, 0.7, 1.0) * t)
}
