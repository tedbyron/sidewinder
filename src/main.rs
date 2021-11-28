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
use std::rc::Rc;
use std::time::Instant;

use indicatif::{HumanDuration, ProgressIterator as _};
use rand::Rng;

use sidewinder::{Camera, HitList, HitRecord, Point, Ray, Rgb, Sphere};

fn main() -> io::Result<()> {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_width_f = f64::from(image_width);
    let image_height_f = f64::from(image_width) / aspect_ratio;
    #[allow(clippy::cast_possible_truncation)]
    let image_height = image_height_f as i32;
    let samples_per_pixel = 100;

    // World

    let mut world = HitList::default();
    world.push(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let camera = Camera::new(aspect_ratio);

    // Render

    let timer = Instant::now();

    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut buf = io::BufWriter::new(lock); // TODO: calculate buffer capacity first?

    writeln!(&mut buf, "P3\n{} {}\n255", image_width, image_height)?;

    let mut pixel_color;
    let mut rng = rand::thread_rng();

    // Write pixel information.
    for j in (0..image_height).rev().progress() {
        for i in 0..image_width {
            pixel_color = Rgb::default();

            for _ in 0..samples_per_pixel {
                let u = (f64::from(i) + rng.gen::<f64>()) / (image_width_f - 1.0);
                let v = (f64::from(j) + rng.gen::<f64>()) / (image_height_f - 1.0);

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            pixel_color.write(&mut buf, samples_per_pixel)?;
        }
    }

    buf.flush()?;
    eprintln!("PPM written in {}", HumanDuration(timer.elapsed()));

    Ok(())
}

#[allow(clippy::shadow_unrelated)]
fn ray_color(r: &Ray, world: &HitList<Sphere>) -> Rgb {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Rgb::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    // (1.0 - t) * Rgb::new(1.0, 1.0, 1.0) + t * Rgb::new(0.5, 0.7, 1.0)
    Rgb::new(1.0, 1.0, 1.0).mul_add(1.0 - t, Rgb::new(0.5, 0.7, 1.0) * t)
}
