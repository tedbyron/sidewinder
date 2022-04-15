use crate::graphics::{Face, HitRecord, Ray};
use crate::math::{Rgb, Vec3};
use crate::util::RngDist;

/// Trait for object materials to define how they scatter [`Ray`]s.
pub trait Material: Send + Sync {
    /// Calculate a scattered [`Ray`] and its resulting color attenuation from a ray-object
    /// intersection.
    fn scatter(&self, r: &Ray, rec: &HitRecord<'_>, rd: &mut RngDist<'_, '_>) -> Option<Scatter>;
}

/// Create a `HashMap` of `&str` and `Arc<dyn Material>` pairs.
#[macro_export]
macro_rules! matlist {
    () => {
        std::collections::HashMap::<&str, std::sync::Arc<dyn Material>>::default()
    };

    ( $($x:literal : $y:expr),* $(,)? ) => {{
        let mut tmp: std::collections::HashMap<&str, std::sync::Arc<dyn Material>> =
            std::collections::HashMap::default();
        $(tmp.insert($x, std::sync::Arc::new($y));)*
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

/// [`Material`] with Lambertian reflection.
#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Rgb,
}

impl Lambertian {
    #[inline]
    #[must_use]
    pub const fn new(albedo: Rgb) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    #[inline]
    fn scatter(&self, _: &Ray, rec: &HitRecord<'_>, rd: &mut RngDist<'_, '_>) -> Option<Scatter> {
        let mut direction = rec.normal + Vec3::random_unit_vec(rd);

        // Catch degenerate scatter direction.
        if direction.near_zero() {
            direction = rec.normal;
        }

        Some(Scatter::new(Ray::new(rec.p, direction), self.albedo))
    }
}

/// [`Material`] with metallic reflection.
#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct Metallic {
    /// The color of the material.
    pub albedo: Rgb,
    /// The roughness of the material.
    pub blur: f64,
}

impl Metallic {
    #[inline]
    #[must_use]
    pub fn new(albedo: Rgb, perturbation: f64) -> Self {
        Self {
            albedo,
            blur: perturbation.min(1.0),
        }
    }
}

impl Material for Metallic {
    #[inline]
    fn scatter(&self, r: &Ray, rec: &HitRecord<'_>, rd: &mut RngDist<'_, '_>) -> Option<Scatter> {
        let reflected = r.direction.unit().reflect(rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.blur * Vec3::random_in_unit_sphere(rd),
        );
        Some(Scatter::new(scattered, self.albedo))
    }
}

/// [`Material`] with dialectric refraction.
#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct Dialectric {
    /// The refractive index of the material.
    pub idx: f64,
}

impl Dialectric {
    #[inline]
    #[must_use]
    pub const fn new(idx: f64) -> Self {
        Self { idx }
    }
}

impl Material for Dialectric {
    #[inline]
    fn scatter(&self, r: &Ray, rec: &HitRecord<'_>, _: &mut RngDist<'_, '_>) -> Option<Scatter> {
        let ratio = match rec.face {
            Face::Front => self.idx.recip(),
            Face::Back => self.idx,
        };

        let unit_direction = r.direction.unit();
        let refracted = unit_direction.refract(rec.normal, ratio);
        let scattered = Ray::new(rec.p, refracted);

        Some(Scatter::new(scattered, Rgb::ONE))
    }
}
