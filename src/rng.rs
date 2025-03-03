use std::sync::LazyLock;

use rand::distr::{Distribution, StandardUniform, Uniform};

use crate::math::Axis;

/// A uniform distribution of type `f64` over the range [-1.0, 1.0).
pub static CLOSED_OPEN_N11: LazyLock<Uniform<f64>> =
    LazyLock::new(|| Uniform::new(-1.0, 1.0).unwrap());
/// A uniform distribution of type `f64` over the range [0.0, 1.0).
pub static CLOSED_OPEN_01: LazyLock<Uniform<f64>> =
    LazyLock::new(|| Uniform::new(0.0, 1.0).unwrap());
static OPEN_02: LazyLock<Uniform<u8>> = LazyLock::new(|| Uniform::new_inclusive(0, 2).unwrap());

impl Distribution<Axis> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        match OPEN_02.sample(rng) {
            0 => Axis::X,
            1 => Axis::Y,
            _ => Axis::Z,
        }
    }
}
