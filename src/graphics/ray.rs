use rand::prelude::*;

use crate::{
    graphics::{Hit, HitList},
    math::{Point, Rgb, Vec3},
};

/// Calculates the color seen along a ray.
///
/// **P**(*t*) = **A** + *t***b** where **P** is a position along a 3D line, **A** is the ray
/// origin, and **b** is the ray direction. Change *t*, the distance from the origin, to affect the
/// color seen along the ray.
#[non_exhaustive]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
    pub t: f64,
}

impl Ray {
    pub const fn new(origin: Point, direction: Vec3, t: f64) -> Self {
        Self {
            origin,
            direction,
            t,
        }
    }

    /// The location along the ray's path which is distance `t` from the ray's origin.
    pub fn at(&self, t: f64) -> Point {
        self.direction.mul_add(t, self.origin)
    }

    /// The color seen along the ray.
    //
    // Better approximation of ideal Lambertian diffuse:
    // let target = rec.p + Vec3::random_in_hemisphere(rec.normal, rd);
    // return 0.5 * Self::new(rec.p, target - rec.p).color(world, depth - 1, rd);
    pub fn color(&self, world: &HitList, depth: usize, rng: &mut ThreadRng) -> Rgb {
        // If the maximum diffuse reflection depth is reached, no more light is gathered.
        if depth == 0 {
            return Rgb::ZERO;
        }

        if let Some(ref rec) = world.hit(self, 0.001, f64::INFINITY) {
            match rec.mat.scatter(self, rec, rng) {
                Some(scattered) => {
                    return scattered.attenuation * scattered.ray.color(world, depth - 1, rng);
                }
                None => return Rgb::ZERO,
            }
        }

        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        // (1.0 - t) * Rgb::ONE + t * Rgb::newf(0.5, 0.7, 1.0)
        Rgb::ONE.mul_add(1.0 - t, t * Rgb::newf(0.5, 0.7, 1.0))
    }
}
