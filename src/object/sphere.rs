use crate::graphics::{Hit, HitRecord, Ray};
use crate::math::Point;

#[non_exhaustive]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    #[inline]
    #[must_use]
    pub const fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        // Quadratic equation.
        let a = r.direction.len_squared();
        let half_b = oc.dot(r.direction);
        // oc.len_squared() - radius * radius
        let c = self.radius.mul_add(-self.radius, oc.len_squared());
        // half_b * half_b - a * c
        let discriminant = half_b.mul_add(half_b, -(a * c));

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;

            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}
