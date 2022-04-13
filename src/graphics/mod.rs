mod hit;
mod material;
mod ray;

pub use hit::{Hit, HitList, HitRecord};
pub use material::{Lambertian, Material, Metallic};
pub use ray::Ray;
