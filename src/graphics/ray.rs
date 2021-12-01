use crate::graphics::{Hit as _, HitList};
use crate::math::{Point, Rgb, Vec3};
use crate::util::RngDist;

/// **P**(*t*) = **A** + *t***b** where **P** is a position along a 3D line, **A** is the ray
/// origin, and **b** is the ray direction. Change *t*, the distance from the origin, to affect the
/// color seen along the ray.
#[non_exhaustive]
#[derive(Clone, Copy)]
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

    #[must_use]
    pub fn color(&self, world: &HitList, depth: usize, rd: &mut RngDist<'_, '_>) -> Rgb {
        if depth == 0 {
            return Rgb::default();
        }

        if let Some(rec) = world.hit(self, 0.001, f64::INFINITY) {
            match rec.mat.scatter(self, &rec, rd) {
                Some(scatter) => {
                    return scatter.attenuation * scatter.ray.color(world, depth - 1, rd)
                }
                None => return Rgb::default(),
            }
        }

        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        // (1.0 - t) * Rgb::new(1.0, 1.0, 1.0) + t * Rgb::new(0.5, 0.7, 1.0)
        Rgb::new(1.0, 1.0, 1.0).mul_add(1.0 - t, Rgb::new(0.5, 0.7, 1.0) * t)
    }
}
