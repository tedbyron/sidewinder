use std::mem;

use strum::IntoEnumIterator;

use crate::graphics::Ray;
use crate::math::{Axis, Point};

/// An axis-aligned bounding box.
#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct Aabb {
    pub min: Point,
    pub max: Point,
}

impl Aabb {
    #[inline]
    #[must_use]
    pub const fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    #[must_use]
    pub fn hit(self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for axis in Axis::iter() {
            let inv_d = r.direction[axis].recip();

            let mut t_start = (self.min[axis] - r.origin[axis]) * inv_d;
            let mut t_end = (self.max[axis] - r.origin[axis]) * inv_d;

            if inv_d < 0.0 {
                mem::swap(&mut t_start, &mut t_end);
            }

            let t_min = if t_start > t_min { t_start } else { t_min };
            let t_max = if t_end < t_max { t_end } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    #[must_use]
    pub fn surrounding_box(self, other: Self) -> Self {
        let small = Point::newf(
            self.min.x.min(other.min.x),
            self.min.y.min(other.min.y),
            self.min.z.min(other.min.z),
        );
        let big = Point::newf(
            self.max.x.max(other.max.x),
            self.max.y.max(other.max.y),
            self.max.z.max(other.max.z),
        );

        Self::new(small, big)
    }
}
