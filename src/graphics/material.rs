use rand::distributions::Distribution;
use rand::rngs::ThreadRng;

use crate::graphics::{Face, HitRecord, Ray};
use crate::math::{Rgb, Vec3};
use crate::rng::UNIFORM_0_1;

/// Trait for object materials to define how they scatter [`Ray`]s.
pub trait Material: Send + Sync {
    /// Calculate a scattered [`Ray`] and its resulting color attenuation from a ray-object
    /// intersection.
    fn scatter(&self, r: &Ray, rec: &HitRecord<'_>, rng: &mut ThreadRng) -> Option<Scatter>;
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

/// A scattered [`Ray`] and its color.
#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct Scatter {
    /// The scattered ray.
    pub ray: Ray,
    /// The color of the scattered ray.
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
    /// The color of the material.
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
    fn scatter(&self, _: &Ray, rec: &HitRecord<'_>, rng: &mut ThreadRng) -> Option<Scatter> {
        let mut direction = rec.normal + Vec3::random_unit_vec(rng);

        // Catch degenerate scatter direction.
        if direction.near_zero() {
            direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, direction);
        Some(Scatter::new(scattered, self.albedo))
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
    pub fn new(albedo: Rgb, blur: f64) -> Self {
        Self {
            albedo,
            blur: blur.min(1.0),
        }
    }
}

impl Material for Metallic {
    #[inline]
    fn scatter(&self, r: &Ray, rec: &HitRecord<'_>, rng: &mut ThreadRng) -> Option<Scatter> {
        let reflected = r.direction.unit().reflect(rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.blur * Vec3::random_in_unit_sphere(rng),
        );

        if scattered.direction.dot(rec.normal) > 0.0 {
            Some(Scatter::new(scattered, self.albedo))
        } else {
            None
        }
    }
}

/// [`Material`] with dielectric refraction.
#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct Dielectric {
    /// The refractive index of the material.
    pub idx: f64,
}

impl Dielectric {
    #[inline]
    #[must_use]
    pub const fn new(idx: f64) -> Self {
        Self { idx }
    }

    #[inline]
    #[must_use]
    fn reflectance(cos: f64, idx: f64) -> f64 {
        // Schlick's approximation for reflectance.
        let r0 = ((1.0 - idx) / (1.0 + idx)).powi(2);
        // r0 + (1.0 - r0) * (1.0 - cos).powi(5)
        (1.0 - r0).mul_add((1.0 - cos).powi(5), r0)
    }
}

impl Material for Dielectric {
    #[inline]
    fn scatter(&self, r: &Ray, rec: &HitRecord<'_>, rng: &mut ThreadRng) -> Option<Scatter> {
        let ratio = match rec.face {
            Face::Front => self.idx.recip(),
            Face::Back => self.idx,
        };
        let unit_direction = r.direction.unit();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = cos_theta.mul_add(-cos_theta, 1.0).sqrt();
        let direction = if ratio * sin_theta > 1.0
            || Self::reflectance(cos_theta, ratio) > UNIFORM_0_1.sample(rng)
        {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, ratio)
        };
        let scattered = Ray::new(rec.p, direction);

        Some(Scatter::new(scattered, Rgb::ONE))
    }
}
