use std::fmt;
use std::io;
use std::ops;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    e0: f64,
    e1: f64,
    e2: f64,
}

pub type Point3 = Vec3;
pub type Rgb = Vec3;

impl Vec3 {
    #[inline]
    #[must_use]
    pub const fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e0, e1, e2 }
    }

    #[inline]
    #[must_use]
    pub fn len_squared(&self) -> f64 {
        self.e0
            .mul_add(self.e0, self.e1.mul_add(self.e1, self.e2 * self.e2))
    }
    #[inline]
    #[must_use]
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    #[inline]
    #[must_use]
    pub fn dot(&self, rhs: Self) -> f64 {
        self.e0
            .mul_add(rhs.e0, self.e1.mul_add(rhs.e1, self.e2 * rhs.e2))
    }
    #[inline]
    #[must_use]
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            e0: self.e1.mul_add(rhs.e2, -(self.e2 * rhs.e1)),
            e1: self.e2.mul_add(rhs.e0, -(self.e0 * rhs.e2)),
            e2: self.e0.mul_add(rhs.e1, -(self.e1 * rhs.e0)),
        }
    }
    #[inline]
    #[must_use]
    pub fn unit(&self) -> Self {
        *self / self.len()
    }

    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    pub fn write<W>(self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        let ir = (255.999 * self.e0) as i32;
        let ig = (255.999 * self.e1) as i32;
        let ib = (255.999 * self.e2) as i32;

        writeln!(writer, "{} {} {}", ir, ig, ib)?;
        Ok(())
    }
}

impl Default for Vec3 {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self {
            e0: f64::default(),
            e1: f64::default(),
            e2: f64::default(),
        }
    }
}

impl fmt::Display for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e0, self.e1, self.e2)
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.e0,
            1 => &self.e1,
            2 => &self.e2,
            _ => panic!(
                "index out of bounds: the len is 3 but the index is {}",
                index
            ),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.e0,
            1 => &mut self.e1,
            2 => &mut self.e2,
            _ => panic!(
                "index out of bounds: the len is 3 but the index is {}",
                index
            ),
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    #[inline]
    #[must_use]
    fn neg(self) -> Self {
        Self {
            e0: -self.e0,
            e1: -self.e1,
            e2: -self.e2,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Self {
            e0: self.e0 + rhs.e0,
            e1: self.e1 + rhs.e1,
            e2: self.e2 + rhs.e2,
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
            e0: self.e0 - rhs.e0,
            e1: self.e1 - rhs.e1,
            e2: self.e2 - rhs.e2,
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
            e0: self.e0 * rhs.e0,
            e1: self.e1 * rhs.e1,
            e2: self.e2 * rhs.e2,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: f64) -> Self {
        Self {
            e0: self.e0 * rhs,
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    #[must_use]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e0: self * rhs.e0,
            e1: self * rhs.e1,
            e2: self * rhs.e2,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e0: self.e0 * rhs,
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
        }
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    #[inline]
    #[must_use]
    fn div(self, rhs: Self) -> Self {
        Self {
            e0: self.e0 / rhs.e0,
            e1: self.e1 / rhs.e1,
            e2: self.e2 / rhs.e2,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    #[inline]
    #[must_use]
    fn div(self, rhs: f64) -> Self {
        Self {
            e0: self.e0 / rhs,
            e1: self.e1 / rhs,
            e2: self.e2 / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            e0: self.e0 / rhs,
            e1: self.e1 / rhs,
            e2: self.e2 / rhs,
        }
    }
}
