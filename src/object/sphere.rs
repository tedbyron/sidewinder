use std::sync::Arc;

use crate::graphics::{Hit, HitRecord, Material, Ray};
use crate::math::Point;

/// A sphere object. The `mat` field uses an `Arc` to avoid duplicating existing [`Material`]s.
#[non_exhaustive]
pub struct Sphere {
    center: Point,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    #[inline]
    #[must_use]
    pub fn new(center: Point, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>> {
        let oc = r.origin - self.center;
        // Quadratic equation.
        let a = r.direction.len_squared();
        let half_b = oc.dot(r.direction);
        // oc.len_squared() - radius * radius
        let c = self.radius.mul_add(-self.radius, oc.len_squared());
        // half_b * half_b - a * c
        let discriminant = half_b.mul_add(half_b, -(a * c));

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;

            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let (face, normal) = HitRecord::face_normal(r, outward_normal);

        Some(HitRecord::new(p, normal, root, face, &*self.mat))
    }
}

#[non_exhaustive]
pub struct MovingSphere {
    center_start: Point,
    center_end: Point,
    t_start: f64,
    t_end: f64,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl MovingSphere {
    #[inline]
    #[must_use]
    pub fn new(
        center_start: Point,
        center_end: Point,
        t_start: f64,
        t_end: f64,
        radius: f64,
        mat: Arc<dyn Material>,
    ) -> Self {
        Self {
            center_start,
            center_end,
            t_start,
            t_end,
            radius,
            mat,
        }
    }

    #[inline]
    #[must_use]
    fn center(&self, t: f64) -> Point {
        self.center_start
            + ((t - self.t_start) / (self.t_end - self.t_start))
                * (self.center_end - self.center_start)
    }
}

impl Hit for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>> {
        let oc = r.origin - self.center(r.t);
        let a = r.direction.len_squared();
        let half_b = oc.dot(r.direction);
        let c = self.radius.mul_add(-self.radius, oc.len_squared());
        let discriminant = half_b.mul_add(half_b, -(a * c));

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;

            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center(r.t)) / self.radius;
        let (face, normal) = HitRecord::face_normal(r, outward_normal);

        Some(HitRecord::new(p, normal, root, face, &*self.mat))
    }
}
