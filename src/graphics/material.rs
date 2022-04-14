use std::collections::HashMap;

use crate::graphics::{HitRecord, Ray};
use crate::math::{Rgb, Vec3};
use crate::util::RngDist;

pub trait Material: Send + Sync {
    fn scatter(&self, r: &Ray, rec: &HitRecord, rd: &mut RngDist<'_, '_>) -> Option<Scatter>;
}

pub type MatList = HashMap<&'static str, Box<dyn Material>>;

#[macro_export]
macro_rules! matlist {
    () => {
        std::collections::HashMap::<&str, Box<dyn Material>>::default()
    };

    ( $($x:literal : $y:expr),* $(,)? ) => {{
        let mut tmp: std::collections::HashMap<&str, Box<dyn Material>> =
            std::collections::HashMap::default();
        $(tmp.insert($x, Box::from($y));)*
        tmp
    }};
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Rgb,
}

impl Scatter {
    #[inline]
    #[must_use]
    pub const fn new(ray: Ray, attenuation: Rgb) -> Self {
        Self { ray, attenuation }
    }
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Rgb,
}

impl From<Rgb> for Lambertian {
    #[inline]
    #[must_use]
    fn from(albedo: Rgb) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    #[inline]
    fn scatter(&self, _: &Ray, rec: &HitRecord, rd: &mut RngDist<'_, '_>) -> Option<Scatter> {
        let mut direction = rec.normal + Vec3::random_unit_vector(rd);
        if direction.near_zero() {
            direction = rec.normal;
        }

        Some(Scatter::new(Ray::new(rec.p, direction), self.albedo))
    }
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct Metallic {
    pub albedo: Rgb,
}

impl From<Rgb> for Metallic {
    #[inline]
    #[must_use]
    fn from(albedo: Rgb) -> Self {
        Self { albedo }
    }
}

impl Material for Metallic {
    #[inline]
    fn scatter(&self, r: &Ray, rec: &HitRecord, _: &mut RngDist<'_, '_>) -> Option<Scatter> {
        let reflected = r.direction.unit().reflect(rec.normal);
        Some(Scatter::new(Ray::new(rec.p, reflected), self.albedo))
    }
}
