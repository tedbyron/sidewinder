use crate::vec3::{Point, Vec3};

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
}
