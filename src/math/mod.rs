//! Primitive types.

mod vec3;

pub use vec3::{Point, Rgb, Vec3};

#[derive(Clone, Copy, strum::EnumIter)]
pub enum Axis {
    X,
    Y,
    Z,
}
