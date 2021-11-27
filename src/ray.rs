use crate::vec3::Vec3;

/// **P**(*t*) = **A** + *t***b** where **P** is a position along a 3D line, **A** is the ray
/// origin, and **b** is the ray direction. Change *t*, the distance from the origin, to affect the
/// color seen along the ray.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    #[inline]
    #[must_use]
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Calculate the
    #[inline]
    #[must_use]
    pub fn at(self, t: f64) -> Vec3 {
        self.direction.mul_add(t, self.origin)
    }
}
