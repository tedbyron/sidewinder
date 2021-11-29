use std::fmt;
use std::io;
use std::ops;

use rand::distributions::Distribution;
use rand::prelude::ThreadRng;

#[non_exhaustive]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point = Vec3;
pub type Rgb = Vec3;

impl Vec3 {
    #[inline]
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    #[must_use]
    pub fn mul_add(self, a: f64, b: Self) -> Self {
        Self {
            x: self.x.mul_add(a, b.x),
            y: self.y.mul_add(a, b.y),
            z: self.z.mul_add(a, b.z),
        }
    }

    #[inline]
    #[must_use]
    pub fn len_squared(self) -> f64 {
        self.x
            .mul_add(self.x, self.y.mul_add(self.y, self.z * self.z))
    }
    #[inline]
    #[must_use]
    pub fn len(self) -> f64 {
        self.len_squared().sqrt()
    }

    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> f64 {
        self.x.mul_add(rhs.x, self.y.mul_add(rhs.y, self.z * rhs.z))
    }
    #[inline]
    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y.mul_add(rhs.z, -(self.z * rhs.y)),
            y: self.z.mul_add(rhs.x, -(self.x * rhs.z)),
            z: self.x.mul_add(rhs.y, -(self.y * rhs.x)),
        }
    }
    #[inline]
    #[must_use]
    pub fn unit(self) -> Self {
        self / self.len()
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

        let scaled_r = (self.x * scale).sqrt();
        let scaled_g = (self.y * scale).sqrt();
        let scaled_b = (self.z * scale).sqrt();

        let r = (256.0 * scaled_r.clamp(0.0, 0.999)) as u8;
        let g = (256.0 * scaled_g.clamp(0.0, 0.999)) as u8;
        let b = (256.0 * scaled_b.clamp(0.0, 0.999)) as u8;

        writeln!(writer, "{} {} {}", r, g, b)?;
        Ok(())
    }

    #[inline]
    #[must_use]
    fn random<D: Distribution<f64>>(rng: &mut ThreadRng, dist: &D) -> Self {
        Self {
            x: dist.sample(rng),
            y: dist.sample(rng),
            z: dist.sample(rng),
        }
    }

    #[inline]
    #[must_use]
    pub fn random_in_unit_sphere<D>(rng: &mut ThreadRng, dist: &D) -> Self
    where
        D: Distribution<f64>,
    {
        loop {
            let p = Self::random(rng, dist);
            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }
}

impl fmt::Display for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_add() {
        let base = Vec3::new(1.0, 2.0, 3.0);
        let add = Vec3::new(3.0, 2.0, 1.0);

        assert_eq!(base.mul_add(2.0, add), base * 2.0 + add);
    }
}
