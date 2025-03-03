use rand::prelude::*;

use crate::math::Point;

/// Perlin noise generator.
pub struct Perlin {
    rand_f: [f64; Self::POINT_COUNT],
    perm_x: [usize; Self::POINT_COUNT],
    perm_y: [usize; Self::POINT_COUNT],
    perm_z: [usize; Self::POINT_COUNT],
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let rng = &mut rand::rng();
        let rand_f = {
            let mut rand_f = [0.0; Self::POINT_COUNT];
            for f in &mut rand_f {
                *f = rng.random();
            }
            rand_f
        };
        let perm_x = Self::gen_perm(rng);
        let perm_y = Self::gen_perm(rng);
        let perm_z = Self::gen_perm(rng);

        Self {
            rand_f,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;

        let mut c = [[[0.0; 2]; 2]; 2];

        #[allow(clippy::needless_range_loop)]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_f[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]];
                }
            }
        }

        Self::interpolate(c, u, v, w)
    }

    fn gen_perm(rng: &mut ThreadRng) -> [usize; Self::POINT_COUNT] {
        let mut p = [0; Self::POINT_COUNT];

        for (i, n) in p.iter_mut().enumerate() {
            *n = i;
        }

        for i in (1..p.len()).rev() {
            let target = rng.random_range(0..i);
            p.swap(i, target);
        }

        p
    }

    fn interpolate(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut acc = 0.0;

        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    acc += (i as f64).mul_add(u, (1.0 - i as f64) * (1.0 - u))
                        * (j as f64).mul_add(v, (1.0 - j as f64) * (1.0 - v))
                        * (k as f64).mul_add(w, (1.0 - k as f64) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }

        acc
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
