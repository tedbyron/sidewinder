use crate::graphics::{HitRecord, Ray};
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
    pub albedo: Rgb,
    pub perturbation: f64,
}

impl Metallic {
    #[inline]
    #[must_use]
    pub fn new(albedo: Rgb, perturbation: f64) -> Self {
        Self {
            albedo,
            perturbation: perturbation.min(1.0),
        }
    }
}

impl Material for Metallic {
    #[inline]
    fn scatter(&self, r: &Ray, rec: &HitRecord<'_>, rd: &mut RngDist<'_, '_>) -> Option<Scatter> {
        let reflected = r.direction.unit().reflect(rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.perturbation * Vec3::random_in_unit_sphere(rd),
        );
        Some(Scatter::new(scattered, self.albedo))
    }
}
