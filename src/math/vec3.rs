use std::io;
use std::ops;

use rand::distributions::Distribution;

use crate::util::RngDist;

/// A vector in 3D Euclidean space (**R**³).
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
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
    /// A vector in which all components are equal to 0.0.
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    /// A vector in which all components are equal to 1.0.
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0);

    #[inline]
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Fused multiply-add of each vector component.
    #[inline]
    #[must_use]
    pub fn mul_add(self, a: f64, b: Self) -> Self {
        Self {
            x: self.x.mul_add(a, b.x),
            y: self.y.mul_add(a, b.y),
            z: self.z.mul_add(a, b.z),
        }
    }

    /// Vector length squared.
    #[inline]
    #[must_use]
    pub fn len_squared(self) -> f64 {
        self.x
            .mul_add(self.x, self.y.mul_add(self.y, self.z * self.z))
    }
    /// Vector length.
    #[inline]
    #[must_use]
    pub fn len(self) -> f64 {
        self.len_squared().sqrt()
    }

    /// Dot product.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> f64 {
        self.x.mul_add(rhs.x, self.y.mul_add(rhs.y, self.z * rhs.z))
    }
    /// Cross product.
    #[inline]
    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y.mul_add(rhs.z, -(self.z * rhs.y)),
            y: self.z.mul_add(rhs.x, -(self.x * rhs.z)),
            z: self.x.mul_add(rhs.y, -(self.y * rhs.x)),
        }
    }
    /// The vector's unit vector.
    #[inline]
    #[must_use]
    pub fn unit(self) -> Self {
        self / self.len()
    }

    /// Whether the vector is near the origin, checking whether each of the components is less than
    /// an offset `DELTA` from zero.
    #[inline]
    #[must_use]
    pub fn near_zero(self) -> bool {
        const DELTA: f64 = 1.0e-8;
        self.x < DELTA && self.y < DELTA && self.z < DELTA
    }

    /// A new vector representing the reflection of `self` at `normal`.
    #[inline]
    #[must_use]
    pub fn reflect(self, normal: Self) -> Self {
        // self - 2.0 * self.dot(n) * n
        (self.dot(normal) * normal).mul_add(-2.0, self)
    }

    /// Write an [`Rgb`] color into a buffer, using the input writer.
    ///
    /// # Errors
    ///
    /// If there is an error writing the data.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[inline]
    pub fn write<W: io::Write>(self, writer: &mut W, samples: u32) -> io::Result<()> {
        let scale = f64::from(samples).recip();

        // Gamma correction.
        let scaled_r = (self.x * scale).sqrt();
        let scaled_g = (self.y * scale).sqrt();
        let scaled_b = (self.z * scale).sqrt();

        let r = (256.0 * scaled_r.clamp(0.0, 0.999)) as u8;
        let g = (256.0 * scaled_g.clamp(0.0, 0.999)) as u8;
        let b = (256.0 * scaled_b.clamp(0.0, 0.999)) as u8;

        writeln!(writer, "{} {} {}", r, g, b)
    }

    /// A random vector with compenents sampled from the given [`RngDist`].
    #[inline]
    #[must_use]
    fn random(rd: &mut RngDist<'_, '_>) -> Self {
        Self {
            x: rd.dist.sample(rd.rng),
            y: rd.dist.sample(rd.rng),
            z: rd.dist.sample(rd.rng),
        }
    }

    /// A random vector within a unit sphere, with components sampled from the given [`RngDist`].
    #[must_use]
    pub fn random_in_unit_sphere(rd: &mut RngDist<'_, '_>) -> Self {
        loop {
            let p = Self::random(rd);
            if p.len_squared() < 1.0 {
                break p;
            }
        }
    }

    /// The unit vector of a random vector within a unit sphere, with components sampled from the
    /// given [`RngDist`].
    #[inline]
    #[must_use]
    pub fn random_unit_vec(rd: &mut RngDist<'_, '_>) -> Self {
        Self::random_in_unit_sphere(rd).unit()
    }

    /// A random vector within the same hemisphere as the given `normal`, with components sampled
    /// from the given [`RngDist`].
    #[inline]
    #[must_use]
    pub fn random_in_hemisphere(normal: Self, rd: &mut RngDist<'_, '_>) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere(rd);

        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    #[inline]
    #[must_use]
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

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    #[inline]
    #[must_use]
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

    #[inline]
    #[must_use]
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

    #[inline]
    #[must_use]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    #[inline]
    #[must_use]
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

    #[inline]
    #[must_use]
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}
