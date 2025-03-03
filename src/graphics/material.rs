use std::sync::Arc;

use rand::prelude::*;

use crate::{
    graphics::{Face, HitRecord, Ray, Texture},
    math::{Rgb, Vec3},
    rng::CLOSED_OPEN_01,
};

/// Trait for object materials to define how they scatter [`Ray`]s.
pub trait Material: Send + Sync {
    /// Calculate a scattered [`Ray`] and its resulting color attenuation from a ray-object
    /// intersection.
    fn scatter(&self, r: &Ray, rec: &HitRecord<'_>, rng: &mut ThreadRng) -> Option<Scatter>;
}

/// Creates a `HashMap` with `String` keys and `Arc<dyn Material>` values.
#[macro_export]
macro_rules! matlist {
    () => {
        use std::collections::HashMap;
        use std::sync::Arc;

        use sidewinder::graphics::Material;

        HashMap::<String, Arc<dyn Material>>::default()
    };

    ( $($x:literal : $y:expr),* $(,)? ) => {{
        use std::collections::HashMap;
        use std::sync::Arc;

        use sidewinder::graphics::Material;

        let mut tmp: HashMap<String, Arc<dyn Material>> = HashMap::default();
        $(tmp.insert($x.to_string(), Arc::new($y));)*
        tmp
    }};
}

/// A scattered [`Ray`] and its color.
#[non_exhaustive]
pub struct Scatter {
    /// The scattered ray.
    pub ray: Ray,
    /// The color of the scattered ray.
    pub attenuation: Rgb,
}

impl Scatter {
    pub const fn new(ray: Ray, attenuation: Rgb) -> Self {
        Self { ray, attenuation }
    }
}

/// [`Material`] with Lambertian reflection.
#[non_exhaustive]
pub struct Lambertian {
    /// The texture of the material.
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub const fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord<'_>, rng: &mut ThreadRng) -> Option<Scatter> {
        let mut direction = rec.normal + Vec3::random_unit_vec(rng);

        // Catch degenerate scatter direction.
        if direction.near_zero() {
            direction = rec.normal;
        }

        let scattered = Ray::new(rec.point, direction, r.t);
        Some(Scatter::new(
            scattered,
            self.albedo.value(rec.u, rec.v, &rec.point),
        ))
    }
}

/// [`Material`] with metallic reflection.
#[non_exhaustive]
pub struct Metallic {
    /// The color of the material.
    pub albedo: Rgb,
    /// The roughness of the material.
    pub blur: f64,
}

impl Metallic {
    pub const fn new(albedo: Rgb, blur: f64) -> Self {
        Self {
            albedo,
            blur: blur.min(1.0),
        }
    }
}

impl Material for Metallic {
    fn scatter(&self, r: &Ray, rec: &HitRecord<'_>, rng: &mut ThreadRng) -> Option<Scatter> {
        let reflected = r.direction.unit().reflect(rec.normal);
        let scattered = Ray::new(
            rec.point,
            reflected + self.blur * Vec3::random_in_unit_sphere(rng),
            r.t,
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
pub struct Dielectric {
    /// The refractive index of the material.
    pub idx: f64,
}

impl Dielectric {
    pub const fn new(idx: f64) -> Self {
        Self { idx }
    }

    fn reflectance(cos: f64, idx: f64) -> f64 {
        // Schlick's approximation for reflectance.
        let r0 = ((1.0 - idx) / (1.0 + idx)).powi(2);
        // r0 + (1.0 - r0) * (1.0 - cos).powi(5)
        (1.0 - r0).mul_add((1.0 - cos).powi(5), r0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord<'_>, rng: &mut ThreadRng) -> Option<Scatter> {
        let ratio = match rec.face {
            Face::Front => self.idx.recip(),
            Face::Back => self.idx,
        };
        let unit_direction = r.direction.unit();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = cos_theta.mul_add(-cos_theta, 1.0).sqrt();
        let direction = if ratio * sin_theta > 1.0
            || Self::reflectance(cos_theta, ratio) > CLOSED_OPEN_01.sample(rng)
        {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, ratio)
        };
        let scattered = Ray::new(rec.point, direction, r.t);

        Some(Scatter::new(scattered, Rgb::ONE))
    }
}
