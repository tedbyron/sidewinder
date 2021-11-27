use std::fmt;
use std::io;
use std::ops;

#[non_exhaustive]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    #[inline]
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding error, yielding a more
    /// accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` may be more performant than an unfused multiply-add if the target
    /// architecture has a dedicated `fma` CPU instruction. However, this is not always true, and
    /// will be heavily dependant on designing algorithms with specific target hardware in mind.
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

    /// Write an RGB color represented by a [`Vec3`] into a buffer, using the input writer.
    ///
    /// # Errors
    ///
    /// If there is an error writing to stdout.
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    pub fn write_rgb<W: io::Write>(self, writer: &mut W) -> io::Result<()> {
        let ir = (255.999 * self.x) as i32;
        let ig = (255.999 * self.y) as i32;
        let ib = (255.999 * self.z) as i32;

        writeln!(writer, "{} {} {}", ir, ig, ib)?;
        Ok(())
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
