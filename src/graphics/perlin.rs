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
        let rng = &mut rand::thread_rng();
        let rand_f = {
            let mut rand_f = [0.0; Self::POINT_COUNT];
            for f in rand_f.iter_mut() {
                *f = rng.gen();
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
        let i = (4.0 * p.x) as usize & 255;
        let j = (4.0 * p.y) as usize & 255;
        let k = (4.0 * p.z) as usize & 255;

        self.rand_f[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn gen_perm(rng: &mut ThreadRng) -> [usize; Self::POINT_COUNT] {
        let mut p = [0; Self::POINT_COUNT];

        for (i, n) in p.iter_mut().enumerate() {
            *n = i;
        }

        Self::permute(&mut p, rng);

        p
    }

    fn permute(p: &mut [usize], rng: &mut ThreadRng) {
        for i in (1..p.len()).rev() {
            let target = rng.gen_range(0..i);
            p.swap(i, target);
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
