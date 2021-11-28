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

use sidewinder::hit::{HitList, HitRecord};
use sidewinder::ray::Ray;
use sidewinder::sphere::Sphere;
use sidewinder::vec3::{Point, Rgb, Vec3};

fn main() -> io::Result<()> {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_width_f = f64::from(image_width);
    let image_height_f = f64::from(image_width) / aspect_ratio;
    #[allow(clippy::cast_possible_truncation)]
    let image_height = image_height_f as i32;

    // World

    let mut world = HitList::default();
    world.push(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::default();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    // Render

    let timer = Instant::now();

    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut buf = io::BufWriter::new(lock); // TODO: calculate buffer capacity first?

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
            let color = ray_color(&r, &world);

            color.write(&mut buf)?;
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

// fn hit_sphere(center: Point, radius: f64, r: &Ray) -> f64 {
//     let oc = r.origin - center;
//     // Quadratic equation.
//     let a = r.direction.len_squared();
//     let half_b = oc.dot(r.direction);
//     let c = radius.mul_add(-radius, oc.len_squared()); // oc.len_squared() - radius * radius
//     let discriminant = half_b.mul_add(half_b, -(a * c)); // half_b * half_b - a * c

//     if discriminant < 0.0 {
//         -1.0
//     } else {
//         (-half_b - discriminant.sqrt()) / a
//     }
// }
