//! Graphics traits and types.

mod hit;
mod material;
mod ray;

pub use hit::{Face, Hit, HitList, HitRecord};
pub use material::{Dialectric, Lambertian, Material, Metallic};
pub use ray::Ray;
