#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    rust_2018_idioms
)]
#![doc = include_str!("../README.md")]

pub mod camera;
pub mod graphics;
pub mod math;
pub mod object;

pub mod rng {
    use once_cell::sync::Lazy;
    use rand::distributions::Uniform;

    /// A uniform distribution over the range [-1.0, 1.0).
    pub static UNIFORM_N1_1: Lazy<Uniform<f64>> = Lazy::new(|| Uniform::from(-1.0..1.0));
    /// A uniform distribution over the range [0.0, 1.0).
    pub static UNIFORM_0_1: Lazy<Uniform<f64>> = Lazy::new(|| Uniform::from(0.0..1.0));
}
