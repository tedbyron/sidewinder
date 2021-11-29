use rand::prelude::{Distribution, ThreadRng};

use crate::graphics::{Hit as _, HitList, HitRecord};
use crate::math::{Point, Rgb, Vec3};

/// **P**(*t*) = **A** + *t***b** where **P** is a position along a 3D line, **A** is the ray
/// origin, and **b** is the ray direction. Change *t*, the distance from the origin, to affect the
/// color seen along the ray.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    #[inline]
    #[must_use]
    pub const fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Get a location along a ray path using the distance `t` from the ray origin.
    #[inline]
    #[must_use]
    pub fn at(self, t: f64) -> Point {
        self.direction.mul_add(t, self.origin) // self.origin + t * self.direction
    }

    #[inline]
    #[must_use]
    pub fn color(
        &self,
        world: &HitList,
        depth: usize,
        rng: &mut ThreadRng,
        dist: &impl Distribution<f64>,
    ) -> Rgb {
        let mut rec = HitRecord::default();
        if depth == 0 {
            return Rgb::default();
        }

        if world.hit(self, 0.001, f64::INFINITY, &mut rec) {
            let target = rec.p + rec.normal + Vec3::random_unit_vector(rng, dist);
            return 0.5 * Self::new(rec.p, target - rec.p).color(world, depth - 1, rng, dist);
        }

        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        // (1.0 - t) * Rgb::new(1.0, 1.0, 1.0) + t * Rgb::new(0.5, 0.7, 1.0)
        Rgb::new(1.0, 1.0, 1.0).mul_add(1.0 - t, Rgb::new(0.5, 0.7, 1.0) * t)
    }
}
