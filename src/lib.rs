#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms
)]
#![doc = include_str!("../README.md")]

mod camera;
mod hit;
mod ray;
mod sphere;
mod vec3;

pub use camera::Camera;
pub use hit::{HitList, HitRecord};
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::{Point, Rgb, Vec3};
