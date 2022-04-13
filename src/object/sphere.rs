use crate::graphics::{Hit, HitRecord, Material, Ray};
use crate::math::Point;

#[non_exhaustive]
pub struct Sphere<'a> {
    center: Point,
    radius: f64,
    mat: &'a Box<dyn Material>,
}

impl<'a> Sphere<'a> {
    #[inline]
    #[must_use]
    pub fn new(center: Point, radius: f64, mat: &'a Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl<'a> Hit for Sphere<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
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
        let mut root = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;

            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let (face, normal) = HitRecord::face_normal(r, outward_normal);

        Some(HitRecord::new(p, normal, root, face, self.mat))
    }
}
