use std::{io, io::Write, ops};

use rand::prelude::*;

use crate::{
    math::Axis,
    rng::{CLOSED_OPEN_01, CLOSED_OPEN_N11},
};

/// A vector in 3D Euclidean space (**R**³).
#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Type alias for a [`Vec3`]; represents a point in 3D space.
pub type Point = Vec3;
/// Type alias for [`Vec3`]; represents an RGB color.
pub type Rgb = Vec3;

impl Vec3 {
    /// A vector in which all components are equal to 0.
    pub const ZERO: Self = Self::newi(0, 0, 0);
    /// A vector in which all components are equal to 1.
    pub const ONE: Self = Self::newi(1, 1, 1);

    pub const fn newf(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn newi(x: i32, y: i32, z: i32) -> Self {
        Self::newf(x as f64, y as f64, z as f64)
    }

    pub const fn new_all(n: f64) -> Self {
        Self::newf(n, n, n)
    }

    /// Fused multiply-add of each vector component.
    pub fn mul_add(self, a: f64, b: Self) -> Self {
        Self {
            x: self.x.mul_add(a, b.x),
            y: self.y.mul_add(a, b.y),
            z: self.z.mul_add(a, b.z),
        }
    }

    /// Vector length squared.
    pub fn len_squared(self) -> f64 {
        self.x
            .mul_add(self.x, self.y.mul_add(self.y, self.z * self.z))
    }
    /// Vector length.
    pub fn len(self) -> f64 {
        self.len_squared().sqrt()
    }

    /// Dot product.
    pub fn dot(self, rhs: Self) -> f64 {
        self.x.mul_add(rhs.x, self.y.mul_add(rhs.y, self.z * rhs.z))
    }
    /// Cross product.
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y.mul_add(rhs.z, -(self.z * rhs.y)),
            y: self.z.mul_add(rhs.x, -(self.x * rhs.z)),
            z: self.x.mul_add(rhs.y, -(self.y * rhs.x)),
        }
    }
    /// The vector's unit vector.
    pub fn unit(self) -> Self {
        self / self.len()
    }

    /// Whether the vector is near the origin, checking whether each of the components is less than
    /// an offset `DELTA` from zero.
    pub fn near_zero(self) -> bool {
        const DELTA: f64 = 1.0e-8;
        self.x < DELTA && self.y < DELTA && self.z < DELTA
    }

    /// A new vector representing the reflection of `self` at `normal`.
    pub fn reflect(self, normal: Self) -> Self {
        // self - 2.0 * self.dot(normal) * normal
        (self.dot(normal) * normal).mul_add(-2.0, self)
    }

    /// A new vector representing the refraction of `self` at `normal`, with the refractive index
    /// `idx`
    pub fn refract(self, normal: Self, idx: f64) -> Self {
        let cos_theta = (-self).dot(normal).min(1.0);
        // idx * (self + cos_theta * normal)
        let perpendicular = idx * normal.mul_add(cos_theta, self);
        let parallel = -(1.0 - perpendicular.len_squared()).abs().sqrt() * normal;

        perpendicular + parallel
    }

    /// RGB8 values scaled and gamma-corrected.
    pub fn to_rgb8(self, samples: u32) -> [u8; 3] {
        let scale = f64::from(samples).recip();

        // Gamma correction.
        let scaled_r = (self.x * scale).sqrt();
        let scaled_g = (self.y * scale).sqrt();
        let scaled_b = (self.z * scale).sqrt();

        let r = (256.0 * scaled_r.clamp(0.0, 0.999)) as u8;
        let g = (256.0 * scaled_g.clamp(0.0, 0.999)) as u8;
        let b = (256.0 * scaled_b.clamp(0.0, 0.999)) as u8;

        [r, g, b]
    }

    /// Write an [`Rgb`] color into a buffer.
    ///
    /// # Errors
    ///
    /// If there is an error writing to the buffer.
    pub fn write(self, buf: &mut dyn Write, samples: u32) -> io::Result<()> {
        let scale = f64::from(samples).recip();

        // Gamma correction.
        let scaled_r = (self.x * scale).sqrt();
        let scaled_g = (self.y * scale).sqrt();
        let scaled_b = (self.z * scale).sqrt();

        let r = (256.0 * scaled_r.clamp(0.0, 0.999)) as u8;
        let g = (256.0 * scaled_g.clamp(0.0, 0.999)) as u8;
        let b = (256.0 * scaled_b.clamp(0.0, 0.999)) as u8;

        writeln!(buf, "{r} {g} {b}")
    }

    /// A random vector with components sampled from the uniform range [0, 1).
    pub fn random(rng: &mut ThreadRng) -> Self {
        Self {
            x: CLOSED_OPEN_01.sample(rng),
            y: CLOSED_OPEN_01.sample(rng),
            z: CLOSED_OPEN_01.sample(rng),
        }
    }

    /// A random vector with components sampled from the given distribution.
    pub fn random_in(dist: &impl Distribution<f64>, rng: &mut ThreadRng) -> Self {
        Self {
            x: dist.sample(rng),
            y: dist.sample(rng),
            z: dist.sample(rng),
        }
    }

    /// A random vector within a unit sphere.
    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::random_in(&*CLOSED_OPEN_N11, rng);
            if p.len_squared() < 1.0 {
                break p;
            }
        }
    }

    /// A random vector within a unit disc.
    pub fn random_in_unit_disc(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::newf(
                CLOSED_OPEN_N11.sample(rng),
                CLOSED_OPEN_N11.sample(rng),
                0.0,
            );
            if p.len_squared() < 1.0 {
                break p;
            }
        }
    }

    /// The unit vector of a random vector within a unit sphere.
    pub fn random_unit_vec(rng: &mut ThreadRng) -> Self {
        Self::random_in_unit_sphere(rng).unit()
    }

    /// A random vector within the same hemisphere as the given `normal`.
    pub fn random_in_hemisphere(normal: Self, rng: &mut ThreadRng) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere(rng);

        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl ops::Index<Axis> for Vec3 {
    type Output = f64;

    fn index(&self, axis: Axis) -> &Self::Output {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}
