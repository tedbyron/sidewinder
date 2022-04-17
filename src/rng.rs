use once_cell::sync::Lazy;
use rand::distributions::{Distribution, Standard, Uniform};

use crate::math::Axis;

/// A uniform distribution of type `f64` over the range [-1.0, 1.0).
pub static CLOSED_OPEN_N11: Lazy<Uniform<f64>> = Lazy::new(|| Uniform::new(-1.0, 1.0));
/// A uniform distribution of type `f64` over the range [0.0, 1.0).
pub static CLOSED_OPEN_01: Lazy<Uniform<f64>> = Lazy::new(|| Uniform::new(0.0, 1.0));
static OPEN_02: Lazy<Uniform<u8>> = Lazy::new(|| Uniform::new_inclusive(0, 2));

impl Distribution<Axis> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        match OPEN_02.sample(rng) {
            0 => Axis::X,
            1 => Axis::Y,
            _ => Axis::Z,
        }
    }
}
