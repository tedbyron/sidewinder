use crate::vec3::Vec3;

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

    #[inline]
    #[must_use]
    pub fn at(self, t: f64) -> Vec3 {
        self.direction.mul_add(t, self.origin)
    }

    #[inline]
    #[must_use]
    pub fn color(self) -> Vec3 {
        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(0.5, 0.7, 1.0).mul_add(t, (1.0 - t) * Vec3::default())
    }
}
