#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms
)]
#![doc = include_str!("../README.md")]

pub mod ppm;
mod vec3;

pub use vec3::{Point3, Rgb};
