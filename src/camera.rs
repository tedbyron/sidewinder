use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use crate::graphics::Ray;
use crate::math::{Point, Vec3};

#[non_exhaustive]
pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    dist: Uniform<f64>,
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub fn new(
        from: Point,
        to: Point,
        v_up: Vec3,
        v_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        t_start: f64,
        t_end: f64,
    ) -> Self {
        let theta = v_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (from - to).unit();
        let u = v_up.cross(w).unit();
        let v = w.cross(u);

        let origin = from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - focus_dist * w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0,
            dist: Uniform::from(t_start..t_end),
        }
    }

    #[inline]
    #[must_use]
    pub fn ray(&self, s: f64, t: f64, rng: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disc(rng);
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            // self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.origin
            //   - offset
            self.horizontal.mul_add(
                s,
                self.vertical
                    .mul_add(t, self.lower_left_corner - self.origin - offset),
            ),
            self.dist.sample(rng),
        )
    }
}
